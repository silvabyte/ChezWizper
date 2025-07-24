#!/bin/bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_step() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}!${NC} $1"
}

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
CHEZWIZPER_DIR="$SCRIPT_DIR"

# Configuration variables
WHISPER_DIR="$HOME/.local/share/chezwizper/whisper"
CONFIG_DIR="$HOME/.config/chezwizper"
INSTALL_DIR="/usr/local/bin"
SOURCE_BACKUP_DIR="$HOME/.local/share/chezwizper/source"

# Check if running on Arch Linux
if [ ! -f /etc/arch-release ]; then
    print_error "This script is designed for Arch Linux. Detected: $(cat /etc/os-release | grep '^NAME=' | cut -d'=' -f2)"
    exit 1
fi

print_step "Starting ChezWizper installation for Arch Linux with Hyprland"

# Step 1: Install system dependencies
print_step "Installing system dependencies..."
if ! sudo pacman -S --needed --noconfirm rust wtype wl-clipboard alsa-lib curl cmake make gcc; then
    print_error "Failed to install system dependencies"
    exit 1
fi
print_success "System dependencies installed"

# Step 2: Clone and build optimized whisper.cpp
print_step "Setting up optimized whisper.cpp..."
mkdir -p "$(dirname "$WHISPER_DIR")"

if [ -d "$WHISPER_DIR" ]; then
    print_warning "Whisper directory already exists. Removing old installation..."
    rm -rf "$WHISPER_DIR"
fi

print_step "Cloning optimized whisper.cpp fork..."
if ! git clone https://github.com/matsilva/whisper.git "$WHISPER_DIR"; then
    print_error "Failed to clone whisper repository"
    exit 1
fi

cd "$WHISPER_DIR"
print_step "Building whisper-cli with large-v3-turbo model (this may take a while)..."
if ! ./build.sh; then
    print_error "Failed to build whisper"
    exit 1
fi
print_success "Whisper built successfully"

# Verify whisper-cli exists
if [ ! -f "$WHISPER_DIR/build/bin/whisper-cli" ]; then
    print_error "whisper-cli binary not found at expected location"
    exit 1
fi

# Step 3: Build ChezWizper
cd "$CHEZWIZPER_DIR"
print_step "Building ChezWizper..."
if ! cargo build --release; then
    print_error "Failed to build ChezWizper"
    exit 1
fi
print_success "ChezWizper built successfully"

# Step 4: Install ChezWizper binary
print_step "Installing ChezWizper binary..."
if ! sudo cp target/release/chezwizper "$INSTALL_DIR/"; then
    print_error "Failed to install ChezWizper binary"
    exit 1
fi
sudo chmod +x "$INSTALL_DIR/chezwizper"
print_success "ChezWizper installed to $INSTALL_DIR"

# Step 4.5: Keep source for updates and install update scripts
print_step "Setting up update mechanism..."
mkdir -p "$SOURCE_BACKUP_DIR"

# Copy source files for future updates
if ! cp -r "$CHEZWIZPER_DIR"/.git "$SOURCE_BACKUP_DIR/" 2>/dev/null; then
    print_warning "Git repository not found. Updates will require manual installation."
