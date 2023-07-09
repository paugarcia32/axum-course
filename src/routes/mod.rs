mod hello_world;
use axum::{routing::patch, Router};
use hello_world::hello_world;

pub fn create_routes() -> Router {
    Router::new().route("/", patch(hello_world))
}
