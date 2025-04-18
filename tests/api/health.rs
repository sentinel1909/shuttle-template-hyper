// tests/api/health.rs

// dependencies
use crate::helpers::{get_test_client, start_test_server};

#[tokio::test]
async fn health_route_returns_200_ok() {
    // Arrange
    let addr = start_test_server().await;
    let client = get_test_client();

    // Act
    let response = client
        .get(format!("http://{}/_health", addr))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), 200);
}
