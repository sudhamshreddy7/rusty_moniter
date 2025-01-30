use axum::{routing::get, Router, Json};
use serde_json::json;
use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom, Read},
    sync::{Arc, Mutex},
    net::SocketAddr,
};
use tokio::net::TcpListener;

const LOG_FILE_PATH: &str = "/home/ubuntu/logging.log";

#[tokio::main]
async fn main() {
    let log_data = Arc::new(Mutex::new(Vec::new()));

    // Spawn a background task to monitor log file changes
    let log_data_clone = log_data.clone();
    tokio::spawn(async move {
        monitor_log_file(log_data_clone).await;
    });

    // Setup API server
    let app = Router::new().route("/logs", get(move || fetch_logs(log_data.clone())));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8777));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

// Monitor log file for new lines and store them
async fn monitor_log_file(log_data: Arc<Mutex<Vec<String>>>) {
    let mut file = File::open(LOG_FILE_PATH).expect("Failed to open log file");
    
    let mut last_position = file.metadata().unwrap().len();
    
    loop {
        let new_len = file.metadata().unwrap().len();
        if new_len > last_position {
            file.seek(SeekFrom::Start(last_position)).unwrap();
            let mut new_lines = Vec::new();
            let mut reader = BufReader::new(&file);
            for line in reader.by_ref().lines() {
                if let Ok(line) = line {
                    println!("{}",line);
                    new_lines.push(line);
                }
            }

            if !new_lines.is_empty() {
                let mut logs = log_data.lock().unwrap();
                logs.extend(new_lines);
            }

            last_position = new_len;
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

// Fetch logs when API is hit
async fn fetch_logs(log_data: Arc<Mutex<Vec<String>>>) -> Json<serde_json::Value> {
    let mut logs = log_data.lock().unwrap();
    let response = json!({ "logs": *logs });

    // Clear buffer after sending
    logs.clear();

    Json(response)
}
