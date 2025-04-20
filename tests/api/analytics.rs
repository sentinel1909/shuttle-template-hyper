use crate::helpers::{get_test_client, start_test_server};
use std::collections::HashMap;

#[tokio::test]
async fn metrics_reflects_ping_activity() {
    // Arrange
    let addr = start_test_server().await;
    let client = get_test_client();

    // Act 1: Get initial metrics
    let resp = client
        .get(format!("http://{}/metrics", addr))
        .send()
        .await
        .expect("Failed to call /metrics");
    assert_eq!(resp.status(), 200);

    let metrics: HashMap<String, usize> = resp
        .json()
        .await
        .expect("Failed to parse JSON from /metrics");
    assert!(metrics.is_empty());

    // Act 2: Hit /ping three times
    for _ in 0..3 {
        let ping_resp = client
            .get(format!("http://{}/ping", addr))
            .send()
            .await
            .expect("Failed to call /ping");
        assert_eq!(ping_resp.status(), 200);
    }

    // Act 3: Get updated metrics
    let resp = client
        .get(format!("http://{}/metrics", addr))
        .send()
        .await
        .expect("Failed to call /metrics");
    assert_eq!(resp.status(), 200);

    let metrics: HashMap<String, usize> = resp
        .json()
        .await
        .expect("Failed to parse JSON from /metrics");

    assert_eq!(metrics.get("ping"), Some(&3));
}
