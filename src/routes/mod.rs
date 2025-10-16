use axum::{routing::get, Extension, Router};
use reqwest::Client;
use std::sync::Arc;

use crate::{models::{ProfileResponse, User}};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(crate::handlers::profile::me_handler), // <-- fully qualified path
    components(schemas(ProfileResponse, User)),
    tags((name = "Profile", description = "Your dynamic profile API"))
)]
pub struct ApiDoc;

pub fn create_router(
    client: Arc<Client>,
) -> Router {
    let openapi = ApiDoc::openapi();

    Router::new()
        .route("/me", get(crate::handlers::profile::me_handler))
        .route("/", get(crate::handlers::health))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
        .layer(Extension(client))
}
