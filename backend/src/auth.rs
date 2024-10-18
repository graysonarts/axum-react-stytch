use super::app_state::AppState;
use axum::{routing::post, Json, Router};
use common_types::auth::RegisterUserRequest;

pub fn routes() -> Router<AppState> {
    Router::new().route("/register", post(new_user_logged_in_handler))
}

async fn new_user_logged_in_handler(Json(body): Json<RegisterUserRequest>) -> &'static str {
    tracing::info!("New User: {:?}", body);
    "User logged in"
}
