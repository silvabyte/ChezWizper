use anyhow::Result;
use chezwizper::whisper::{WhisperTranscriber, ApiProvider};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists
    let _ = dotenv::dotenv();
    
    // Initialize logging
    let env_filter = EnvFilter::try_new("debug").unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    println!("Testing API integration...");

    // Test Groq API mode (requires GROQ_API_KEY environment variable)
    if std::env::var("GROQ_API_KEY").is_ok() {
        println!("✓ GROQ_API_KEY found in environment");
        
        match WhisperTranscriber::new(
            None, 
            true, 
            ApiProvider::Groq,
            Some("https://api.groq.com/openai/v1/audio/transcriptions".to_string())
        ) {
            Ok(transcriber) => {
                println!("✓ Groq API client initialized successfully");
                println!("  Model: {}", transcriber.model);
                println!("  Using API: {}", transcriber.use_api);
                
                // Test would require an actual audio file
                println!("  API client ready for transcription");
            }
            Err(e) => {
                println!("✗ Failed to initialize API client: {}", e);
            }
        }
    } else {
        println!("✗ GROQ_API_KEY not found in environment");
        println!("  Set GROQ_API_KEY to test API functionality");
    }

    // Test CLI mode (fallback)
    println!("\nTesting CLI mode fallback...");
    match WhisperTranscriber::new(None, false, ApiProvider::Groq, None) {
        Ok(transcriber) => {
            println!("✓ CLI mode initialized successfully");
            println!("  Model: {}", transcriber.model);  
            println!("  Using API: {}", transcriber.use_api);
            println!("  OpenAI Whisper CLI: {}", transcriber.is_openai_whisper);
        }
        Err(e) => {
            println!("✗ CLI mode failed (expected if whisper CLI not installed): {}", e);
        }
    }

    Ok(())
}