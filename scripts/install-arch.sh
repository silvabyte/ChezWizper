#!/bin/bash
set -euo pipefail

# Parse command line arguments
CLEAN_INSTALL=false
SKIP_DEPS=false
SKIP_WHISPER=false
FORCE_REBUILD=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --clean)
            CLEAN_INSTALL=true
            shift
            ;;
        --skip-deps)
            SKIP_DEPS=true
            shift
            ;;
        --skip-whisper)
            SKIP_WHISPER=true
            shift
            ;;
        --rebuild)
            FORCE_REBUILD=true
            shift
            ;;
        --help)
            echo "ChezWizper Installation Script for Arch Linux"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --clean         Clean install (remove existing installations)"
            echo "  --skip-deps     Skip system dependency installation"
            echo "  --skip-whisper  Skip whisper.cpp build"
            echo "  --rebuild       Force rebuild ChezWizper even if binary exists"
            echo "  --help          Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                    # Normal install with smart detection"
            echo "  $0 --clean            # Fresh install from scratch"
            echo "  $0 --skip-whisper     # Update only ChezWizper"
            echo "  $0 --rebuild          # Force rebuild ChezWizper"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Colors for output - check if terminal supports colors
if [ -t 1 ] && [ -n "${TERM}" ] && [ "${TERM}" != "dumb" ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    NC='\033[0m' # No Color
else
    RED=''
    GREEN=''
    YELLOW=''
    BLUE=''
    NC=''
fi

# Print colored output
print_step() {
    printf "${BLUE}==>${NC} %s\n" "$1"
}

print_success() {
    printf "${GREEN}✓${NC} %s\n" "$1"
}

print_error() {
    printf "${RED}✗${NC} %s\n" "$1"
}

print_warning() {
    printf "${YELLOW}!${NC} %s\n" "$1"
}

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
# ChezWizper project root is one level up from scripts directory
CHEZWIZPER_DIR="$(dirname "$SCRIPT_DIR")"

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

print_step "Starting ChezWizper installation for Arch Linux"

# Clean install check
if [ "$CLEAN_INSTALL" = true ]; then
    print_warning "Performing clean install - removing existing installations"
    [ -d "$WHISPER_DIR" ] && rm -rf "$WHISPER_DIR"
    [ -f "$INSTALL_DIR/chezwizper" ] && sudo rm -f "$INSTALL_DIR/chezwizper"
    [ -d "$SOURCE_BACKUP_DIR" ] && rm -rf "$SOURCE_BACKUP_DIR"
    print_success "Clean install prepared"
fi

# Step 1: Install system dependencies
if [ "$SKIP_DEPS" = false ]; then
    print_step "Checking system dependencies..."
    MISSING_DEPS=()
    for dep in rust ydotool wtype wl-clipboard alsa-lib curl cmake make gcc; do
        if ! pacman -Qi "$dep" &>/dev/null; then
            MISSING_DEPS+=("$dep")
        fi
    done
    
    if [ ${#MISSING_DEPS[@]} -eq 0 ]; then
        print_success "All system dependencies already installed"
    else
        print_step "Installing missing dependencies: ${MISSING_DEPS[*]}"
        if ! sudo pacman -S --needed --noconfirm "${MISSING_DEPS[@]}"; then
            print_error "Failed to install system dependencies"
            exit 1
        fi
        print_success "System dependencies installed"
    fi
else
    print_warning "Skipping system dependency check (--skip-deps)"
fi

# Step 1.5: Setup ydotool service
print_step "Checking ydotool service..."
if systemctl --user is-active --quiet ydotool.service; then
    print_success "ydotool service is already running"
else
    print_step "Setting up ydotool service..."
    if ! systemctl --user enable --now ydotool.service; then
        print_warning "Failed to enable ydotool service - you may need to start it manually"
        print_warning "Run: systemctl --user enable --now ydotool.service"
    else
        print_success "ydotool service enabled and started"
    fi
fi

# Step 2: Clone and build optimized whisper.cpp
if [ "$SKIP_WHISPER" = false ]; then
    mkdir -p "$(dirname "$WHISPER_DIR")"
    
    # Check if whisper is already installed and working
    if [ -d "$WHISPER_DIR" ] && [ -f "$WHISPER_DIR/build/bin/whisper-cli" ] && [ -f "$WHISPER_DIR/models/ggml-large-v3-turbo-q5_1.bin" ]; then
        print_success "Whisper already installed with model"
        print_warning "Use --clean to reinstall whisper from scratch"
    else
        print_step "Setting up optimized whisper.cpp..."
        
        if [ -d "$WHISPER_DIR" ]; then
            print_warning "Incomplete whisper installation found. Removing..."
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
    fi
else
    print_warning "Skipping whisper installation (--skip-whisper)"
fi

# Step 3: Build ChezWizper
cd "$CHEZWIZPER_DIR"

# Check if we need to rebuild
BUILD_NEEDED=false
if [ ! -f "target/release/chezwizper" ]; then
    BUILD_NEEDED=true
    print_step "ChezWizper binary not found, building..."
elif [ "$FORCE_REBUILD" = true ]; then
    BUILD_NEEDED=true
    print_step "Force rebuild requested, building ChezWizper..."
else
    # Check if source files are newer than binary
    if [ -n "$(find src -newer target/release/chezwizper -print -quit 2>/dev/null)" ]; then
        BUILD_NEEDED=true
        print_step "Source files changed, rebuilding ChezWizper..."
    else
        print_success "ChezWizper binary is up to date"
    fi
fi

if [ "$BUILD_NEEDED" = true ]; then
    if ! cargo build --release; then
        print_error "Failed to build ChezWizper"
        exit 1
    fi
    print_success "ChezWizper built successfully"
fi

# Step 4: Install ChezWizper binary
print_step "Installing ChezWizper binary..."

# Check if service is running and stop it to avoid "Text file busy" error
SERVICE_WAS_RUNNING=false
if systemctl --user is-active --quiet chezwizper.service; then
    SERVICE_WAS_RUNNING=true
    print_warning "ChezWizper service is running, stopping it temporarily..."
    systemctl --user stop chezwizper.service
    sleep 1  # Give it a moment to fully stop
fi

# Try to copy the binary
if ! sudo cp target/release/chezwizper "$INSTALL_DIR/"; then
    print_error "Failed to install ChezWizper binary"
    # If service was running, start it again even on failure
    if [ "$SERVICE_WAS_RUNNING" = true ]; then
        systemctl --user start chezwizper.service
    fi
    exit 1
fi
sudo chmod +x "$INSTALL_DIR/chezwizper"
print_success "ChezWizper installed to $INSTALL_DIR"

# If service was running before, start it again
if [ "$SERVICE_WAS_RUNNING" = true ]; then
    print_step "Restarting ChezWizper service..."
    systemctl --user start chezwizper.service
    print_success "ChezWizper service restarted"
fi

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
    cp "$CHEZWIZPER_DIR/scripts/update-chezwizper.sh" "$SOURCE_BACKUP_DIR/" 2>/dev/null || true
    cp "$CHEZWIZPER_DIR/scripts/chezwizper-update" "$SOURCE_BACKUP_DIR/" 2>/dev/null || true
    print_success "Source backed up for future updates"
fi

# Make update scripts executable
chmod +x "$CHEZWIZPER_DIR/scripts/update-chezwizper.sh"
chmod +x "$CHEZWIZPER_DIR/scripts/chezwizper-update"

# Install system-wide update command
if [ -f "$CHEZWIZPER_DIR/scripts/chezwizper-update" ]; then
    # Update the wrapper with correct source directory
    sed -i "s|CHEZWIZPER_SOURCE_DIR:-.*}|CHEZWIZPER_SOURCE_DIR:-$SOURCE_BACKUP_DIR}|" "$CHEZWIZPER_DIR/scripts/chezwizper-update"
    
    if ! sudo cp "$CHEZWIZPER_DIR/scripts/chezwizper-update" "$INSTALL_DIR/"; then
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
input_method = "ydotool"
use_hyprland_ipc = true

[behavior]
auto_paste = true
preserve_clipboard = false
delete_audio_files = true
audio_feedback = true
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
WorkingDirectory=$CHEZWIZPER_DIR
ExecStart=$INSTALL_DIR/chezwizper
Restart=always
RestartSec=5
Environment="RUST_LOG=info"
Environment="HOME=$HOME"
Environment="PATH=/usr/local/bin:/usr/bin:/bin"
# Memory settings - adjust based on your system
MemoryMax=8G
MemorySwapMax=12G
# CPU settings - let whisper use multiple cores
# Remove CPUQuota to allow full CPU usage
# Set thread count for whisper (adjust based on your CPU)
Environment="OMP_NUM_THREADS=$(nproc)"

[Install]
WantedBy=default.target
EOF

systemctl --user daemon-reload
print_success "Systemd service created"

# Step 7: Print Hyprland keybind instructions
print_step "Installation complete!"
echo
print_warning "Next steps:"
echo
printf "1. Start ChezWizper service:\n"
printf "   ${GREEN}make start${NC}\n"
echo
printf "2. Add this keybind to your Hyprland config (~/.config/hypr/hyprland.conf):\n"
printf "   ${GREEN}bind = SUPER, R, exec, curl -X POST http://127.0.0.1:3737/toggle${NC}\n"
printf "   or for Omachy:\n"
printf "   ${GREEN}bind = SUPER, R, exec, \$terminal -e curl -X POST http://127.0.0.1:3737/toggle${NC}\n"
echo
printf "3. Check service status:\n"
printf "   ${GREEN}make status${NC}\n"
echo
printf "4. View logs if needed:\n"
printf "   ${GREEN}make logs${NC}\n"
echo
printf "5. Common development commands:\n"
printf "   ${GREEN}make help${NC}      # Show all available commands\n"
printf "   ${GREEN}make restart${NC}   # Restart the service\n"
printf "   ${GREEN}make build${NC}     # Rebuild ChezWizper\n"
echo
printf "6. Update ChezWizper anytime with:\n"
printf "   ${GREEN}make update${NC}     # Update ChezWizper only\n"
printf "   ${GREEN}make update-all${NC} # Update both ChezWizper and whisper.cpp\n"
printf "   Note: If 'chezwizper-update' command not found, run: ${GREEN}hash -r${NC}\n"
echo
print_success "ChezWizper is ready to use! Press Super+R to start recording."