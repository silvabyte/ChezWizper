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
BACKUP_DIR="$HOME/.config/chezwizper/backups"

# Parse command line arguments
UPDATE_WHISPER=false
FORCE_UPDATE=false
CHECK_ONLY=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --whisper)
            UPDATE_WHISPER=true
            shift
            ;;
        --force)
            FORCE_UPDATE=true
            shift
            ;;
        --check)
            CHECK_ONLY=true
            shift
            ;;
        --help)
            echo "ChezWizper Update Script"
            echo
            echo "Usage: $0 [OPTIONS]"
            echo
            echo "Options:"
            echo "  --whisper    Also update whisper.cpp installation"
            echo "  --force      Force update even if already up to date"
            echo "  --check      Only check for updates, don't install"
            echo "  --help       Show this help message"
            echo
            echo "Examples:"
            echo "  $0                  # Update ChezWizper only"
            echo "  $0 --whisper        # Update both ChezWizper and whisper.cpp"
            echo "  $0 --check          # Check for available updates"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Function to check for updates
check_for_updates() {
    cd "$1"
    git fetch origin >/dev/null 2>&1
    LOCAL=$(git rev-parse HEAD)
    REMOTE=$(git rev-parse origin/HEAD 2>/dev/null || git rev-parse origin/main 2>/dev/null || git rev-parse origin/master)
    
    if [ "$LOCAL" != "$REMOTE" ]; then
        return 0  # Updates available
    else
        return 1  # No updates
    fi
}

# Function to get current version
get_version() {
    cd "$1"
    git describe --tags --always 2>/dev/null || git rev-parse --short HEAD
}

print_step "ChezWizper Update Manager"
echo

# Check if ChezWizper is installed
if [ ! -f "$INSTALL_DIR/chezwizper" ]; then
    print_error "ChezWizper not found in $INSTALL_DIR"
    print_warning "Please run scripts/install-arch.sh first"
    exit 1
fi

# Stop the service before updating
print_step "Checking ChezWizper service status..."
if systemctl --user is-active --quiet chezwizper.service; then
    SERVICE_WAS_RUNNING=true
    print_warning "ChezWizper service is running. It will be restarted after update."
else
    SERVICE_WAS_RUNNING=false
fi

# Check for updates
cd "$CHEZWIZPER_DIR"
CURRENT_VERSION=$(get_version "$CHEZWIZPER_DIR")
print_step "Current ChezWizper version: $CURRENT_VERSION"

if check_for_updates "$CHEZWIZPER_DIR" || [ "$FORCE_UPDATE" = true ]; then
    CHEZWIZPER_UPDATE_AVAILABLE=true
    NEW_VERSION=$(cd "$CHEZWIZPER_DIR" && git rev-parse --short origin/HEAD 2>/dev/null || echo "latest")
    print_warning "ChezWizper update available: $CURRENT_VERSION → $NEW_VERSION"
else
    CHEZWIZPER_UPDATE_AVAILABLE=false
    print_success "ChezWizper is up to date"
fi

# Check whisper updates if requested
if [ "$UPDATE_WHISPER" = true ] && [ -d "$WHISPER_DIR" ]; then
    cd "$WHISPER_DIR"
    WHISPER_CURRENT=$(get_version "$WHISPER_DIR")
    print_step "Current whisper.cpp version: $WHISPER_CURRENT"
    
    if check_for_updates "$WHISPER_DIR" || [ "$FORCE_UPDATE" = true ]; then
        WHISPER_UPDATE_AVAILABLE=true
        print_warning "whisper.cpp update available"
    else
        WHISPER_UPDATE_AVAILABLE=false
        print_success "whisper.cpp is up to date"
    fi
fi

# If only checking, exit here
if [ "$CHECK_ONLY" = true ]; then
    if [ "$CHEZWIZPER_UPDATE_AVAILABLE" = true ] || [ "${WHISPER_UPDATE_AVAILABLE:-false}" = true ]; then
        print_warning "Updates are available. Run without --check to install."
        exit 0
    else
        print_success "Everything is up to date!"
        exit 0
    fi
fi

# Exit if no updates available (unless forced)
if [ "$CHEZWIZPER_UPDATE_AVAILABLE" = false ] && [ "${WHISPER_UPDATE_AVAILABLE:-false}" = false ] && [ "$FORCE_UPDATE" = false ]; then
    print_success "Nothing to update!"
    exit 0
fi

# Create backup directory
mkdir -p "$BACKUP_DIR"
BACKUP_DATE=$(date +%Y%m%d_%H%M%S)

