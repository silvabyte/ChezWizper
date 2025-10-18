#!/bin/bash
set -euo pipefail

# Parse command line arguments
KEEP_CONFIG=false
KEEP_WHISPER=false
FORCE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --keep-config)
            KEEP_CONFIG=true
            shift
            ;;
        --keep-whisper)
            KEEP_WHISPER=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --help)
            echo "ChezWizper Uninstallation Script"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --keep-config   Keep configuration files in ~/.config/chezwizper"
            echo "  --keep-whisper  Keep whisper.cpp installation"
            echo "  --force         Skip confirmation prompts"
            echo "  --help          Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                    # Full uninstall with confirmation"
            echo "  $0 --keep-config      # Uninstall but keep configuration"
            echo "  $0 --keep-whisper     # Uninstall but keep whisper.cpp"
            echo "  $0 --force            # Uninstall without prompts"
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

# Configuration variables
WHISPER_DIR="$HOME/.local/share/chezwizper/whisper"
CONFIG_DIR="$HOME/.config/chezwizper"
INSTALL_DIR="/usr/local/bin"
SOURCE_BACKUP_DIR="$HOME/.local/share/chezwizper/source"
DATA_DIR="$HOME/.local/share/chezwizper"
SERVICE_FILE="$HOME/.config/systemd/user/chezwizper.service"

# Show what will be removed
print_warning "ChezWizper Uninstallation"
echo ""
echo "The following will be removed:"
echo ""

# Check what exists and will be removed
ITEMS_TO_REMOVE=()

if [ -f "$INSTALL_DIR/chezwizper" ]; then
    echo "  • ChezWizper binary: $INSTALL_DIR/chezwizper"
    ITEMS_TO_REMOVE+=("binary")
fi

if [ -f "$INSTALL_DIR/chezwizper-update" ]; then
    echo "  • Update script: $INSTALL_DIR/chezwizper-update"
    ITEMS_TO_REMOVE+=("update")
fi

if [ -f "$SERVICE_FILE" ]; then
    echo "  • Systemd service: $SERVICE_FILE"
    ITEMS_TO_REMOVE+=("service")
fi

if [ -d "$CONFIG_DIR" ] && [ "$KEEP_CONFIG" = false ]; then
    echo "  • Configuration: $CONFIG_DIR"
    ITEMS_TO_REMOVE+=("config")
elif [ -d "$CONFIG_DIR" ] && [ "$KEEP_CONFIG" = true ]; then
    print_warning "  • Configuration will be kept: $CONFIG_DIR"
fi

if [ -d "$WHISPER_DIR" ] && [ "$KEEP_WHISPER" = false ]; then
    echo "  • Whisper installation: $WHISPER_DIR"
    ITEMS_TO_REMOVE+=("whisper")
elif [ -d "$WHISPER_DIR" ] && [ "$KEEP_WHISPER" = true ]; then
    print_warning "  • Whisper will be kept: $WHISPER_DIR"
fi

if [ -d "$SOURCE_BACKUP_DIR" ]; then
    echo "  • Source backup: $SOURCE_BACKUP_DIR"
    ITEMS_TO_REMOVE+=("source")
fi

# Check for temp files
TEMP_FILES=$(find /tmp -maxdepth 1 -name "chezwizper_*.wav" 2>/dev/null || true)
if [ -n "$TEMP_FILES" ]; then
    echo "  • Temporary audio files in /tmp"
    ITEMS_TO_REMOVE+=("temp")
fi

echo ""

