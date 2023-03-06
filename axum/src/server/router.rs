use axum::{
    body::Body,
    routing::{delete, get, post},
    Router,
};
use std::sync::{Arc, Mutex};
use crate::handlers::{health_check, movies};
use crate::server::{observability, state};

pub fn build_app_router() -> Router<(), Body>{
    let base_routes = build_base_routes();
    let movies_routes = build_movies_routes();
    let app = Router::new().merge(base_routes).merge(movies_routes);
    let app = observability::add_service_builder(app);
    app
}

fn build_base_routes() -> Router {
    let base_routes = Router::new()
    .route("/health", get(health_check::health_check));
    base_routes
}

fn build_movies_routes() -> Router {
    let app_state = Arc::new(Mutex::new(state::AppState::new()));
    let movies_routes = Router::new()
        .route("/movies", post(movies::create_movie))
        .route("/movies", get(movies::list_movies))
        .route("/movies/:id", delete(movies::delete_movie))
        .with_state(app_state);
    movies_routes
}