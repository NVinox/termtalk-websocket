mod logger;
mod rate_limiter;
mod websocket;

#[tokio::main]
async fn main() {
    logger::init_logger();
    websocket::create().await;
}
