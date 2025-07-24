use anyhow::Result;
use chezwizper::whisper::WhisperTranscriber;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;
use clap::Parser;

#[derive(Parser)]
#[command(name = "transcribe_file")]
#[command(about = "Transcribe an audio file using ChezWizper's OpenAI API integration")]
struct Args {
    #[arg(help = "Path to the audio file to transcribe")]
    audio_file: String,
    
    #[arg(short, long, default_value = "whisper-1")]
    model: String,
    
    #[arg(short, long, default_value = "en")]
    language: String,
    
    #[arg(long, help = "Use CLI mode instead of API")]
    cli: bool,
    
    #[arg(short, long)]
    verbose: bool,
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

    let audio_path = PathBuf::from(&args.audio_file);
    
    if !audio_path.exists() {
        eprintln!("❌ Error: Audio file '{}' not found", args.audio_file);
        std::process::exit(1);
    }
    
    println!("🎤 Transcribing audio file: {}", args.audio_file);
    println!("📁 File size: {} bytes", std::fs::metadata(&audio_path)?.len());
    
    if !args.cli {
        // API mode
        if std::env::var("OPENAI_API_KEY").is_err() {
            eprintln!("❌ Error: OPENAI_API_KEY environment variable not set");
            eprintln!("   Set it with: export OPENAI_API_KEY='sk-your-key'");
            std::process::exit(1);
        }
        println!("🌐 Using OpenAI API (model: {})", args.model);
    } else {
        println!("💻 Using local CLI mode");
    }
    
    let transcriber = WhisperTranscriber::new(
        None,
        !args.cli, // use_api = !cli
        Some("https://api.openai.com/v1/audio/transcriptions".to_string())
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