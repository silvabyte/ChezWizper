# Whisper Transcription Setup Guide

ChezWizper supports multiple transcription providers for converting speech to text. This guide explains the available providers, how to set them up, and configuration options.

## Automatic Provider Detection

ChezWizper automatically detects the best available transcription provider based on:

1. **User preference** (if specified in config)
2. **Environment variables** (e.g., OPENAI_API_KEY)
3. **Available tools** on your system

The detection priority is:
- If user specifies a provider in config, use that
- If OPENAI_API_KEY is set, prefer OpenAI API
- Check for OpenAI Whisper CLI installation
- Check for whisper.cpp installation
- Return error if no provider is available

## Supported Providers

### 1. OpenAI API (Recommended)

**Best for**: High accuracy, no local setup required, consistent performance

**Requirements**:
- OpenAI API key
- Internet connection

**Setup**:
```bash
# Set your OpenAI API key
export OPENAI_API_KEY="sk-your-api-key-here"

# Add to your shell profile for persistence
echo 'export OPENAI_API_KEY="sk-your-api-key-here"' >> ~/.bashrc
```

**Configuration**:
```toml
[whisper]
provider = "openai-api"
model = "whisper-1"     # Currently the only available model
language = "en"         # ISO 639-1 language code, or "auto"
# api_endpoint = "https://custom.endpoint/v1/audio/transcriptions"  # Optional
```

**Cost**: $0.006 per minute of audio (as of 2025)

### 2. OpenAI Whisper CLI

**Best for**: Local processing, no API costs, privacy-conscious users

**Installation**:
```bash
# Using pip (recommended)
pip install openai-whisper

# Arch Linux AUR
yay -S openai-whisper

# Verify installation
whisper --help | grep output_format  # Should show OpenAI-specific options
```

**Configuration**:
```toml
[whisper]
provider = "openai-cli"
model = "base"          # tiny, base, small, medium, large-v3
language = "en"         # ISO 639-1 language code
# command_path = "/usr/local/bin/whisper"  # Optional custom path
```

**Models**:
- `tiny` - 39M parameters, ~1GB VRAM
- `base` - 74M parameters, ~1GB VRAM (default)
- `small` - 244M parameters, ~2GB VRAM
- `medium` - 769M parameters, ~5GB VRAM
- `large-v3` - 1550M parameters, ~10GB VRAM

### 3. whisper.cpp (Experimental)

**Best for**: Resource-constrained systems, CPU-only inference

**Note**: This provider is experimental and may have compatibility issues.

**Installation**:
```bash
# Build from source
git clone https://github.com/ggerganov/whisper.cpp
cd whisper.cpp
make

# Download models
bash ./models/download-ggml-model.sh base

# Install binary
sudo cp main /usr/local/bin/whisper

# Arch Linux AUR
yay -S whisper.cpp
```

**Configuration**:
```toml
[whisper]
provider = "whisper-cpp"
model = "base"          # Model name (without ggml- prefix)
language = "en"
# command_path = "/usr/local/bin/whisper"  # Optional
# model_path = "/path/to/ggml-base.bin"    # Optional custom model path
```

## Configuration Examples

### Minimal (auto-detection)
```toml
[whisper]
# Provider not specified - auto-detect best available
model = "base"
language = "en"
```

### Explicit OpenAI API
```toml
[whisper]
provider = "openai-api"
model = "whisper-1"
language = "en"  # or "auto" for automatic detection
```

### Local OpenAI Whisper
```toml
[whisper]
provider = "openai-cli"
model = "small"      # Better accuracy than base
language = "en"
```

### whisper.cpp with Custom Model
```toml
[whisper]
provider = "whisper-cpp"
model = "base"
model_path = "/home/user/models/ggml-base.bin"
language = "en"
```


## Provider Comparison

| Feature | OpenAI API | OpenAI CLI | whisper.cpp |
|---------|------------|------------|-------------|
| Accuracy | ⭐⭐⭐⭐⭐ Best | ⭐⭐⭐⭐ Excellent | ⭐⭐⭐ Good |
| Speed | ⭐⭐⭐⭐ Fast | ⭐⭐⭐ Varies | ⭐⭐ Slower |
| Cost | 💰 $0.006/min | Free | Free |
| Privacy | ☁️ Cloud | 🏠 Local | 🏠 Local |
| Setup | ⭐⭐⭐⭐⭐ Easy | ⭐⭐⭐ Moderate | ⭐⭐ Complex |
| GPU Required | No | Recommended | No |
| Internet | Required | Not required | Not required |

