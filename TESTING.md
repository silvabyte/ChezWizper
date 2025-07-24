# Testing ChezWizper OpenAI API Integration

## Quick Tests

### 1. Test API Connectivity
```bash
export OPENAI_API_KEY="sk-your-key-here"
cargo run --bin test_api
```

### 2. Test Audio File Transcription
```bash
# Transcribe any audio file using OpenAI API
./target/release/transcribe_file your_audio.wav

# With specific language
./target/release/transcribe_file your_audio.wav --language es

# Verbose output
./target/release/transcribe_file your_audio.wav --verbose
```

### 3. Test Full Application Server
```bash
# Start the server
export OPENAI_API_KEY="sk-your-key"
./target/release/chezwizper --config example_config_api.toml

# In another terminal, test the API
curl -X POST http://127.0.0.1:3737/toggle
```

## Creating Test Audio Files

### Using ffmpeg (if available):
```bash
# Generate a 5-second sine wave
ffmpeg -f lavfi -i "sine=frequency=440:duration=5" -ar 16000 -ac 1 test_sine.wav

# Generate speech-like audio
ffmpeg -f lavfi -i "sine=frequency=200:duration=2, sine=frequency=400:duration=2" -ar 16000 -ac 1 test_speech.wav
```

### Using sox (if available):
```bash
sox -n -r 16000 -c 1 test_tone.wav synth 3 sine 440
```

### Using system audio:
```bash
# Record from microphone (Linux with ALSA)
arecord -f cd -t wav -d 5 test_recording.wav

# Record from microphone (macOS)
sox -t coreaudio default test_recording.wav trim 0 5
```

## Expected Results

### API Mode Success:
```
🎤 Transcribing audio file: test.wav
📁 File size: 160044 bytes
🌐 Using OpenAI API (model: whisper-1)
⏳ Transcribing... (this may take a moment)
✅ Transcription completed in 2.34s
📝 Result (25 chars):
─────────────────────────────────────
This is a test recording.
─────────────────────────────────────
```

### Server Mode Success:
```
INFO chezwizper: Starting ChezWizper
INFO chezwizper::config: Loaded config from "example_config_api.toml"  
INFO chezwizper::whisper::api_client: Initialized OpenAI client
INFO chezwizper::whisper: Using OpenAI API for transcription
ChezWizper is ready!
Add this to your Hyprland config:
bind = CTRL SHIFT, R, exec, curl -X POST http://127.0.0.1:3737/toggle
```

## Troubleshooting

### Common Issues:

1. **"OPENAI_API_KEY environment variable is required"**
   ```bash
   export OPENAI_API_KEY="sk-your-actual-key-here"
   ```

2. **"Audio file not found"**
   - Ensure the file path is correct
   - Supported formats: WAV, MP3, M4A, FLAC

3. **"OpenAI API error: Incorrect API key"**
   - Verify your API key is valid and active
   - Check your OpenAI account status

4. **"No text injection tool found"**
   - This is expected if not on Wayland
   - The transcription still works, just can't inject text

5. **Network errors**
   - Check internet connection
   - Verify firewall allows HTTPS to api.openai.com

### Debug Mode:
```bash
# Enable verbose logging
./target/release/transcribe_file your_audio.wav --verbose

# Or set log level
RUST_LOG=debug ./target/release/chezwizper --config example_config_api.toml
```

## Performance Notes

- **API Latency**: Usually 1-5 seconds depending on file size
- **File Size Limits**: OpenAI has a 25MB limit per file
- **Cost**: ~$0.006 per minute of audio
- **Supported Formats**: WAV, MP3, MP4, MPEG, MPGA, M4A, WEBM, FLAC

## Integration Test Script

Run the comprehensive test:
```bash
chmod +x test_with_audio.sh
./test_with_audio.sh
```