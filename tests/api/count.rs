// tests/api/count.rs

// dependencies
use crate::helpers::{get_test_client, start_test_server, start_test_server_with_state};
use serde::Deserialize;
use shuttle_hyper_template_lib::AnalyticsMessage;
use shuttle_hyper_template_lib::actors::PingMessage;
use shuttle_hyper_template_lib::init::build_route_table;
use shuttle_hyper_template_lib::state::AppState;
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
async fn count_route_returns_200_ok_and_correct_ping_count() {
    // Arrange
    let addr = start_test_server().await;
    let client = get_test_client();

    // Act: Send 3 ping requests
    for _ in 0..3 {
        let response = client
            .get(format!("http://{}/ping", addr))
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(response.status(), 200);
    }

    // Act: Call /count
    let response = client
        .get(format!("http://{}/count", addr))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), 200);

    let body: TestResponse<usize> = response.json().await.expect("Failed to parse JSON");

    assert_eq!(body.msg, "success");
    assert_eq!(body.content, 3);
}

#[tokio::test]
async fn count_route_returns_500_when_actor_does_not_respond() {
    // Arrange: actor receives GetCount but never replies
    let (ping_tx, mut ping_rx) = mpsc::channel::<PingMessage>(1);
    let (analytics_tx, _analytics_rx) = mpsc::channel::<AnalyticsMessage>(1);

    // Spawn an actor that ignores GetCount messages
    tokio::spawn(async move {
        while let Some(msg) = ping_rx.recv().await {
            match msg {
                PingMessage::GetCount(_reply_tx) => {
                    // Intentionally drop the reply sender — simulates silent failure
                }
                PingMessage::Ping => {
                    // Accept ping so /ping tests don’t break if reused
                }
            }
        }
    });

    let state = AppState {
        routes: Arc::new(build_route_table()),
        analytics_tx,
        ping_tx,
    };

    let addr = start_test_server_with_state(state).await;
    let client = get_test_client();

    // Act
    let response = client
        .get(format!("http://{}/count", addr))
        .send()
        .await
        .expect("Request to /count failed");

    let status = response.status();

    let body: TestError = response
        .json()
        .await
        .expect("Failed to parse JSON from /count");

    // Assert
    assert_eq!(status, 500);
    assert_eq!(body.msg, "error");
    assert!(
        body.error.contains("Actor channel failed"),
        "Unexpected error message: {}",
        body.error
    );
}
