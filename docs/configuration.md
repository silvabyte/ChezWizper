# ChezWizper Configuration Guide

ChezWizper is configured via a single TOML file located at `~/.config/chezwizper/config.toml`. This guide covers everything you need to know about configuring ChezWizper for your needs.

## Quick Start

The minimal configuration to get started:

```toml
[whisper]
# Auto-detection (recommended) - ChezWizper will automatically choose the best provider
language = "en"

# For OpenAI API access, add your key to config:
# provider = "openai-api"
# api_key = "sk-your-api-key-here"
```

ChezWizper will create a default configuration file on first run if none exists.

## Complete Configuration Example

Here's a full configuration file with all available options:

```toml
[audio]
device = "default"              # Audio input device name
sample_rate = 16000             # Sample rate in Hz (8000, 16000, 44100, 48000)
channels = 1                    # Number of audio channels (1 = mono, 2 = stereo)

[whisper]
provider = "openai-api"         # Transcription provider (see Providers section)
api_key = "sk-your-api-key-here" # API key for API providers
model = "whisper-1"             # Model name (provider-specific)
language = "en"                 # Language code (ISO 639-1)
command_path = "/usr/bin/whisper"  # Custom CLI tool path (optional)
model_path = "/path/to/model.bin"  # Custom model file path (optional)
api_endpoint = "https://api.openai.com/v1/audio/transcriptions"  # Custom API endpoint (optional)

[ui]
indicator_position = "top-right"  # Visual indicator position
indicator_size = 20             # Indicator size in pixels
show_notifications = true       # Show desktop notifications
layer_shell_anchor = "top | right"  # Wayland layer shell anchor
layer_shell_margin = 10         # Margin from screen edge in pixels

[wayland]
input_method = "wtype"          # Text injection method
use_hyprland_ipc = true         # Use Hyprland IPC for better integration

[behavior]
auto_paste = true               # Automatically paste transcribed text
preserve_clipboard = false      # Keep clipboard content after pasting
delete_audio_files = true       # Delete temporary audio files after processing
audio_feedback = true           # Play audio feedback sounds
```

## Configuration Sections

### [audio] - Audio Input Settings

Controls how ChezWizper captures audio from your microphone.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `device` | string | `"default"` | Audio input device name. Use `"default"` for system default, or specific device name |
| `sample_rate` | number | `16000` | Audio sample rate in Hz. Common values: 8000, 16000, 44100, 48000 |
| `channels` | number | `1` | Number of audio channels. 1 = mono (recommended), 2 = stereo |

**Tips:**
- 16000 Hz sample rate provides the best balance of quality and performance for speech
- Mono (1 channel) is sufficient for speech recognition and reduces file size
- To list available audio devices: `arecord -l` (on Linux)

### [whisper] - Transcription Settings

Configures speech-to-text transcription providers and models.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `provider` | string | auto-detect | Transcription provider: `"openai-api"`, `"openai-cli"`, `"whisper-cpp"`, or omit for auto-detection |
| `api_key` | string | none | API key for API-based providers (required for openai-api) |
| `model` | string | `"base"` | Model name (provider-specific, see Providers section) |
| `language` | string | `"en"` | Language code (ISO 639-1 format) |
| `command_path` | string | auto-detect | Custom path to whisper CLI tool (optional) |
| `model_path` | string | auto-detect | Custom path to model file (whisper.cpp only) |
| `api_endpoint` | string | OpenAI API | Custom API endpoint URL (API providers only) |

#### Providers

ChezWizper supports multiple transcription providers:

**OpenAI API** (`provider = "openai-api"`)
- **Best for:** High accuracy, no local setup
- **Requirements:** API key in config, internet connection  
- **Models:** `"whisper-1"` (only available model)
- **Cost:** ~$0.006 per minute of audio

**OpenAI Whisper CLI** (`provider = "openai-cli"`)
- **Best for:** Local processing, no API costs, privacy
- **Requirements:** `pip install openai-whisper`
- **Models:** `"tiny"`, `"base"`, `"small"`, `"medium"`, `"large-v3"`
- **Cost:** Free (local processing)

**whisper.cpp** (`provider = "whisper-cpp"`)
- **Best for:** Resource-constrained systems, CPU-only inference
- **Requirements:** Build from source or install via package manager
- **Models:** `"tiny"`, `"base"`, `"small"`, `"medium"`, `"large"`
- **Status:** Experimental
- **Cost:** Free (local processing)

**Auto-Detection** (omit `provider`)
- ChezWizper automatically selects the best available provider:
  1. OpenAI Whisper CLI (if installed)
  2. whisper.cpp (fallback)
- Note: API providers require explicit configuration with api_key

#### Language Codes

Common language codes (ISO 639-1):

| Code | Language | Code | Language | Code | Language |
|------|----------|------|----------|------|----------|
| `en` | English | `es` | Spanish | `fr` | French |
| `de` | German | `it` | Italian | `pt` | Portuguese |
| `ru` | Russian | `zh` | Chinese | `ja` | Japanese |
| `ko` | Korean | `ar` | Arabic | `auto` | Auto-detect* |

