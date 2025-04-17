// tests/api/count.rs

// dependencies
use crate::helpers::start_test_server;
use reqwest::Client;

#[tokio::test]
async fn count_route_works() {
    // Arrange
    let addr = start_test_server().await;
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

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

    let body_bytes = response.bytes().await.unwrap();
    let body_str = std::str::from_utf8(&body_bytes).expect("Body not valid UTF-8");

    // Assert: count is 3
    assert_eq!(body_str.trim(), "3");
}
