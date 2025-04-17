// tests/api/ping.rs

// dependencies
use crate::helpers::start_test_server;
use reqwest::Client;
use serde::Deserialize;

// struct type to represent a response body from the /ping endpoint
#[derive(Deserialize    )]
struct PingResponse {
    msg: String,
}

#[tokio::test]
async fn ping_route_returns_200_ok() {
    // Arrange
    let addr = start_test_server().await;
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

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
