// tests/api/ping.rs

// dependencies
use crate::helpers::{get_test_client, start_test_server, start_test_server_with_state};
use serde::Deserialize;
use shuttle_hyper_template_lib::actors::PingMessage;
use shuttle_hyper_template_lib::init::build_route_table;
use shuttle_hyper_template_lib::state::AppState;
use std::sync::Arc;
use tokio::sync::mpsc;

// struct type to represent a response body from the /ping endpoint
#[derive(Deserialize)]
struct PingResponse {
    msg: String,
}

#[tokio::test]
async fn ping_route_returns_200_ok() {
    // Arrange
    let addr = start_test_server().await;
    let client = get_test_client();

    // Act
    let response = client
        .get(format!("http://{}/ping", addr))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), 200);

    let ping_response: PingResponse = response
        .json()
        .await
        .expect("Failed to parse JSON from /count");

    // Assert: count is 3
    assert_eq!(ping_response.msg, "Pong");
}

#[tokio::test]
async fn ping_route_returns_502_when_actor_dropped() {
    // Arrange
    let (tx, rx) = mpsc::channel::<PingMessage>(1);
    drop(rx);

    let state = AppState {
        routes: Arc::new(build_route_table()),
        ping_tx: tx,
    };

    let addr = start_test_server_with_state(state).await;
    let client = get_test_client();

    // Act
    let response = client
        .get(format!("http://{}/ping", addr))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), 502);
}
