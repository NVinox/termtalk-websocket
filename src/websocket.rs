use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::{Mutex, mpsc};
use tokio::time::{Duration, timeout};

use tokio_tungstenite::accept_async_with_config;
use tokio_tungstenite::tungstenite::protocol::{Message, WebSocketConfig};

use futures_util::{SinkExt, StreamExt};

use crate::logger;
use crate::rate_limiter::RateLimiter;

const DEFAULT_PORT: u16 = 8000;
const MAX_MESSAGE_LENGTH: usize = 2000;
const AFK_USER_TIMEOUT: u64 = 300;

pub async fn create() {
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let listener = TcpListener::bind(address)
        .await
        .expect("Error to create websocket");
    let clients = Arc::new(Mutex::new(
        HashMap::<SocketAddr, mpsc::Sender<Message>>::new(),
    ));

    loop {
        let (stream, client_address) = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                print_log(&format!("TCP connect is error: {}", e));
                continue;
            }
        };

        print_log(format!("Client with IP {} is connected", client_address).as_str());

        let clients_clone = clients.clone();

        tokio::spawn(async move {
            let ws_stream = match accept_async_with_config(stream, Some(websocket_config())).await {
                Ok(stream) => stream,
                Err(e) => {
                    print_log(&format!(
                        "Handshake failed for IP {}: {}",
                        client_address, e
                    ));
                    return;
                }
            };

            print_log(format!("Client with IP {} websocket successed", client_address).as_str());

            let (tx, mut rx) = mpsc::channel::<Message>(100);
            let (mut ws_tx, mut ws_rx) = ws_stream.split();
            let mut rate_limiter = RateLimiter::new(5.0, 2.0);

            clients_clone.lock().await.insert(client_address, tx);

            tokio::spawn(async move {
                while let Some(message) = rx.recv().await {
                    if ws_tx.send(message).await.is_err() {
                        break;
                    }
                }
            });

            while let Ok(Some(Ok(msg))) =
                timeout(Duration::from_secs(AFK_USER_TIMEOUT), ws_rx.next()).await
            {
                match msg {
                    Message::Text(text) => {
                        if text.chars().count() > MAX_MESSAGE_LENGTH {
                            continue;
                        }

                        match rate_limiter.check() {
                            Ok(true) => {}
                            Ok(false) => continue,
                            Err(_) => {
                                print_log(&format!(
                                    "Client {} kicked for flooding",
                                    client_address
                                ));
                                break;
                            }
                        }

                        let all_clients = clients_clone.lock().await;
                        let message = Message::Text(text.into());

                        for tx_pipe in all_clients.values() {
                            let _ = tx_pipe.try_send(message.clone());
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                };
            }

            clients_clone.lock().await.remove(&client_address);
            print_log(format!("Client with IP {} disconnected", client_address).as_str());
        });
    }
}

fn websocket_config() -> WebSocketConfig {
    let mut config = WebSocketConfig::default();

    config.max_message_size = Some(64 * 1024);
    config.max_frame_size = Some(16 * 1024);

    config
}

fn print_log(message: &str) {
    println!("{}", message);
    logger::write_log(message);
}
