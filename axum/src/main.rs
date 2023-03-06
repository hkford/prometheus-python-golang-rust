mod handlers;
mod server;
use server::init;

#[tokio::main]
async fn main() {
    init::run().await;
}
