// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

type Clients = Arc<Mutex<Vec<tokio::sync::mpsc::UnboundedSender<String>>>>;

#[derive(Serialize, Deserialize, Debug)]
struct Telemetry {
    #[serde(rename = "isRaceOn")]
    is_race_on: i32,
    #[serde(rename = "maxRpm")]
    max_rpm: f32,
    rpm: f32,
    speed: f32,
    power: f32,
    torque: f32,
    boost: f32,
    gear: u8,
    throttle: f32,
    brake: f32,
    #[serde(rename = "carOrdinal")]
    car_ordinal: i32,
    #[serde(rename = "racePosition")]
    race_position: u8,  // NEW!
    #[serde(rename = "lapNumber")]
    lap_number: u16,    // NEW! (bonus)
}

fn parse_telemetry(data: &[u8]) -> Option<Telemetry> {
    if data.len() < 323 {
        return None;
    }

    // Parse using same byte offsets as Python forza_bridge.py
    Some(Telemetry {
        is_race_on: i32::from_le_bytes(data[0..4].try_into().ok()?),
        max_rpm: f32::from_le_bytes(data[8..12].try_into().ok()?),
        rpm: f32::from_le_bytes(data[16..20].try_into().ok()?),
        speed: f32::from_le_bytes(data[256..260].try_into().ok()?), // m/s
        power: f32::from_le_bytes(data[260..264].try_into().ok()?) / 745.7, // HP
        torque: f32::from_le_bytes(data[264..268].try_into().ok()?), // N·m
        boost: f32::from_le_bytes(data[284..288].try_into().ok()?), // PSI
        gear: data[319],
        throttle: data[315] as f32 / 255.0,
        brake: data[316] as f32 / 255.0,
        car_ordinal: i32::from_le_bytes(data[212..216].try_into().ok()?),
        lap_number: u16::from_le_bytes(data[312..314].try_into().ok()?),  // NEW!
        race_position: data[314],  // NEW!
    })
}

async fn udp_listener(clients: Clients) {
    let socket = UdpSocket::bind("127.0.0.1:5300").expect("Failed to bind UDP socket");
    socket.set_nonblocking(true).expect("Failed to set non-blocking");
    
    println!("[Bridge] Listening for Forza telemetry on 127.0.0.1:5300");
    
    let mut buffer = [0u8; 1024];
    
    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, _)) => {
                if let Some(telemetry) = parse_telemetry(&buffer[..size]) {
                    if let Ok(json) = serde_json::to_string(&telemetry) {
                        let mut clients = clients.lock().unwrap();
                        clients.retain(|tx| tx.send(json.clone()).is_ok());
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            }
            Err(e) => eprintln!("[Bridge] Error: {}", e),
        }
    }
}

async fn websocket_server(clients: Clients) {
    let listener = TcpListener::bind("127.0.0.1:8765").await.expect("Failed to bind WebSocket");
    println!("[WebSocket] Running on ws://127.0.0.1:8765");
    
    while let Ok((stream, _)) = listener.accept().await {
        let clients = clients.clone();
        
        tokio::spawn(async move {
            let ws_stream = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    eprintln!("[WebSocket] Error: {}", e);
                    return;
                }
            };
            
            let (mut ws_tx, _ws_rx) = ws_stream.split();
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            
            {
                let mut clients = clients.lock().unwrap();
                clients.push(tx);
                println!("[WebSocket] Client connected. Total: {}", clients.len());
            }
            
            while let Some(msg) = rx.recv().await {
                if ws_tx.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await.is_err() {
                    break;
                }
            }
            
            println!("[WebSocket] Client disconnected");
        });
    }
}

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(Vec::new()));
    
    // Start UDP listener
    let clients_udp = clients.clone();
    tokio::spawn(async move {
        udp_listener(clients_udp).await;
    });
    
    // Start WebSocket server
    let clients_ws = clients.clone();
    tokio::spawn(async move {
        websocket_server(clients_ws).await;
    });
    
    // Start Tauri window
    tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}