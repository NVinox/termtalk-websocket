use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::{Mutex, mpsc};

use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use futures_util::{SinkExt, StreamExt};

pub async fn create() {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8000);
    let listener = TcpListener::bind(address)
        .await
        .expect("Error to create websocket");
    let clients = Arc::new(Mutex::new(HashMap::<
        SocketAddr,
        mpsc::UnboundedSender<Message>,
    >::new()));

    loop {
        let (stream, client_address) = listener.accept().await.unwrap();
        println!("Client with ip {} is connected", client_address);

        let clients_clone = clients.clone();

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            println!("Client with ip {} websocket successed", client_address);

            let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
            let (mut ws_tx, mut ws_rx) = ws_stream.split();

            clients_clone.lock().await.insert(client_address, tx);

            tokio::spawn(async move {
                while let Some(message) = rx.recv().await {
                    if ws_tx.send(message).await.is_err() {
                        break;
                    }
                }
            });

            while let Some(Ok(message)) = ws_rx.next().await {
                if message.is_text() {
                    let text = message.to_text().unwrap();
                    let broadcast_msg = Message::Text(text.to_string().into());
                    let all_clients = clients_clone.lock().await;

                    for tx_pipe in all_clients.values() {
                        let _ = tx_pipe.send(broadcast_msg.clone());
                    }
                }
            }
        });
    }
}
