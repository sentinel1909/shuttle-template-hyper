use crate::helpers::{get_test_client, start_test_server, start_test_server_with_state};
use serde::Deserialize;
use shuttle_hyper_template_lib::actors::{AnalyticsMessage, PingMessage};
use shuttle_hyper_template_lib::init::build_route_table;
use shuttle_hyper_template_lib::state::AppState;
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Deserialize)]
struct TestResponse<T> {
    msg: String,
    content: T,
}

#[derive(Deserialize)]
struct TestError {
    msg: String,
    error: String,
}

#[tokio::test]
async fn metrics_returns_200_and_reflects_ping_activity() {
    // Arrange
    let addr = start_test_server().await;
    let client = get_test_client();

    // Act 1: Get initial metrics response
    let resp = client
        .get(format!("http://{}/metrics", addr))
        .send()
        .await
        .expect("Failed to call /metrics");
    assert_eq!(resp.status(), 200);

    let initial: TestResponse<HashMap<String, usize>> = resp
        .json()
        .await
        .expect("Failed to parse JSON from /metrics");
    assert_eq!(initial.msg, "success");
    assert!(initial.content.is_empty());

    // Act 2: Hit /ping three times
    for _ in 0..3 {
        let ping_resp = client
            .get(format!("http://{}/ping", addr))
            .send()
            .await
            .expect("Failed to call /ping");
        assert_eq!(ping_resp.status(), 200);
    }

    // Act 3: Get updated metrics
    let resp = client
        .get(format!("http://{}/metrics", addr))
        .send()
        .await
        .expect("Failed to call /metrics");

    // Assert
    assert_eq!(resp.status(), 200);

    let metrics: TestResponse<HashMap<String, usize>> = resp
        .json()
        .await
        .expect("Failed to parse JSON from /metrics");

    assert_eq!(metrics.msg, "success");
    assert_eq!(metrics.content.get("ping"), Some(&3));
}

#[tokio::test]
async fn metrics_route_returns_500_when_actor_does_not_respond() {
    // Arrange: Analytics actor receives GetAll but never replies
    let (analytics_tx, mut analytics_rx) = mpsc::channel::<AnalyticsMessage>(1);
    let (ping_tx, _ping_rx) = mpsc::channel::<PingMessage>(1);

    tokio::spawn(async move {
        while let Some(msg) = analytics_rx.recv().await {
            if let AnalyticsMessage::GetAll { reply } = msg {
                // Intentionally drop reply without sending
                drop(reply);
            }
        }
    });

    let state = AppState {
        routes: Arc::new(build_route_table()),
        ping_tx,
        analytics_tx,
    };

    let addr = start_test_server_with_state(state).await;
    let client = get_test_client();

    // Act
    let response = client
        .get(format!("http://{}/metrics", addr))
        .send()
        .await
        .expect("Request to /metrics failed");

    // Assert
    assert_eq!(response.status(), 500);

    let body: TestError = response
        .json()
        .await
        .expect("Failed to deserialize error response");

    assert_eq!(body.msg, "error");
    assert!(
        body.error.contains("Actor"),
        "Unexpected error message: {}",
        body.error
    );
}
