use std::env;
use std::io::{self};
use std::path::PathBuf;
use std::sync::OnceLock;

use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;

use tokio::sync::mpsc::{self, UnboundedSender};

use chrono::Local;

const DIR_NAME: &'static str = "logs";

static LOG_TX: OnceLock<UnboundedSender<String>> = OnceLock::new();

pub fn init_logger() {
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    if LOG_TX.set(tx).is_err() {
        return;
    }

    tokio::spawn(async move {
        while let Some(formatted_line) = rx.recv().await {
            if let Ok(path) = get_path().await {
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .await
                {
                    let _ = file.write_all(formatted_line.as_bytes()).await;
                }
            }
        }
    });
}

pub fn write_log(message: &str) {
    let time = Local::now().format("%H:%M:%S");
    let message = format!("[{}]: {}\n", time, message);

    if let Some(tx) = LOG_TX.get() {
        let _ = tx.send(message);
    }
}

async fn get_path() -> io::Result<PathBuf> {
    let mut path = env::current_dir()?;
    let file_name = Local::now().to_utc().format("%d-%m-%Y").to_string();

    path.push(DIR_NAME);

    fs::create_dir_all(&path).await?;

    path.push(format!("{}.log", file_name));

    Ok(path)
}
