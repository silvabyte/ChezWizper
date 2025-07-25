# ChezWizper Installation Guide

Complete installation instructions for different operating systems and environments.

## Quick Install (Recommended)

### Arch Linux + Omarchy

For users running Omarchy on Arch Linux, use the automated installer:

```bash
git clone https://github.com/silvabyte/ChezWizper.git
cd ChezWizper
make install
```

This installer:
- Installs all system dependencies (Rust, ydotool, wtype, wl-clipboard, etc.)
- Builds optimized Whisper.cpp with large-v3-turbo model
- Compiles and installs ChezWizper
- Creates systemd user service
- Sets up update mechanism
- Creates proper configuration

**Installation Options:**
```bash
make install              # Normal install with smart detection
make install -- --clean   # Fresh install from scratch
make install -- --skip-whisper  # Update only ChezWizper
make install -- --rebuild       # Force rebuild ChezWizper
```

**Post-installation steps:**
1. `make start` - Enable and start the service
2. Add to Hyprland config: `bind = SUPER, R, exec, curl -X POST http://127.0.0.1:3737/toggle`

## Manual Installation

### Prerequisites

All systems require:
- **Rust toolchain** (1.70+)
- **Whisper implementation** (see [Whisper Installation Options](#whisper-installation-options))
- **Text injection tool**: `ydotool` (recommended) or `wtype`
- **Clipboard tools**: `wl-clipboard` (Wayland) or `xclip`/`xsel` (X11)
- **Audio dependencies**: ALSA libraries
- **curl** for API communication

### System Dependencies

#### Arch Linux

```bash
sudo pacman -S rust ydotool wtype wl-clipboard alsa-lib curl cmake make gcc
```

#### Ubuntu/Debian

```bash
sudo apt update
sudo apt install cargo libasound2-dev wl-clipboard curl cmake build-essential

# Install ydotool (may need to compile from source)
sudo apt install ydotool || {
    git clone https://github.com/ReimuNotMoe/ydotool.git
    cd ydotool && mkdir build && cd build
    cmake .. && make -j$(nproc)
    sudo make install
}
```

#### Fedora

```bash
sudo dnf install rust cargo ydotool cmake gcc-c++ alsa-lib-devel curl openssl-devel
```

### Text Injection Setup

ChezWizper requires a text injection method. See the [Text Injection Setup Guide](./text-injection-setup.md) for detailed configuration.

**Quick setup for ydotool (recommended):**

```bash
# Enable ydotool user service
systemctl --user enable --now ydotool.service

# Add to shell profile
echo 'export YDOTOOL_SOCKET="/run/user/$(id -u)/.ydotool_socket"' >> ~/.bashrc
source ~/.bashrc
```

## Whisper Installation Options

ChezWizper supports multiple Whisper implementations:

### Option 1: Optimized whisper.cpp (Recommended)

Use the optimized fork with automatic build:

```bash
git clone https://github.com/matsilva/whisper.git ~/.local/share/chezwizper/whisper
cd ~/.local/share/chezwizper/whisper
./build.sh
```

This downloads and quantizes the large-v3-turbo model automatically.

### Option 2: OpenAI Whisper (Python)

```bash
pip install -U openai-whisper
```

### Option 3: Standard whisper.cpp

```bash
git clone https://github.com/ggerganov/whisper.cpp.git
cd whisper.cpp
make
./models/download-ggml-model.sh base
```

## Building ChezWizper

```bash
# Clone the repository
git clone https://github.com/silvabyte/ChezWizper.git
cd ChezWizper

# Build release version
cargo build --release

# Install binary
sudo cp target/release/chezwizper /usr/local/bin/
sudo chmod +x /usr/local/bin/chezwizper
```

## Configuration

Create the configuration directory and file:

```bash
mkdir -p ~/.config/chezwizper
```

ChezWizper will create a default config on first run, or you can create one manually:

### For Optimized Whisper.cpp

```toml
[audio]
device = "default"
sample_rate = 16000
channels = 1

[whisper]
model = "large-v3-turbo"
language = "en"
command_path = "/home/user/.local/share/chezwizper/whisper/build/bin/whisper-cli"
model_path = "/home/user/.local/share/chezwizper/whisper/models/ggml-large-v3-turbo-q5_1.bin"

[ui]
indicator_position = "top-right"
indicator_size = 20
show_notifications = true
layer_shell_anchor = "top | right"
layer_shell_margin = 10

[wayland]
input_method = "ydotool"
use_hyprland_ipc = true

[behavior]
auto_paste = true
preserve_clipboard = false
delete_audio_files = true
audio_feedback = true
```

### For OpenAI Whisper

```toml
[audio]
device = "default"
sample_rate = 16000
channels = 1

[whisper]
model = "base"
language = "en"
# command_path is auto-detected if whisper is in PATH

[ui]
indicator_position = "top-right"
indicator_size = 20
show_notifications = true

[wayland]
input_method = "ydotool"
use_hyprland_ipc = true

[behavior]
auto_paste = true
preserve_clipboard = false
delete_audio_files = true
audio_feedback = true
```

## Systemd Service Setup

Create a user service for automatic startup:

```bash
mkdir -p ~/.config/systemd/user
```

Create `~/.config/systemd/user/chezwizper.service`:

```ini
[Unit]
Description=ChezWizper Voice Transcription Service
After=graphical-session.target

[Service]
Type=simple
ExecStart=/usr/local/bin/chezwizper
Restart=always
RestartSec=5
Environment="RUST_LOG=info"
MemoryLimit=6G
CPUQuota=80%

[Install]
WantedBy=default.target
```

Enable and start the service:

```bash
systemctl --user daemon-reload
systemctl --user enable --now chezwizper.service
```

## Hyprland Integration

Add to your Hyprland config (`~/.config/hypr/hyprland.conf`):

```
bind = SUPER, R, exec, curl -X POST http://127.0.0.1:3737/toggle
```

For Omarchy users:
```
bind = SUPER, R, exec, $terminal -e curl -X POST http://127.0.0.1:3737/toggle
```

## GNOME + Wayland Setup

GNOME requires special setup due to security restrictions:

### 1. Install ydotool and setup daemon

```bash
sudo pacman -S ydotool  # or appropriate package manager

# Create user service
mkdir -p ~/.config/systemd/user
```

Create `~/.config/systemd/user/ydotoold.service`:

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

```bash
# Add environment variable
echo 'export YDOTOOL_SOCKET="/run/user/$(id -u)/.ydotool_socket"' >> ~/.bashrc
source ~/.bashrc

# Enable services
systemctl --user daemon-reload
systemctl --user enable --now ydotoold.service
systemctl --user enable --now chezwizper.service
```

### 2. Configure ChezWizper for GNOME

```toml
[wayland]
input_method = "ydotool"
use_hyprland_ipc = false
```

### 3. Create GNOME Keyboard Shortcut

1. Open GNOME Settings
2. Go to Keyboard → Keyboard Shortcuts → View and Customize Shortcuts
3. Go to Custom Shortcuts
4. Add new shortcut with command: `curl -X POST http://127.0.0.1:3737/toggle`
5. Set your preferred key combination (e.g., Super+R)

## Testing Installation

1. **Test service**: `systemctl --user status chezwizper.service`
2. **Test API**: `curl -X POST http://127.0.0.1:3737/toggle`
3. **Test recording**: Press your configured keybind
4. **Check logs**: `make logs`

## Troubleshooting

### Service fails to start
- Check logs: `make logs` or `journalctl --user -u chezwizper.service -e`
- Check status: `make status`
- Verify binary path: `which chezwizper`
- Test config: `chezwizper --verbose`

### Recording doesn't work
- Check microphone permissions
- Verify audio device: `arecord -l`
- Test with different device in config

### Text injection fails
- Verify ydotool service: `systemctl --user status ydotool.service`
- Check socket: `ls -la /run/user/$(id -u)/.ydotool_socket`
- See [Text Injection Setup](./text-injection-setup.md)

### Memory issues
- Large Whisper models need 3-5GB RAM
- Adjust `MemoryLimit` in service file
- Use smaller models if needed

### GNOME-specific issues
- Ensure ydotoold is running as user service (not system)
- Verify YDOTOOL_SOCKET environment variable
- wtype will NOT work on GNOME - use ydotool only

## Updating

The automated installer sets up an update mechanism:

```bash
chezwizper-update                    # Update ChezWizper only
chezwizper-update --whisper          # Update both ChezWizper and Whisper
chezwizper-update --check            # Check for available updates
chezwizper-update --force            # Force update even if up-to-date
```

## Uninstalling

```bash
# Stop and disable service
systemctl --user stop chezwizper.service
systemctl --user disable chezwizper.service

# Remove files
sudo rm /usr/local/bin/chezwizper
sudo rm /usr/local/bin/chezwizper-update
rm -rf ~/.config/chezwizper
rm -rf ~/.local/share/chezwizper
rm ~/.config/systemd/user/chezwizper.service

# Reload systemd
systemctl --user daemon-reload
```