# Backup configuration
print_step "Backing up configuration..."
if [ -f "$CONFIG_DIR/config.toml" ]; then
    cp "$CONFIG_DIR/config.toml" "$BACKUP_DIR/config_${BACKUP_DATE}.toml"
    print_success "Configuration backed up to $BACKUP_DIR/config_${BACKUP_DATE}.toml"
fi

# Stop service if running
if [ "$SERVICE_WAS_RUNNING" = true ]; then
    print_step "Stopping ChezWizper service..."
    systemctl --user stop chezwizper.service
fi

# Update ChezWizper
if [ "$CHEZWIZPER_UPDATE_AVAILABLE" = true ] || [ "$FORCE_UPDATE" = true ]; then
    print_step "Updating ChezWizper..."
    cd "$CHEZWIZPER_DIR"
    
    # Pull latest changes
    DEFAULT_BRANCH=$(git remote show origin | grep 'HEAD branch' | cut -d' ' -f5)
    if ! git pull origin "$DEFAULT_BRANCH"; then
        print_error "Failed to pull latest changes"
        exit 1
    fi
    
    # Clean and rebuild
    print_step "Building ChezWizper..."
    cargo clean
    if ! cargo build --release; then
        print_error "Failed to build ChezWizper"
        exit 1
    fi
    
    # Install new binary
    print_step "Installing updated binary..."
    if ! sudo cp target/release/chezwizper "$INSTALL_DIR/"; then
        print_error "Failed to install ChezWizper binary"
        exit 1
    fi
    sudo chmod +x "$INSTALL_DIR/chezwizper"
    print_success "ChezWizper updated successfully"
fi

# Update whisper if requested
if [ "$UPDATE_WHISPER" = true ] && [ "${WHISPER_UPDATE_AVAILABLE:-false}" = true -o "$FORCE_UPDATE" = true ]; then
    print_step "Updating whisper.cpp..."
    cd "$WHISPER_DIR"
    
    # Pull latest changes
    WHISPER_BRANCH=$(git remote show origin | grep 'HEAD branch' | cut -d' ' -f5)
    if ! git pull origin "$WHISPER_BRANCH"; then
        print_error "Failed to pull whisper.cpp updates"
        exit 1
    fi
    
    # Clean and rebuild
    print_step "Rebuilding whisper.cpp (this may take a while)..."
    # Clean build directory if it exists
    [ -d build ] && rm -rf build
    if ! ./build.sh; then
        print_error "Failed to build whisper.cpp"
        exit 1
    fi
    
    print_success "whisper.cpp updated successfully"
fi

# Check for configuration changes
print_step "Checking configuration compatibility..."

# Simply warn user to check for new config options
if [ -f "$CONFIG_DIR/config.toml" ]; then
    print_warning "Please check if new configuration options are available:"
    echo "  Current config: $CONFIG_DIR/config.toml"
    echo "  Backup saved to: $BACKUP_DIR/config_${BACKUP_DATE}.toml"
    echo "  Check documentation at: https://github.com/silvabyte/ChezWizper/blob/main/docs/"
fi

# Update systemd service if needed
print_step "Updating systemd service..."
systemctl --user daemon-reload

# Restart service if it was running
if [ "$SERVICE_WAS_RUNNING" = true ]; then
    print_step "Starting ChezWizper service..."
    if systemctl --user start chezwizper.service; then
        print_success "ChezWizper service restarted"
    else
        print_error "Failed to start ChezWizper service"
        print_warning "Check logs with: journalctl --user -u chezwizper.service -e"
    fi
fi

# Show update summary
echo
print_success "Update completed!"
echo
NEW_VERSION=$(get_version "$CHEZWIZPER_DIR")
echo "ChezWizper version: $NEW_VERSION"
if [ "$UPDATE_WHISPER" = true ]; then
    WHISPER_VERSION=$(get_version "$WHISPER_DIR" 2>/dev/null || echo "unknown")
    echo "whisper.cpp version: $WHISPER_VERSION"
fi

# Show post-update instructions
echo
print_step "Post-update steps:"
echo "1. Check service status: ${GREEN}systemctl --user status chezwizper.service${NC}"
echo "2. View logs if needed: ${GREEN}journalctl --user -u chezwizper.service -f${NC}"
echo "3. Test recording with your keybind (e.g., Super+R)"

# Check for release notes
if [ -f "$CHEZWIZPER_DIR/CHANGELOG.md" ] || [ -f "$CHEZWIZPER_DIR/RELEASES.md" ]; then
    echo
    print_warning "Check release notes for important changes"
fi