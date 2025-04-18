// tests/api/count.rs

// dependencies
use crate::helpers::{get_test_client, start_test_server};
use serde::Deserialize;

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
