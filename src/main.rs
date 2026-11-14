mod logger;
mod websocket;

#[tokio::main]
async fn main() {
    logger::init_logger();
    websocket::create().await;
}
