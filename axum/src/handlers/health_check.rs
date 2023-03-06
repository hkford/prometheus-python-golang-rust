use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct JsonMessage {
    message: String,
}

impl JsonMessage {
    pub fn new(message: &str) -> JsonMessage {
        JsonMessage {
            message: String::from(message),
        }
    }
}

// health check function
pub async fn health_check() -> impl IntoResponse {
    let m = JsonMessage::new("Healthy");
    Json(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    // for using health_check.into_service()
    use axum::handler::HandlerWithoutStateExt;
    use axum_test_helper::TestClient;
    #[tokio::test]
    async fn health_check_response() -> Result<(), ()> {
        let client = TestClient::new(health_check.into_service());

        let res = client.get("/").send().await;
        let got: JsonMessage = res.json().await;
        let expected = JsonMessage::new("Healthy");
        assert_eq!(got, expected);
        Ok(())
    }
}
