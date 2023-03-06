use crate::server::router;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::fmt;

pub async fn run() {
    let event_format = fmt::format::json();
    tracing_subscriber::fmt().event_format(event_format).init();

    let app = router::build_app_router();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
