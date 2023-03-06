use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    movies: Vec<Movie>,
}

impl AppState {
    pub fn new() -> Self {
        AppState { movies: Vec::new() }
    }
    pub fn add_movie(&mut self, movie: Movie) {
        self.movies.push(movie);
    }
    pub fn get_movies(&self) -> &Vec<Movie> {
        &self.movies
    }

    pub fn remove_movie(&mut self, index: usize) -> &Vec<Movie> {
        if index < self.movies.len() {
            let _ = &self.movies.remove(index);
        }
        &self.movies
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct Movie {
    id: u64,
}

impl Movie {
    pub fn new(id: u64) -> Self {
        Movie { id: id }
    }
}
