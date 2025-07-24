# ChezWizper - OpenAI API Integration

This fork adds OpenAI Whisper API support to ChezWizper, allowing you to use OpenAI's cloud-based transcription service instead of local CLI tools.

## Features

- **OpenAI API Integration**: Use OpenAI's Whisper API for high-quality transcription
- **Backward Compatibility**: Still supports local whisper CLI (whisper.cpp, OpenAI Whisper CLI)
- **Flexible Configuration**: Easy switching between API and CLI modes
- **Environment-based Authentication**: Secure API key handling via environment variables

## Configuration

### API Mode (Recommended)

Create a config file with `use_api = true`:

```toml
[whisper]
model = "whisper-1"          # OpenAI API model
language = "en"              # Language code (optional)
use_api = true               # Enable API mode
api_endpoint = "https://api.openai.com/v1/audio/transcriptions"  # Optional custom endpoint

# Set OPENAI_API_KEY environment variable
```

### CLI Mode (Legacy)

```toml
[whisper]
model = "base"               # Local model
language = "en"
use_api = false              # Use local CLI
# command_path = "/path/to/whisper"  # Optional custom path
# model_path = "/path/to/model.bin"  # Optional custom model
```

## Setup

1. **Set OpenAI API Key**:
   ```bash
   export OPENAI_API_KEY="your-openai-api-key-here"
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Create config file**:
   ```bash
   cp example_config_api.toml ~/.config/chezwizper/config.toml
   # Edit the config file as needed
   ```

4. **Run ChezWizper**:
   ```bash
   ./target/release/chezwizper --config config.toml
   ```

## API Models

OpenAI supports the following Whisper models via API:
- `whisper-1` - Latest Whisper model (recommended)

## Supported Languages

Set the `language` field to any ISO-639-1 code. Examples:
- `en` - English
- `es` - Spanish  
- `fr` - French
- `de` - German
- `ja` - Japanese
- `zh` - Chinese
- Leave empty or set to `"auto"` for automatic detection

## Cost Considerations

OpenAI Whisper API pricing (as of 2025):
- $0.006 per minute of audio

For heavy usage, consider:
- Local CLI mode for cost savings
- Shorter recording sessions
- Audio preprocessing to reduce duration

## Testing

Test with configuration file:

```bash
OPENAI_API_KEY="your-key" ./target/release/chezwizper --config example_config_api.toml
```

Test direct file transcription:

```bash
./target/release/transcribe_file your_audio.wav
```

## Security

- API keys are only read from environment variables
- No API keys are stored in configuration files
- HTTPS used for all API communications
- Audio files are sent directly to OpenAI and not stored elsewhere

## Error Handling

Common issues and solutions:

1. **"OPENAI_API_KEY environment variable is required"**
   - Set the environment variable: `export OPENAI_API_KEY="your-key"`

2. **"OpenAI API error: Incorrect API key provided"**
   - Verify your API key is correct and active

3. **"OpenAI API error: You exceeded your current quota"**
   - Check your OpenAI account billing and usage limits

4. **Network/timeout errors**
   - Check internet connection
   - Verify firewall settings allow HTTPS to api.openai.com

## Development

The OpenAI integration consists of:

- `src/whisper/api_client.rs` - HTTP client for OpenAI API
- `src/whisper/mod.rs` - Updated transcriber with API support  
- `src/config/mod.rs` - Configuration extensions for API settings

## Contributing

Improvements welcome! Areas for contribution:
- Custom API endpoint support
- Additional audio format support
- Error recovery and retry logic
- Streaming transcription support
- Cost tracking and usage metrics