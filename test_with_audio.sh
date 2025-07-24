#!/bin/bash

# Test script for ChezWizper OpenAI API integration
set -e

echo "🎤 ChezWizper OpenAI API Test"
echo "=============================="

# Check if API key is set
if [ -z "$OPENAI_API_KEY" ]; then
    echo "❌ Error: OPENAI_API_KEY environment variable not set"
    echo "   Please set it with: export OPENAI_API_KEY='sk-your-key'"
    exit 1
fi

echo "✅ API key found in environment"

# Build the project
echo "🔨 Building ChezWizper..."
cargo build --release

# Test API connectivity
echo "🌐 Testing API connectivity..."
cargo run --bin test_api

# Create a simple test audio file using system tools
echo "🎵 Creating test audio file..."
if command -v sox &> /dev/null; then
    # Use Sox to generate a 3-second sine wave
    sox -n -r 16000 -c 1 test_audio.wav synth 3 sine 440
    echo "✅ Created test audio with sox"
elif command -v ffmpeg &> /dev/null; then
    # Use ffmpeg to generate test audio
    ffmpeg -f lavfi -i "sine=frequency=440:duration=3" -ar 16000 -ac 1 -y test_audio.wav -v quiet
    echo "✅ Created test audio with ffmpeg"
else
    echo "⚠️  No audio generation tool found (sox or ffmpeg)"
    echo "   Please install one to test with actual audio"
    echo "   Or provide your own test_audio.wav file"
    if [ ! -f "test_audio.wav" ]; then
        echo "❌ No test_audio.wav file found"
        exit 1
    fi
fi

# Test direct API call (if we have an audio file)
if [ -f "test_audio.wav" ]; then
    echo "🎯 Testing direct OpenAI API call..."
    
    # Create a simple Rust test for direct transcription
    cat > src/bin/direct_test.rs << 'EOF'
use anyhow::Result;
use chezwizper::whisper::WhisperTranscriber;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let env_filter = EnvFilter::try_new("info").unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    println!("🎤 Testing direct audio transcription...");
    
    let transcriber = WhisperTranscriber::new(
        None, 
        true, 
        Some("https://api.openai.com/v1/audio/transcriptions".to_string())
    )?;
    
    let audio_path = PathBuf::from("test_audio.wav");
    if !audio_path.exists() {
        println!("❌ test_audio.wav not found");
        return Ok(());
    }
    
    match transcriber.transcribe(&audio_path).await {
        Ok(text) => {
            println!("✅ Transcription successful!");
            println!("📝 Result: '{}'", text);
        }
        Err(e) => {
            println!("❌ Transcription failed: {}", e);
        }
    }
    
    Ok(())
}
EOF

    # Add the new binary to Cargo.toml
    if ! grep -q "direct_test" Cargo.toml; then
        cat >> Cargo.toml << 'EOF'

[[bin]]
name = "direct_test"
path = "src/bin/direct_test.rs"
EOF
    fi

    # Build and run the direct test
    cargo build --release
    cargo run --bin direct_test
    
    # Cleanup
    rm -f src/bin/direct_test.rs
    rm -f test_audio.wav
else
    echo "⚠️  Skipping audio transcription test (no audio file)"
fi

echo ""
echo "🎉 Test completed!"
echo "   If you saw '✅ Transcription successful!' above, the API integration is working!"
echo ""
echo "💡 To test the full application:"
echo "   1. Make sure you're on a Wayland system with wtype installed"
echo "   2. Run: ./target/release/chezwizper --config example_config_api.toml"
echo "   3. Use the keybind or curl to trigger recording"