# Check if anything needs to be removed
if [ ${#ITEMS_TO_REMOVE[@]} -eq 0 ]; then
    print_warning "ChezWizper does not appear to be installed"
    exit 0
fi

# Confirmation prompt
if [ "$FORCE" = false ]; then
    printf "${YELLOW}Do you want to continue? [y/N]${NC} "
    read -r response
    if [[ ! "$response" =~ ^[Yy]$ ]]; then
        print_warning "Uninstallation cancelled"
        exit 0
    fi
fi

echo ""
print_step "Starting uninstallation..."

# Step 1: Stop and disable service
if systemctl --user is-active --quiet chezwizper.service 2>/dev/null; then
    print_step "Stopping ChezWizper service..."
    if systemctl --user stop chezwizper.service; then
        print_success "Service stopped"
    else
        print_warning "Failed to stop service (continuing anyway)"
    fi
fi

if systemctl --user is-enabled --quiet chezwizper.service 2>/dev/null; then
    print_step "Disabling ChezWizper service..."
    if systemctl --user disable chezwizper.service; then
        print_success "Service disabled"
    else
        print_warning "Failed to disable service (continuing anyway)"
    fi
fi

# Step 2: Remove systemd service file
if [ -f "$SERVICE_FILE" ]; then
    print_step "Removing systemd service file..."
    if rm -f "$SERVICE_FILE"; then
        systemctl --user daemon-reload
        print_success "Service file removed"
    else
        print_error "Failed to remove service file"
    fi
fi

# Step 3: Remove binaries
if [ -f "$INSTALL_DIR/chezwizper" ]; then
    print_step "Removing ChezWizper binary..."
    if sudo rm -f "$INSTALL_DIR/chezwizper"; then
        print_success "Binary removed"
    else
        print_error "Failed to remove binary (you may need to run with sudo)"
    fi
fi

if [ -f "$INSTALL_DIR/chezwizper-update" ]; then
    print_step "Removing update script..."
    if sudo rm -f "$INSTALL_DIR/chezwizper-update"; then
        print_success "Update script removed"
    else
        print_warning "Failed to remove update script"
    fi
fi

# Step 4: Remove configuration
if [ "$KEEP_CONFIG" = false ] && [ -d "$CONFIG_DIR" ]; then
    print_step "Removing configuration..."
    if rm -rf "$CONFIG_DIR"; then
        print_success "Configuration removed"
    else
        print_error "Failed to remove configuration"
    fi
fi

# Step 5: Remove whisper installation
if [ "$KEEP_WHISPER" = false ] && [ -d "$WHISPER_DIR" ]; then
    print_step "Removing Whisper installation..."
    if rm -rf "$WHISPER_DIR"; then
        print_success "Whisper removed"
    else
        print_error "Failed to remove Whisper"
    fi
fi

# Step 6: Remove source backup
if [ -d "$SOURCE_BACKUP_DIR" ]; then
    print_step "Removing source backup..."
    if rm -rf "$SOURCE_BACKUP_DIR"; then
        print_success "Source backup removed"
    else
        print_error "Failed to remove source backup"
    fi
fi

# Step 7: Clean up data directory if empty
if [ -d "$DATA_DIR" ] && [ -z "$(ls -A "$DATA_DIR" 2>/dev/null)" ]; then
    print_step "Removing empty data directory..."
    if rmdir "$DATA_DIR"; then
        print_success "Data directory removed"
    fi
fi

# Step 8: Remove temporary files
if [ -n "$TEMP_FILES" ]; then
    print_step "Removing temporary audio files..."
    if rm -f /tmp/chezwizper_*.wav 2>/dev/null; then
        print_success "Temporary files removed"
    else
        print_warning "Some temporary files may remain in /tmp"
    fi
fi

# Final message
echo ""
print_success "ChezWizper has been uninstalled!"

if [ "$KEEP_CONFIG" = true ] && [ -d "$CONFIG_DIR" ]; then
    echo ""
    print_warning "Configuration preserved at: $CONFIG_DIR"
fi

if [ "$KEEP_WHISPER" = true ] && [ -d "$WHISPER_DIR" ]; then
    echo ""
    print_warning "Whisper installation preserved at: $WHISPER_DIR"
fi

echo ""
print_step "Optional cleanup steps:"
echo ""
echo "1. If you installed system dependencies specifically for ChezWizper,"
echo "   you may want to remove them:"
echo "   sudo pacman -Rs rust ydotool wtype wl-clipboard alsa-lib"
echo ""
echo "2. Remove Hyprland keybind from ~/.config/hypr/hyprland.conf:"
echo "   bindd = SUPER, R, ChezWizper, ..."
echo ""
echo "3. If you enabled ydotool service only for ChezWizper:"
echo "   systemctl --user disable --now ydotool.service"
echo ""
