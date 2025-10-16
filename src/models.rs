use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub email: String,
    pub name: String,
    pub stack: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ProfileResponse {
    pub status: String,  // "success"
    pub user: User,
    pub timestamp: String,
    pub fact: String,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: String,    // "failed"
    pub message: String,   // friendly text
    pub status_code: u16,  // mirrors HTTP status we return
}
