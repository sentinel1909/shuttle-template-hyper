// tests/api/ping.rs

// dependencies
use crate::helpers::{get_test_client, start_test_server, start_test_server_with_sender};
use serde::Deserialize;
use shuttle_hyper_template_lib::actors::PingMessage;
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

// function to create a sender whose receiver has been dropped, simulating a dead actor
fn make_dropped_actor_sender() -> mpsc::Sender<PingMessage> {
    let (tx, rx) = mpsc::channel::<PingMessage>(1);
    drop(rx);
    tx
}

#[tokio::test]
async fn ping_route_returns_502_when_actor_dropped() {
    
    // Arrange
    let tx = make_dropped_actor_sender();

    let addr = start_test_server_with_sender(tx).await;
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
