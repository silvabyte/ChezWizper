use anyhow::Result;
use chezwizper::whisper::{WhisperTranscriber, ApiProvider};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;
use clap::Parser;

#[derive(Parser)]
#[command(name = "transcribe_file")]
#[command(about = "Transcribe an audio file using ChezWizper's API integration")]
struct Args {
    #[arg(help = "Path to the audio file to transcribe")]
    audio_file: String,
    
    #[arg(short, long, default_value = "whisper-large-v3-turbo")]
    model: String,
    
    #[arg(short, long, default_value = "groq", help = "API provider (groq or openai)")]
    provider: String,
    
    #[arg(short, long, default_value = "en")]
    language: String,
    
    #[arg(long, help = "Use CLI mode instead of API")]
    cli: bool,
    
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists
    let _ = dotenv::dotenv();
    
    let args = Args::parse();
    
    // Initialize logging
    let log_level = if args.verbose { "debug" } else { "info" };
    let env_filter = EnvFilter::try_new(log_level).unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    let audio_path = PathBuf::from(&args.audio_file);
    
    if !audio_path.exists() {
        eprintln!("❌ Error: Audio file '{}' not found", args.audio_file);
        std::process::exit(1);
    }
    
    println!("🎤 Transcribing audio file: {}", args.audio_file);
    println!("📁 File size: {} bytes", std::fs::metadata(&audio_path)?.len());
    
    if !args.cli {
        // API mode
        let api_key_var = match args.provider.to_lowercase().as_str() {
            "groq" => "GROQ_API_KEY",
            "openai" => "OPENAI_API_KEY",
            _ => "GROQ_API_KEY",
        };
        
        if std::env::var(api_key_var).is_err() {
            eprintln!("❌ Error: {} environment variable not set", api_key_var);
            eprintln!("   Set it with: export {}='your-key'", api_key_var);
            std::process::exit(1);
        }
        println!("🌐 Using {} API (model: {})", args.provider, args.model);
    } else {
        println!("💻 Using local CLI mode");
    }
    
    let api_provider = match args.provider.to_lowercase().as_str() {
        "groq" => ApiProvider::Groq,
        "openai" => ApiProvider::OpenAI,
        _ => ApiProvider::Groq,
    };
    
    let endpoint = match args.provider.to_lowercase().as_str() {
        "groq" => Some("https://api.groq.com/openai/v1/audio/transcriptions".to_string()),
        "openai" => Some("https://api.openai.com/v1/audio/transcriptions".to_string()),
        _ => Some("https://api.groq.com/openai/v1/audio/transcriptions".to_string()),
    };
    
    let transcriber = WhisperTranscriber::new(
        None,
        !args.cli, // use_api = !cli
        api_provider,
        endpoint
    )?
    .with_model(args.model)
    .with_language(args.language);
    
    println!("⏳ Transcribing... (this may take a moment)");
    
    let start_time = std::time::Instant::now();
    match transcriber.transcribe(&audio_path).await {
        Ok(text) => {
            let duration = start_time.elapsed();
            println!("✅ Transcription completed in {:.2}s", duration.as_secs_f64());
            println!("📝 Result ({} chars):", text.len());
            println!("─────────────────────────────────────");
            println!("{}", text);
            println!("─────────────────────────────────────");
        }
        Err(e) => {
            println!("❌ Transcription failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}