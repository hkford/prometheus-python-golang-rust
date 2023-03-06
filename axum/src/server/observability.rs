use axum::{
    body::Body,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use http::Request;
use metrics::{histogram, increment_counter};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::time::Instant;
use tower::ServiceBuilder;
use tracing::info;

pub fn add_service_builder(router: Router<(), Body>) -> Router<(), Body> {
    let recorder_handle = setup_metrics_recorder();

    router
        .route("/metrics", get(|| async move { recorder_handle.render() }))
        .layer(
            ServiceBuilder::new()
                .layer(axum::middleware::from_fn(track_metrics))
                .layer(axum::middleware::from_fn(access_log_on_request)),
        )
}

fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("axum_http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let path = req.uri().path().to_owned();
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    increment_counter!("axum_http_requests_total", &labels);
    histogram!("axum_http_requests_duration_seconds", latency, &labels);

    response
}

async fn access_log_on_request<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    if req.uri() != "/metrics" {
        info!("{} {}", req.method(), req.uri());
    }
    Ok(next.run(req).await)
}
