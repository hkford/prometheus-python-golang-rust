use crate::server::state;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use metrics::gauge;
use rand::random;
use std::sync::{Arc, Mutex};

pub async fn create_movie(
    State(app_state): State<Arc<Mutex<state::AppState>>>,
) -> impl IntoResponse {
    let id = random::<u64>();
    let movie = state::Movie::new(id);
    let app_movies = Arc::clone(&app_state);
    app_movies.lock().unwrap().add_movie(movie);
    let movies = app_movies.lock().unwrap().get_movies().to_owned();
    gauge!("axum_movies_count", movies.len() as f64);
    Json(movies)
}

pub async fn list_movies(
    State(app_state): State<Arc<Mutex<state::AppState>>>,
) -> impl IntoResponse {
    let app_movies = Arc::clone(&app_state);
    let movies = app_movies.lock().unwrap().get_movies().to_owned();
    Json(movies)
}

pub async fn delete_movie(
    Path(id): Path<usize>,
    State(app_state): State<Arc<Mutex<state::AppState>>>,
) -> impl IntoResponse {
    let app_movies = Arc::clone(&app_state);
    let movies = app_movies.lock().unwrap().remove_movie(id).to_owned();
    gauge!("axum_movies_count", movies.len() as f64);
    Json(movies)
}
