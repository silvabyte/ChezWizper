use anyhow::Result;
use chezwizper::whisper::{ProviderConfig, WhisperTranscriber};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let env_filter = EnvFilter::try_new("debug").unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    println!("Testing OpenAI API integration...");

    // Test API mode (requires api_key in config)
    println!("Testing OpenAI API provider...");

    let config = ProviderConfig {
        model: Some("whisper-1".to_string()),
        api_endpoint: Some("https://api.openai.com/v1/audio/transcriptions".to_string()),
        language: Some("en".to_string()),
        api_key: Some("test-api-key".to_string()), // Would normally come from config
        ..Default::default()
    };
    match WhisperTranscriber::with_provider("openai-api", config) {
        Ok(_transcriber) => {
            println!("✓ OpenAI API client initialized successfully");
            println!("  API client ready for transcription");
        }
        Err(e) => {
            println!("✗ Failed to initialize API client: {e}");
        }
    }

    // Test CLI mode (fallback)
    println!("\nTesting CLI mode fallback...");
    let config = ProviderConfig::default();
    match WhisperTranscriber::auto_detect(config) {
        Ok(_transcriber) => {
            println!("✓ CLI mode initialized successfully");
            println!("  Local provider ready for transcription");
        }
        Err(e) => {
            println!("✗ CLI mode failed (expected if whisper CLI not installed): {e}");
        }
    }

    Ok(())
}
