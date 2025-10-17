// tests/me_success.rs
use axum::{
    body::Body,
    http::{Request, StatusCode, header::CONTENT_TYPE},
};
use std::sync::Arc;
use tower::ServiceExt; // for .oneshot
use serde_json::Value;

// import your router
use dynamic_profile_endpoint::routes::create_router;

#[tokio::test]
async fn me_returns_200_and_success_status() {
    // Build app with a real client (success path depends on external API availability,
    // but we only assert shape and field values if 200)
    let client = Arc::new(reqwest::Client::new());
    let app = create_router(client);

    let resp = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Must be 200 OK
    assert_eq!(resp.status(), StatusCode::OK);

    // Must be JSON
    let content_type = resp.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap();
    assert!(content_type.starts_with("application/json"));

    // Read body
    use http_body_util::BodyExt; // for .collect()
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();

    // Parse and assert contract
    let v: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v.get("status").and_then(|s| s.as_str()), Some("success"));
    assert!(v.get("user").is_some());
    assert!(v.get("timestamp").is_some());
    assert!(v.get("fact").is_some());

    // Optional: check user subfields exist
    let user = v.get("user").unwrap();
    assert!(user.get("email").is_some());
    assert!(user.get("name").is_some());
    assert!(user.get("stack").is_some());
}
