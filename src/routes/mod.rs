mod hello_world;
mod mirror_body_string;
use axum::{
    routing::{patch, post},
    Router,
};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", patch(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
}
