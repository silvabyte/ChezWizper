use anyhow::Result;
use hound::WavReader;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(about = "Analyze ChezWizper audio files")]
struct Args {
    #[arg(help = "Path to WAV file")]
    wav_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = PathBuf::from(&args.wav_file);
    
    if !path.exists() {
        eprintln!("❌ File not found: {}", args.wav_file);
        std::process::exit(1);
    }
    
    println!("🎵 Analyzing: {}", args.wav_file);
    println!("─────────────────────────────");
    
    // Get file size
    let metadata = std::fs::metadata(&path)?;
    let size_mb = metadata.len() as f64 / 1_048_576.0;
    println!("📁 File size: {:.2} MB", size_mb);
    
    if size_mb > 25.0 {
        println!("⚠️  WARNING: File exceeds OpenAI 25MB limit!");
    }
    
    // Read WAV header
    let reader = WavReader::open(&path)?;
    let spec = reader.spec();
    
    println!("🎚️  Sample rate: {} Hz", spec.sample_rate);
    println!("🔊 Channels: {}", spec.channels);
    println!("🎯 Bits per sample: {}", spec.bits_per_sample);
    println!("📊 Sample format: {:?}", spec.sample_format);
    
    // Calculate duration
    let num_samples = reader.len();
    let duration_secs = num_samples as f64 / spec.sample_rate as f64;
    let duration_mins = duration_secs / 60.0;
    
    println!("⏱️  Duration: {:.1} seconds ({:.1} minutes)", duration_secs, duration_mins);
    println!("📈 Total samples: {}", num_samples);
    
    // Analyze audio levels
    let samples: Vec<i16> = reader.into_samples::<i16>()
        .filter_map(Result::ok)
        .collect();
    
    if !samples.is_empty() {
        let max_sample = samples.iter().map(|&s| s.abs()).max().unwrap_or(0);
        let avg_sample: f64 = samples.iter()
            .map(|&s| s.abs() as f64)
            .sum::<f64>() / samples.len() as f64;
        
        let max_amplitude = max_sample as f64 / i16::MAX as f64;
        let avg_amplitude = avg_sample / i16::MAX as f64;
        
        println!("\n📊 Audio Analysis:");
        println!("   Max amplitude: {:.1}% ({}/32767)", max_amplitude * 100.0, max_sample);
        println!("   Avg amplitude: {:.1}%", avg_amplitude * 100.0);
        
        if max_amplitude < 0.01 {
            println!("   ⚠️  WARNING: Audio is nearly silent!");
        } else if max_amplitude < 0.1 {
            println!("   ⚠️  WARNING: Audio level is very low");
        }
        
        // Check for silence
        let silence_threshold = 100; // ~0.3% of max
        let silent_samples = samples.iter()
            .filter(|&&s| s.abs() < silence_threshold)
            .count();
        let silence_percent = (silent_samples as f64 / samples.len() as f64) * 100.0;
        
        println!("   Silence: {:.1}% of recording", silence_percent);
        
        if silence_percent > 90.0 {
            println!("   ⚠️  WARNING: Recording is mostly silent!");
        }
    }
    
    println!("\n💡 OpenAI Whisper API limits:");
    println!("   - Max file size: 25 MB");
    println!("   - Supported formats: mp3, mp4, mpeg, mpga, m4a, wav, webm");
    println!("   - Optimal: 16kHz mono WAV");
    
    if size_mb > 25.0 {
        let max_duration = 25.0 * 1_048_576.0 / (spec.sample_rate as f64 * spec.channels as f64 * 2.0);
        println!("\n🔧 To fix file size issue:");
        println!("   - Max recording duration: {:.1} minutes at current settings", max_duration / 60.0);
        println!("   - Current duration: {:.1} minutes", duration_mins);
    }
    
    Ok(())
}