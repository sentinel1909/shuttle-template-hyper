// tests/api/count.rs

// dependencies
use crate::helpers::{get_test_client, start_test_server, start_test_server_with_state};
use serde::Deserialize;
use shuttle_hyper_template_lib::actors::PingMessage;
use shuttle_hyper_template_lib::init::build_route_table;
use shuttle_hyper_template_lib::state::AppState;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Deserialize)]
struct CountResponse {
    count: usize,
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

    assert_eq!(response.status(), 200);

    let count_response: CountResponse = response
        .json()
        .await
        .expect("Failed to parse JSON from /count");

    // Assert: count is 3
    assert_eq!(count_response.count, 3);
}

#[tokio::test]
async fn count_route_returns_500_when_actor_does_not_respond() {
    // Arrange: actor receives GetCount but never replies
    let (tx, mut rx) = mpsc::channel::<PingMessage>(1);

    // Spawn an actor that ignores GetCount messages
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
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
        ping_tx: tx,
    };

    let addr = start_test_server_with_state(state).await;
    let client = get_test_client();

    // Act
    let response = client
        .get(format!("http://{}/count", addr))
        .send()
        .await
        .expect("Request to /count failed");

    // Assert
    let status = response.status();
    let bytes = response.bytes().await.expect("Failed to read body");

    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("Expected valid JSON");

    assert_eq!(status, 500);
    let error_text = json["error"]
        .as_str()
        .expect("error field should be a string");
    assert!(
        error_text.contains("Actor channel failed"),
        "Unexpected error message: {error_text}"
    );
}