## Language Support

All providers support multiple languages. Common codes:
- `en` - English
- `es` - Spanish
- `fr` - French
- `de` - German
- `it` - Italian
- `pt` - Portuguese
- `ru` - Russian
- `zh` - Chinese
- `ja` - Japanese
- `ko` - Korean
- `ar` - Arabic
- `auto` - Automatic detection (OpenAI API only)

See [ISO 639-1 codes](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) for full list.

## Troubleshooting

### OpenAI API Issues

**"OPENAI_API_KEY environment variable is required"**:
```bash
# Check if key is set
echo $OPENAI_API_KEY

# Set the key
export OPENAI_API_KEY="sk-your-key-here"
```

**"OpenAI API error: Incorrect API key provided"**:
- Verify your API key at https://platform.openai.com/api-keys
- Check for typos or extra spaces

**"You exceeded your current quota"**:
- Check usage at https://platform.openai.com/usage
- Add billing information if needed

### OpenAI CLI Issues

**"Whisper CLI not found"**:
```bash
# Check if whisper is in PATH
which whisper

# Install with pip
pip install openai-whisper

# Or specify full path in config
command_path = "/home/user/.local/bin/whisper"
```

**Out of memory errors**:
- Use a smaller model (tiny or base)
- Close other applications
- Use OpenAI API instead

### whisper.cpp Issues

**"whisper.cpp: command not found"**:
```bash
# Check installation
which whisper

# Build from source (see installation section)
# Or specify path in config
```

**"Model file not found"**:
```bash
# Download the model
cd whisper.cpp
bash ./models/download-ggml-model.sh base

# Or specify custom path in config
model_path = "/path/to/model.bin"
```

## Performance Optimization

### OpenAI API
- Batch multiple short recordings if possible
- Consider audio preprocessing to reduce file size
- Use appropriate audio format (16kHz, mono recommended)

### Local Providers
- Use GPU acceleration when available (OpenAI CLI)
- Choose model size based on accuracy needs vs performance
- Consider running on dedicated hardware

### Audio Quality Tips
- Sample rate: 16000 Hz (16kHz) is optimal
- Channels: Mono (1 channel) is sufficient
- Format: WAV is universally supported
- Reduce background noise for better accuracy

## Security Considerations

### API Keys
- Never commit API keys to version control
- Use environment variables or secure key management
- Rotate keys regularly
- Set usage limits in OpenAI dashboard

### Local Processing
- OpenAI CLI and whisper.cpp process audio locally
- No data sent to external servers
- Suitable for sensitive content

### Network Security
- API requests use HTTPS
- Consider VPN for additional privacy
- Monitor API usage for anomalies

## Testing Your Setup

Test transcription providers:
```bash
# Test with sample audio (you'll need a test.wav file)
# API mode
OPENAI_API_KEY="your-key" cargo run --bin test_api

# Test with actual recording
./chezwizper --verbose

# Check which provider is being used
# Look for "Using [provider] for transcription" in logs
```

Manual provider testing:
```bash
# Test OpenAI CLI
whisper test.wav --model base --language en

# Test whisper.cpp
whisper -f test.wav -m models/ggml-base.bin -l en

# Test API with curl
curl https://api.openai.com/v1/audio/transcriptions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: multipart/form-data" \
  -F file="@test.wav" \
  -F model="whisper-1"
```

## Migration Guide

### Upgrading from Earlier Versions

If you're upgrading from earlier versions that used `use_api = true/false`, simply replace with the appropriate provider:

```toml
# Old: use_api = true
# New:
provider = "openai-api"

# Old: use_api = false  
# New:
provider = "openai-cli"  # or "whisper-cpp"
```

## Adding Custom Providers

The provider system is extensible. To add a new provider:

1. Implement the `TranscriptionProvider` trait
2. Add provider to the match statement in `WhisperTranscriber::with_provider`
3. Update auto-detection logic if needed

See `src/whisper/providers/` for examples.