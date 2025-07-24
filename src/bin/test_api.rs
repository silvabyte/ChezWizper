use anyhow::Result;
use chezwizper::whisper::WhisperTranscriber;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let env_filter = EnvFilter::try_new("debug").unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    println!("Testing OpenAI API integration...");

    // Test API mode (requires OPENAI_API_KEY environment variable)
    if std::env::var("OPENAI_API_KEY").is_ok() {
        println!("✓ OPENAI_API_KEY found in environment");
        
        match WhisperTranscriber::new(
            None, 
            true, 
            Some("https://api.openai.com/v1/audio/transcriptions".to_string())
        ) {
            Ok(transcriber) => {
                println!("✓ OpenAI API client initialized successfully");
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
        println!("✗ OPENAI_API_KEY not found in environment");
        println!("  Set OPENAI_API_KEY to test API functionality");
    }

    // Test CLI mode (fallback)
    println!("\nTesting CLI mode fallback...");
    match WhisperTranscriber::new(None, false, None) {
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