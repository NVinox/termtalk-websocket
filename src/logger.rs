use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

use chrono::Local;

const DIR_NAME: &'static str = "logs";

pub fn write_log(message: &str) {
    if let Ok(path) = get_path() {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let time = Local::now().to_utc().format("%H:%M:%S").to_string();
            let _ = writeln!(file, "[{}]: {}", time, message);
        }
    }
}

fn get_path() -> io::Result<PathBuf> {
    let mut path = env::current_dir()?;
    let file_name = Local::now().to_utc().format("%d-%m-%Y").to_string();

    path.push(DIR_NAME);

    fs::create_dir_all(&path)?;

    path.push(format!("{}.log", file_name));

    Ok(path)
}
