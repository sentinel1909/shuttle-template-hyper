// tests/api/ping.rs

// dependencies
use crate::helpers::start_test_server;
use reqwest::Client;

#[tokio::test]
async fn ping_route_works() {
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
}