else
    # Copy essential files including update scripts
    cp -r "$CHEZWIZPER_DIR"/* "$SOURCE_BACKUP_DIR/" 2>/dev/null || true
    # Ensure update scripts are in backup location
    cp "$CHEZWIZPER_DIR/update-chezwizper.sh" "$SOURCE_BACKUP_DIR/" 2>/dev/null || true
    cp "$CHEZWIZPER_DIR/chezwizper-update" "$SOURCE_BACKUP_DIR/" 2>/dev/null || true
    print_success "Source backed up for future updates"
fi

# Make update scripts executable
chmod +x "$CHEZWIZPER_DIR/update-chezwizper.sh"
chmod +x "$CHEZWIZPER_DIR/chezwizper-update"

# Install system-wide update command
if [ -f "$CHEZWIZPER_DIR/chezwizper-update" ]; then
    # Update the wrapper with correct source directory
    sed -i "s|CHEZWIZPER_SOURCE_DIR:-.*}|CHEZWIZPER_SOURCE_DIR:-$SOURCE_BACKUP_DIR}|" "$CHEZWIZPER_DIR/chezwizper-update"
    
    if ! sudo cp "$CHEZWIZPER_DIR/chezwizper-update" "$INSTALL_DIR/"; then
        print_warning "Failed to install update wrapper"
    else
        sudo chmod +x "$INSTALL_DIR/chezwizper-update"
        print_success "Update command installed: chezwizper-update"
    fi
fi

# Step 5: Create configuration
print_step "Creating configuration..."
mkdir -p "$CONFIG_DIR"

cat > "$CONFIG_DIR/config.toml" << EOF
[audio]
device = "default"
sample_rate = 16000
channels = 1

[whisper]
model = "large-v3-turbo"
language = "en"
command_path = "$WHISPER_DIR/build/bin/whisper-cli"
model_path = "$WHISPER_DIR/models/ggml-large-v3-turbo-q5_1.bin"

[ui]
indicator_position = "top-right"
indicator_size = 20
show_notifications = true
layer_shell_anchor = "top | right"
layer_shell_margin = 10

[wayland]
input_method = "wtype"
use_hyprland_ipc = true

[behavior]
auto_paste = true
preserve_clipboard = false
delete_audio_files = true
EOF

print_success "Configuration created at $CONFIG_DIR/config.toml"

# Step 6: Create systemd user service
print_step "Creating systemd user service..."
mkdir -p ~/.config/systemd/user

cat > ~/.config/systemd/user/chezwizper.service << EOF
[Unit]
Description=ChezWizper Voice Transcription Service
After=graphical-session.target

[Service]
Type=simple
ExecStart=$INSTALL_DIR/chezwizper
Restart=always
RestartSec=5
Environment="RUST_LOG=info"
MemoryLimit=6G
CPUQuota=80%

[Install]
WantedBy=default.target
EOF

systemctl --user daemon-reload
print_success "Systemd service created"

# Step 7: Create toggle script
print_step "Creating toggle script..."
cat > "$CHEZWIZPER_DIR/chezwizper-toggle.sh" << 'EOF'
#!/bin/bash
curl -X POST http://127.0.0.1:3737/toggle
EOF
chmod +x "$CHEZWIZPER_DIR/chezwizper-toggle.sh"
print_success "Toggle script created"

# Step 8: Print Hyprland keybind instructions
print_step "Installation complete!"
echo
print_warning "Next steps:"
echo
echo "1. Start ChezWizper service:"
echo "   ${GREEN}systemctl --user enable --now chezwizper.service${NC}"
echo
echo "2. Add this keybind to your Hyprland config (~/.config/hypr/hyprland.conf):"
echo "   ${GREEN}bind = SUPER, R, exec, curl -X POST http://127.0.0.1:3737/toggle${NC}"
echo "   or for Omachy:"
echo "   ${GREEN}bind = SUPER, R, exec, \$terminal -e curl -X POST http://127.0.0.1:3737/toggle${NC}"
echo
echo "3. Check service status:"
echo "   ${GREEN}systemctl --user status chezwizper.service${NC}"
echo
echo "4. View logs if needed:"
echo "   ${GREEN}journalctl --user -u chezwizper.service -f${NC}"
echo
echo "5. Update ChezWizper anytime with:"
echo "   ${GREEN}chezwizper-update${NC}               # Update ChezWizper only"
echo "   ${GREEN}chezwizper-update --whisper${NC}     # Update both ChezWizper and whisper.cpp"
echo "   ${GREEN}chezwizper-update --check${NC}       # Check for available updates"
echo
print_success "ChezWizper is ready to use! Press Super+R to start recording."