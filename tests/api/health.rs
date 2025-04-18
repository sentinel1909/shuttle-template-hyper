// tests/api/health.rs

// dependencies
use crate::helpers::start_test_server;
use reqwest::Client;

#[tokio::test]
async fn health_route_returns_200_ok() {
    // Arrange
    let addr = start_test_server().await;
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    // Act
    let response = client
        .get(format!("http://{}/_health", addr))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), 200);
}
