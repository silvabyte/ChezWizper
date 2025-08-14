# ChezWizper - OpenAI API Integration

This fork adds OpenAI Whisper API support to ChezWizper, allowing you to use OpenAI's cloud-based transcription service instead of local CLI tools.

## Features

- **OpenAI API Integration**: Use OpenAI's Whisper API for high-quality transcription
- **Backward Compatibility**: Still supports local whisper CLI (whisper.cpp, OpenAI Whisper CLI)
- **Flexible Configuration**: Easy switching between API and CLI modes
- **Environment-based Authentication**: Secure API key handling via environment variables

## Configuration

### Provider-Based Configuration (Recommended)

The new provider-based configuration allows for more flexibility and easier addition of new transcription services:

```toml
[whisper]
provider = "openai-api"      # Explicitly specify the provider
model = "whisper-1"          # Model name (provider-specific)
language = "en"              # Language code

# Optional settings
api_endpoint = "https://api.openai.com/v1/audio/transcriptions"  # Custom API endpoint
```

Available providers:
- `openai-api` - OpenAI Whisper API (requires OPENAI_API_KEY)
- `openai-cli` - OpenAI Whisper CLI tool
- `whisper-cpp` - whisper.cpp implementation
- Leave empty for auto-detection

### Auto-Detection

If no provider is specified, ChezWizper will auto-detect the best available provider:
1. OpenAI API (if `OPENAI_API_KEY` is set)
2. OpenAI Whisper CLI (if available) 
3. whisper.cpp (fallback)

### Provider Details

#### OpenAI Whisper CLI
```toml
[whisper]
provider = "openai-cli"
model = "base"
language = "en"
# command_path = "/path/to/whisper"  # Optional custom path
```

#### whisper.cpp
```toml
[whisper]
provider = "whisper-cpp"
model = "base"
language = "en"
# command_path = "/path/to/whisper"  # Optional custom path
# model_path = "/path/to/model.bin"  # Optional custom model path
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

Test API integration without audio recording:

```bash
OPENAI_API_KEY="your-key" cargo run --bin test_api
```

Test with configuration file:

```bash
OPENAI_API_KEY="your-key" cargo run --bin chezwizper -- --config example_config_api.toml
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