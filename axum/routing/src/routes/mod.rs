mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_user_agent;
mod mirror_custom_headers;
mod path_variables;
mod query_params;

use axum::{
    routing::{get, post},
    Router,
};

use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_headers::mirror_custom_headers;
use path_variables::{hard_coded_path, path_variables};
use query_params::query_params;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_headers", get(mirror_custom_headers))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
}
