// tests/api/echo.rs

// dependencies
use crate::helpers::{get_test_client, start_test_server};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EchoResponse {
    foo: String,
    bar: String,
}

#[tokio::test]
async fn echo_query_param() {
    // Arrange
    let addr = start_test_server().await;
    let client = get_test_client();

    // Act
    let resp = client
        .get(format!("http://{}/echo?foo=hello&bar=world", addr))
        .send()
        .await
        .expect("Request to /echo failed");

    // Assert
    assert_eq!(resp.status(), 200);

    let body: EchoResponse = resp.json().await.expect("Failed to parse JSON response");

    assert_eq!(body.foo, "hello");
    assert_eq!(body.bar, "world");
}
