mod api;
mod audio;
mod clipboard;
mod config;
mod normalizer;
mod text_injection;
mod transcription;
mod ui;
mod whisper;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crate::api::{ApiCommand, ApiServer};
use crate::audio::AudioStreamManager;
use crate::clipboard::ClipboardManager;
use crate::config::Config;
use crate::text_injection::TextInjector;
use crate::transcription::TranscriptionService;
use crate::ui::Indicator;
use crate::whisper::WhisperTranscriber;

#[derive(Parser)]
#[command(name = "chezwizper")]
#[command(about = "Voice transcription tool for Wayland/Hyprland", long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<PathBuf>,
    
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Clone)]
struct RecordingState {
    recording: Arc<Mutex<bool>>,
    audio_recorder: Arc<Mutex<AudioStreamManager>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize logging
    let log_level = if args.verbose { "debug" } else { "info" };
    let env_filter = EnvFilter::try_new(log_level).unwrap_or_else(|_| EnvFilter::new("info"));
    
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();
    
    info!("Starting ChezWizper");
    
    // Load configuration
    let config = Config::load()?;
    
    // Initialize components
    let (tx, mut rx) = mpsc::channel::<ApiCommand>(10);
    
    let audio_recorder = AudioStreamManager::new()?;
    
    // Build whisper transcriber
    let whisper = WhisperTranscriber::new(config.whisper.command_path.clone())?
        .with_model(config.whisper.model.clone())
        .with_model_path(config.whisper.model_path.clone())
        .with_language(config.whisper.language.clone());
    
    // Compose transcription service with whisper and normalizer
    let transcription_service = TranscriptionService::new(whisper)?;
    
    let text_injector = TextInjector::new(Some(&config.wayland.input_method))?;
    let mut clipboard = ClipboardManager::new()?
        .with_preserve(config.behavior.preserve_clipboard);
    
    let indicator = Indicator::new()
        .with_audio_feedback(config.behavior.audio_feedback);
    
    let recording_flag = Arc::new(Mutex::new(false));
    let state = RecordingState {
        recording: recording_flag.clone(),
        audio_recorder: Arc::new(Mutex::new(audio_recorder)),
    };
    
    // Create and start API server
    let api_server = ApiServer::new(tx, recording_flag.clone());
    
    // Start API server in background
    tokio::spawn(async move {
        if let Err(e) = api_server.start().await {
            error!("API server failed: {}", e);
        }
    });
    
    // Print instructions for Hyprland setup
    info!("ChezWizper is ready!");
    info!("Add this to your Hyprland config:");
    info!("bind = CTRL SHIFT, R, exec, -e curl -X POST http://127.0.0.1:3737/toggle");
    info!("Or test manually: curl -X POST http://127.0.0.1:3737/toggle");
    
    // Main event loop
    while let Some(command) = rx.recv().await {
        match command {
            ApiCommand::ToggleRecording => {
                let mut recording = state.recording.lock().await;
                *recording = !*recording;
                
                if *recording {
                    // Start recording
                    info!("Starting recording");
                    
                    if let Err(e) = indicator.show_recording().await {
                        error!("Failed to show recording indicator: {}", e);
                    }
                    
                    let audio_recorder = state.audio_recorder.lock().await;
                    if let Err(e) = audio_recorder.start_recording().await {
                        error!("Failed to start recording: {}", e);
                        *recording = false;
                        let _ = indicator.show_error(&format!("Recording failed: {}", e)).await;
                        continue;
                    }
                } else {
                    // Stop recording and process
                    info!("Stopping recording");
                    
                    let audio_recorder = state.audio_recorder.lock().await;
                    let temp_path = PathBuf::from(format!("/tmp/chezwizper_{}.wav", 
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()));
                    
                    match audio_recorder.stop_recording(temp_path.clone()).await {
                        Ok(_) => {
                            // Show processing indicator
                            if let Err(e) = indicator.show_processing().await {
                                error!("Failed to show processing indicator: {}", e);
                            }
                            
                            // Transcribe audio
                            match transcription_service.transcribe(&temp_path).await {
                                Ok(text) => {
                                    if !text.is_empty() {
                                        info!("Transcription successful: {} chars", text.len());
                                        
                                        // Copy to clipboard
                                        if let Err(e) = clipboard.copy_with_wayland_fallback(&text).await {
                                            error!("Failed to copy to clipboard: {}", e);
                                        }
                                        
                                        // Inject text or paste
                                        if config.behavior.auto_paste {
                                            if let Err(e) = text_injector.inject_text(&text).await {
                                                error!("Failed to inject text: {}, trying paste", e);
                                                let _ = text_injector.paste_from_clipboard().await;
                                            }
                                        }
                                        
                                        // Show completion
                                        if let Err(e) = indicator.show_complete(&text).await {
                                            error!("Failed to show completion indicator: {}", e);
                                        }
                                    } else {
                                        let _ = indicator.show_error("No speech detected").await;
                                    }
                                }
                                Err(e) => {
                                    error!("Transcription failed: {}", e);
                                    let _ = indicator.show_error(&format!("Transcription failed: {}", e)).await;
                                }
                            }
                            
                            // Clean up audio file
                            if config.behavior.delete_audio_files {
                                let _ = std::fs::remove_file(&temp_path);
                            }
                        }
                        Err(e) => {
                            error!("Failed to stop recording: {}", e);
                            let _ = indicator.show_error(&format!("Failed to save audio: {}", e)).await;
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}
