# ChezWizper

Voice transcription tool for Wayland/Hyprland keybinds, records audio, transcribes it using Whisper CLI, and inserts the transcribed text into the currently focused input field.

📚 **[View Documentation](./docs/index.md)** - Detailed configuration guides and more

## Features

- 🎤 use hyprland keybind to toggle recording (I use Super+R)
- 🔴 Visual indicators for recording status via notifications
- 🎯 Automatic text injection into focused applications
- 📋 Clipboard integration with fallback support
- ⚡ Optimized for Wayland/Hyprland environments
- 🔧 Configurable settings via TOML

## Prerequisites

- Rust toolchain (1.70+)
- Whisper CLI (see [Whisper Installation Options](#whisper-installation-options) below)
- Text injection tool: `wtype` (preferred) or `ydotool`
- Wayland clipboard: `wl-clipboard`
- Audio dependencies: `libasound2-dev` (ALSA)
- `curl` for API communication

### Install system dependencies on Arch Linux

```bash
sudo pacman -S rust wtype wl-clipboard alsa-lib curl
```

### Install system dependencies on Ubuntu/Debian

\*\* Below is untested, if someone could confirm, that would be great!

```bash
sudo apt install cargo libasound2-dev wl-clipboard curl
# Install wtype from source or alternative repositories
```

### Install system dependencies on Fedora 42+

```bash
sudo dnf install ydotool cmake gcc-c++ libevdev-devel libuinput-devel openssl-devel
```

#### Gnome and Wayland compatibility

On GNOME with Wayland, ydotoold (the ydotool daemon) must be run as a user service because Wayland enforces strict security boundaries that prevent system-level services (running as root) from interacting with user input devices in a desktop session. Running ydotoold as a user service ensures it has access to your session’s input devices and environment, allowing tools like ChezWizper to inject text reliably and securely into applications. This approach also avoids permission issues and aligns with modern desktop security practices, making native text injection possible without requiring root privileges.

The wtype tool does not work for text injection on GNOME with Wayland because GNOME’s compositor (Mutter) does not implement the “virtual keyboard protocol” required by wtype to simulate keyboard input. This protocol is only supported by some other Wayland compositors (such as Sway or Hyprland). As a result, attempts to use wtype on GNOME/Wayland will fail with errors like “Compositor does not support the virtual keyboard protocol.” For this reason, using ydotool (with its daemon running as a user service) is the recommended and reliable method for text injection on GNOME + Wayland.

So, let's configure `ydotool` properly. Create the user service file:

```sh
mkdir -p ~/.config/systemd/user
nano ~/.config/systemd/user/ydotoold.service
```

Paste:

```ini
[Unit]
Description=ydotoold user daemon
After=graphical-session.target

[Service]
Type=simple
ExecStart=/usr/bin/ydotoold -P 660

[Install]
WantedBy=default.target
```

*(Adjust the path to `ydotoold` if needed, e.g., `/usr/local/bin/ydotoold` if you built from source)*
Run `which ydotool` to check the path on your system.

Add this to your `.bashrc` or `.zshrc`:

```bash
export YDOTOOL_SOCKET="/run/user/$(id -u)/.ydotool_socket"
```

And source it if you don't want to restart you session:

```sh
source ~/.bashrc
source ~/.zshrc
```

*Make sure* you configure ChezWizper to use ydotool:

```toml
[wayland]
input_method = "ydotool"
use_hyprland_ipc = false
```

Then Enable and Start the User Service

```sh
systemctl --user daemon-reload
systemctl --user enable ydotoold.service
systemctl --user start ydotoold.service
systemctl --user status ydotoold.service
```

Finally, create a shortcut to toggle ChezWizper:

Gnome Settings > Keyboard > Keyboard Shortcuts > View and Customize Shortcuts:

Go to Custom Shortcuts, and add a new one with command:

```bash
bash /home/user/path-where-you-cloned-ChezWizper/chezwizper-toggle.sh
```

And you're good to go!

## Whisper Installation Options

ChezWizper supports multiple Whisper implementations. Choose one:

### Option 1: Optimized whisper.cpp (Recommended)

This fork provides an optimized build script that handles everything automatically:

```bash
# Clone the optimized whisper.cpp fork
git clone https://github.com/matsilva/whisper.git
cd whisper

# Run the universal build script
# This will download models, quantize them, and compile whisper-cli
./build.sh

# The whisper-cli binary will be at: build/bin/whisper-cli
```

Then configure ChezWizper to use it:

```toml
[whisper]
command_path = "/path/to/whisper/build/bin/whisper-cli"
model_path = "/path/to/whisper/models/ggml-large-v3-turbo-q5_1.bin"
```

See [Configuration](https://github.com/silvabyte/ChezWizper?tab=readme-ov-file#configuration)

### Option 2: OpenAI Whisper (Python)

Install the official [OpenAI Whisper:](https://github.com/openai/whisper)

```bash
# Install via pip
pip install -U openai-whisper

# Or with conda
conda install -c conda-forge openai-whisper
```

Then configure ChezWizper:

```toml
[whisper]
# OpenAI whisper is usually in PATH after pip install
command_path = "whisper"  # or leave empty to search PATH
model = "base"  # or small, medium, large, large-v2, large-v3
```

See [Configuration](https://github.com/silvabyte/ChezWizper?tab=readme-ov-file#configuration)

### Option 3: Standard whisper.cpp

```bash
# Clone standard whisper.cpp
git clone https://github.com/ggerganov/whisper.cpp.git
cd whisper.cpp

# Build
make

# Download models
./models/download-ggml-model.sh base

# Configure ChezWizper
```

```toml
[whisper]
command_path = "/path/to/whisper.cpp/main"
model_path = "/path/to/whisper.cpp/models/ggml-base.bin"
```

See [Configuration](https://github.com/silvabyte/ChezWizper?tab=readme-ov-file#configuration)

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

If you are using Omachy installation

```
bind = SUPER, R, exec, $terminal -e curl -X POST http:/127.0.0.1:3737/toggle
```

Or simply

```
bind = SUPER, R, exec, -e curl -X POST http://127.0.0.1:3737/toggle
```

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

## Running as a Service

To run ChezWizper automatically on startup, you can create a systemd user service:

### 1. Create the service file

```bash
mkdir -p ~/.config/systemd/user
nano ~/.config/systemd/user/chezwizper.service
```

### 2. Add the following content

```ini
[Unit]
Description=ChezWizper Voice Transcription Service
After=graphical-session.target

[Service]
Type=simple
ExecStart=/usr/local/bin/chezwizper
Restart=always
RestartSec=5

# Environment variables (optional)
Environment="RUST_LOG=info"

# Resource limits (optional)
# Note: Whisper models require significant memory (3-5GB for larger models)
MemoryLimit=6G
CPUQuota=80%

[Install]
WantedBy=default.target
```

### 3. Enable and start the service

```bash
# Reload systemd to recognize the new service
systemctl --user daemon-reload

# Enable the service to start on boot
systemctl --user enable chezwizper.service

# Start the service immediately
systemctl --user start chezwizper.service

# Check service status
systemctl --user status chezwizper.service
```

### 4. View logs

```bash
# View recent logs
journalctl --user -u chezwizper.service -n 50

# Follow logs in real-time
journalctl --user -u chezwizper.service -f
```

### Service Management

```bash
# Stop the service
systemctl --user stop chezwizper.service

# Restart the service
systemctl --user restart chezwizper.service

# Disable auto-start
systemctl --user disable chezwizper.service
```

### Alternative: Using chezwizper from source directory

If you prefer to run from your build directory instead of installing system-wide:

```ini
[Service]
Type=simple
WorkingDirectory=/home/user/code/whispy
ExecStart=/home/user/code/whispy/target/release/chezwizper
# ... rest of the service file
```

### Troubleshooting

If the service fails to start:

1. Check logs: `journalctl --user -u chezwizper.service -e`
2. Ensure the binary path is correct: `which chezwizper`
3. Verify your config file is valid: `chezwizper --verbose`
4. Make sure audio permissions are correct: `groups | grep audio`
5. For Wayland access issues, ensure `WAYLAND_DISPLAY` is set in your environment

**Memory Issues:**

- If the service crashes during transcription, check memory usage
- Large Whisper models (large-v3) require 3-5GB of RAM
- Adjust `MemoryLimit` in the service file if needed
- Monitor with: `systemctl --user status chezwizper.service`

## Configuration

Configuration file is located at `~/.config/chezwizper/config.toml`:

### Example for Optimized whisper.cpp (Recommended)

```toml
[audio]
device = "default"
sample_rate = 16000
channels = 1

[whisper]
model = "large-v3-turbo"  # Model name for reference
language = "en"
command_path = "/home/user/whisper/build/bin/whisper-cli"
model_path = "/home/user/whisper/models/ggml-large-v3-turbo-q5_1.bin"
```

### Example for OpenAI Whisper

```toml
[audio]
device = "default"
sample_rate = 16000
channels = 1

[whisper]
model = "base"  # Options: tiny, base, small, medium, large, large-v2, large-v3
language = "en"
# command_path = "whisper"  # Usually in PATH after pip install
```

### Full Configuration Example

```toml
[audio]
device = "default"
sample_rate = 16000
channels = 1

[whisper]
model = "base"
language = "en"
# Optional: specify custom whisper CLI path
# command_path = "/path/to/whisper-cli"
# Optional: specify custom model file path (whisper.cpp only)
# model_path = "/path/to/ggml-model.bin"

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

## License

MIT
