use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;

use backend_wizard_profile::routes::create_router;

#[tokio::test]
async fn test_me_endpoint() {
    std::env::set_var("PORT", "0");

    let client = Arc::new(reqwest::Client::new());
    let app = create_router(client);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
