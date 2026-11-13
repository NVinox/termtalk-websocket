mod logger;
mod websocket;

#[tokio::main]
async fn main() {
    websocket::create().await;
}
