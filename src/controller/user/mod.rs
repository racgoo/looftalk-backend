use axum::Router;
use axum::routing::get;

pub fn create_user_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
}