*Auto-detection only works with OpenAI API

For the complete list, see [ISO 639-1 codes](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).

### [ui] - User Interface Settings

Controls visual indicators and desktop notifications.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `indicator_position` | string | `"top-right"` | Screen position: `"top-left"`, `"top-right"`, `"bottom-left"`, `"bottom-right"` |
| `indicator_size` | number | `20` | Visual indicator size in pixels |
| `show_notifications` | bool | `true` | Show desktop notifications for transcription results |
| `layer_shell_anchor` | string | `"top \| right"` | Wayland layer shell anchor points |
| `layer_shell_margin` | number | `10` | Distance from screen edge in pixels |

### [wayland] - Wayland Integration

Configures integration with Wayland desktop environments.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `input_method` | string | `"wtype"` | Text injection method: `"wtype"`, `"clipboard"` |
| `use_hyprland_ipc` | bool | `true` | Use Hyprland IPC for better window management integration |

**Text Injection Methods:**
- `"wtype"` - Direct text typing (fast, works in most apps)
- `"clipboard"` - Via clipboard (universal compatibility, slower)

### [behavior] - Application Behavior

Controls how ChezWizper handles transcribed text and temporary files.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `auto_paste` | bool | `true` | Automatically paste/type transcribed text |
| `preserve_clipboard` | bool | `false` | Keep existing clipboard content when using clipboard injection |
| `delete_audio_files` | bool | `true` | Delete temporary audio recordings after processing |
| `audio_feedback` | bool | `true` | Play audio feedback sounds (start/stop recording) |

## Configuration File Location

ChezWizper looks for its configuration file at:

- **Linux:** `~/.config/chezwizper/config.toml`
- **macOS:** `~/Library/Application Support/chezwizper/config.toml`
- **Windows:** `%APPDATA%\chezwizper\config.toml`

## Environment Variables

ChezWizper respects these environment variables:

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Logging level (`error`, `warn`, `info`, `debug`, `trace`) |

## Common Configuration Scenarios

### For OpenAI API Users
```toml
[whisper]
provider = "openai-api"
api_key = "sk-your-api-key-here"  # Your OpenAI API key
model = "whisper-1"
language = "en"  # or "auto" for automatic detection
```

### For Local Processing (Privacy-Focused)
```toml
[whisper]
provider = "openai-cli"
model = "small"  # Good balance of speed and accuracy
language = "en"

# No API key needed - everything runs locally
```

### For Multiple Languages
```toml
[whisper]
provider = "openai-api"
model = "whisper-1" 
language = "auto"  # Automatically detect language

# Or set a specific language code like "es" for Spanish
```

### For Low-Resource Systems
```toml
[audio]
sample_rate = 16000  # Lower sample rate
channels = 1         # Mono audio

[whisper]
provider = "openai-cli"
model = "tiny"       # Smallest, fastest model
language = "en"

[behavior]
delete_audio_files = true  # Clean up temp files
```

### For High Accuracy Transcription
```toml
[audio]
sample_rate = 48000  # Higher quality audio
channels = 1

[whisper]
provider = "openai-cli"
model = "large-v3"   # Most accurate model
language = "en"

[behavior]
audio_feedback = false  # Reduce distractions
```

## Migrating from Earlier Versions

If you're upgrading from an earlier version that used `use_api = true/false`, update your config:

**Old format:**
```toml
[whisper]
use_api = true
model = "whisper-1"
```

**New format:**
```toml
[whisper]
provider = "openai-api"
model = "whisper-1"
```

**Migration mapping:**
- `use_api = true` → `provider = "openai-api"`
- `use_api = false` → `provider = "openai-cli"` (or `"whisper-cpp"`)

## Troubleshooting Configuration

### Config File Issues

**"Failed to parse config file"**
- Check TOML syntax with an online validator
- Ensure strings are quoted: `language = "en"` not `language = en`
- Verify boolean values: `true`/`false` not `"true"`/`"false"`

**"Config file not found"**
- ChezWizper will create a default config on first run
- Manually create the config directory: `mkdir -p ~/.config/chezwizper`

### Provider Issues

**"No transcription provider available"**
- Install a provider: `pip install openai-whisper` 
- Or set OpenAI API key: `export OPENAI_API_KEY="sk-..."`
- Check provider installation: `whisper --help`

**"OPENAI_API_KEY environment variable required"**
- Set your API key: `export OPENAI_API_KEY="sk-your-key"`
- Get an API key from https://platform.openai.com/api-keys

### Audio Issues

**"No audio input detected"**
- Check `device = "default"` in config
- List devices: `arecord -l`
- Test audio: `arecord -f cd test.wav` (Ctrl+C to stop, `aplay test.wav` to playback)

### Validation

Test your configuration:
```bash
# Start ChezWizper with verbose logging
RUST_LOG=debug chezwizper

# Look for these log messages:
# "Loaded config from ..."
# "Using [Provider] for transcription"
```

For more troubleshooting, see the [Whisper Transcription Setup](./whisper-transcription-setup.md) guide.