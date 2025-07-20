# ChezWizper

Voice transcription tool for Wayland/Hyprland that listens for keyboard shortcuts, records audio, transcribes it using Whisper CLI, and inserts the transcribed text into the currently focused input field.

## Features

- 🎤 Global hotkey to toggle recording (Ctrl+Shift+A by default)
- 🔴 Visual indicators for recording status via notifications
- 🎯 Automatic text injection into focused applications
- 📋 Clipboard integration with fallback support
- ⚡ Optimized for Wayland/Hyprland environments
- 🔧 Configurable settings via TOML

## Prerequisites

- Rust toolchain (1.70+)
- Whisper CLI (`whisper` or `whisper-cpp`)
- Text injection tool: `wtype` (preferred) or `ydotool`
- Wayland clipboard: `wl-clipboard`
- Audio dependencies: `libasound2-dev` (ALSA)
- `curl` for API communication

### Install dependencies on Arch Linux:
```bash
sudo pacman -S rust whisper-cpp wtype wl-clipboard alsa-lib curl
```

### Install dependencies on Ubuntu/Debian:
```bash
sudo apt install cargo libasound2-dev wl-clipboard curl
# Install whisper and wtype from source or alternative repositories
```

## Building

```bash
cargo build --release
```

## Installation

```bash
# Build the project
cargo build --release

# Copy binary to PATH
sudo cp target/release/chezwizper /usr/local/bin/

# Create config directory
mkdir -p ~/.config/chezwizper

# Run to generate default config
chezwizper
```

## Usage

### Setup for Hyprland

1. Start ChezWizper:
```bash
chezwizper
```

2. Add keybind to your Hyprland config (`~/.config/hypr/hyprland.conf`):
```
bind = CTRL SHIFT, A, exec, curl -X POST http://127.0.0.1:3737/toggle
```

Or use the provided script:
```bash
./chezwizper-keybind.sh
```

3. Press `Ctrl+Shift+A` to start recording
4. Speak your text
5. Press `Ctrl+Shift+A` again to stop recording
6. Wait for transcription
7. Text will be automatically inserted at cursor position

### Manual Control
You can also control ChezWizper manually:

**Toggle recording:**
```bash
./chezwizper-toggle.sh
# Or directly:
curl -X POST http://127.0.0.1:3737/toggle
```

**Check status:**
```bash
./chezwizper-status.sh
# Or directly:
curl http://127.0.0.1:3737/status
```

### API Endpoints
- `GET /` - Service info
- `POST /toggle` - Toggle recording
- `GET /status` - Get recording status

### Command Line Options

```bash
chezwizper --help
chezwizper --verbose  # Enable debug logging
```

## Configuration

Configuration file is located at `~/.config/chezwizper/config.toml`:

```toml
[hotkeys]
toggle_recording = "Ctrl+Shift+A"

[audio]
device = "default"
sample_rate = 16000
channels = 1

[whisper]
model = "base"
language = "en"
command_path = "/usr/local/bin/whisper"

[ui]
indicator_position = "top-right"
indicator_size = 20
show_notifications = true
layer_shell_anchor = "top | right"
layer_shell_margin = 10

[wayland]
input_method = "wtype"  # or "ydotool"
use_hyprland_ipc = true

[behavior]
auto_paste = true
preserve_clipboard = false
delete_audio_files = true
```

## Troubleshooting

### Recording doesn't start
- Check if you have microphone permissions
- Verify audio device with `arecord -l`
- Try changing the audio device in config

### Text injection fails
- Ensure `wtype` or `ydotool` is installed
- For `ydotool`, make sure `ydotoold` daemon is running
- Try disabling `auto_paste` and use clipboard instead

### Whisper not found
- Install whisper: `pip install openai-whisper` or build whisper.cpp
- Set the correct path in `whisper.command_path` config

### Keyboard shortcuts don't work
- Run with `sudo` if needed (for global hotkeys)
- Check if another application is using the same hotkey

## License

MIT