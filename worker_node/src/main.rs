use axum::{routing::get, Router, Json};
use serde_json::json;
use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom, Read},
    sync::{Arc, Mutex},
    net::SocketAddr,
    env,
};
use psutil::process::Process; 
use chrono::Local;
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
    let app = Router::new()
        .route("/logs", get(move || fetch_logs(log_data.clone())))
        .route("/details", get(move || fetch_details()));
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
    let current_time = Local::now().to_string();
    let response = json!({ "logs": *logs ,"Time": current_time});

    // Clear buffer after sending
    logs.clear();

    Json(response)
}
// async fn fetch_processes() -> Json<serde_json::Value>{
//     let mem = sys_info::mem_info().unwrap();
//     let system_info = json!({
//         "total_memory_kb": mem.total / 1024,
//         "free_memory_kb": mem.free / 1024,
//     });

//     let mut processes_info = Vec::new();
//     match psutil::process::all() {
//         Ok(processes) => {
//             for proc in processes {
//                 match proc {
//                     Ok(proc) => {
//                         let pid = proc.pid();
//                         let name = proc.name().unwrap_or_else(|_| "Unknown".to_string());
//                         let memory_info = proc.memory_info().unwrap();
//                         let memory_usage_kb = memory_info.rss / 1024; // Memory usage in KB

//                         let process_info = json!({
//                             "pid": pid,
//                             "name": name,
//                             "memory_usage_kb": memory_usage_kb,
//                         });

//                         processes_info.push(process_info);
//                     }
//                     Err(_) => {}
//                 }
//             }
//         }
//         Err(_) => {
//             // Handle error if processes couldn't be retrieved
//             return Json(json!({ "error": "Could not retrieve processes" }));
//         }
//     }

//     // Combine system info and processes info into one JSON object
//     let result = json!({
//         "system_info": system_info,
//         "processes": processes_info
//     });

//     Json(result)
// }
async fn fetch_details() -> Json<serde_json::Value>{
    let current_time = Local::now().to_string();
    let total_memory = sys_info::mem_info().unwrap();
    let disk = sys_info::disk_info().unwrap();
    let os = env::consts::OS.to_string();
    let total_memory_used =  total_memory.total / 1024 - total_memory.free / 1024;
    let total_memory = total_memory.total / 1024;
    let disk_used = disk.total / 1024 - disk.free / 1024;
    let disk = disk.total;
    // let process = fetch_processes().await;
    let response = json!({ "Time": current_time,
                            "OS": os,
                            "Total memory": total_memory,
                            "Memory used": total_memory_used,
                            "Total disk used": disk,
                            "Disk used": disk_used,
                            // "Process": process,
                        });

    Json(response)
}