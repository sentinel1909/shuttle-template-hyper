// tests/api/echo.rs

// dependencies
use crate::helpers::{get_test_client, start_test_server};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct TestResponse<T> {
    msg: String,
    content: T,
}

#[tokio::test]
async fn echo_query_param_returns_200_and_params() {
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

    let body: TestResponse<HashMap<String, String>> =
        resp.json().await.expect("Failed to parse JSON response");

    assert_eq!(body.msg, "success");
    let expected = {
        let mut map = std::collections::HashMap::new();
        map.insert("foo".to_string(), "hello".to_string());
        map.insert("bar".to_string(), "world".to_string());
        map
    };

    assert_eq!(body.content, expected);
}

#[tokio::test]
async fn echo_route_handles_missing_query_string_gracefully() {
    // Arrange
    let addr = start_test_server().await;
    let client = get_test_client();

    // Act
    let response = client
        .get(format!("http://{}/echo", addr))
        .send()
        .await
        .expect("Request to /echo failed");

    // Assert
    assert_eq!(response.status(), 200);

    let body: TestResponse<HashMap<String, String>> = response
        .json()
        .await
        .expect("Failed to deserialize JSON response");

    assert_eq!(body.msg, "success");
    assert!(body.content.is_empty(), "Expected empty content map");
}
