use anyhow::Result;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tower::ServiceBuilder;
use tracing::{error, info};

#[derive(Clone)]
pub enum ApiCommand {
    ToggleRecording,
}

#[derive(Clone)]
pub struct AppState {
    tx: mpsc::Sender<ApiCommand>,
    recording: Arc<Mutex<bool>>,
}

pub struct ApiServer {
    port: u16,
    state: AppState,
}

impl ApiServer {
    pub fn new(tx: mpsc::Sender<ApiCommand>, recording: Arc<Mutex<bool>>) -> Self {
        Self {
            port: 3737, // WHSP in numbers
            state: AppState { tx, recording },
        }
    }

    pub async fn start(self) -> Result<()> {
        let app = Router::new()
            .route("/", get(status))
            .route("/toggle", post(toggle_recording))
            .route("/status", get(recording_status))
            .layer(ServiceBuilder::new())
            .with_state(self.state);

        let listener = tokio::net::TcpListener::bind(&format!("127.0.0.1:{}", self.port)).await?;

        info!("API server listening on http://127.0.0.1:{}", self.port);
        info!("Endpoints:");
        info!("  POST /toggle - Toggle recording");
        info!("  GET /status  - Get recording status");

        axum::serve(listener, app).await?;

        Ok(())
    }
}

async fn status() -> Json<Value> {
    Json(json!({
        "service": "chezwizper",
        "version": "0.1.0",
        "status": "running"
    }))
}

async fn toggle_recording(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    match state.tx.send(ApiCommand::ToggleRecording).await {
        Ok(_) => {
            info!("Toggle recording command received via API");
            Ok(Json(json!({
                "success": true,
                "message": "Recording toggled"
            })))
        }
        Err(e) => {
            error!("Failed to send toggle command: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn recording_status(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Json<Value> {
    let recording = *state.recording.lock().await;
    
    // Check if waybar style is requested
    if params.get("style") == Some(&"waybar".to_string()) {
        return Json(generate_waybar_response(recording));
    }
    
    // Default JSON response
    Json(json!({
        "recording": recording,
        "status": if recording { "recording" } else { "idle" }
    }))
}

fn generate_waybar_response(recording: bool) -> Value {
    json!({
        "text": if recording { "🔴 REC" } else { "" },
        "class": if recording { "chezwizper-recording" } else { "chezwizper-idle" },
        "tooltip": if recording { 
            "Recording... Press Super+R to stop" 
        } else { 
            "Press Super+R to record" 
        },
        "style": if recording {
            "color: #f7768e; background-color: rgba(247, 118, 142, 0.2); animation: recording-pulse 1s ease-in-out infinite;"
        } else {
            "padding: 0; margin: 0;"
        }
    })
}
