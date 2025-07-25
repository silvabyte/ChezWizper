#!/bin/bash
set -euo pipefail

# Check for help flag first
if [[ "$*" == *"--help"* ]]; then
    echo "ChezWizper Universal Installation Script"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "This script detects your OS and runs the appropriate installer."
    echo "All options are passed to the distribution-specific installer."
    echo ""
    echo "Common Options:"
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
    echo ""
    echo "Supported Systems:"
    echo "  - Arch Linux (including Omarchy)"
    echo "  - Ubuntu/Debian (coming soon)"
    echo "  - Fedora (coming soon)"
    exit 0
fi

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

# Detect OS and distribution
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if [ -f /etc/arch-release ]; then
            echo "arch"
        elif [ -f /etc/debian_version ]; then
            echo "debian"
        elif [ -f /etc/fedora-release ]; then
            echo "fedora"
        elif [ -f /etc/os-release ]; then
            . /etc/os-release
            echo "$ID"
        else
            echo "unknown"
        fi
    else
        echo "unsupported"
    fi
}

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
CHEZWIZPER_DIR="$SCRIPT_DIR"

# Configuration variables
WHISPER_DIR="$HOME/.local/share/chezwizper/whisper"
CONFIG_DIR="$HOME/.config/chezwizper"
INSTALL_DIR="/usr/local/bin"
SOURCE_BACKUP_DIR="$HOME/.local/share/chezwizper/source"

# Detect OS
OS=$(detect_os)
print_step "Detected OS: $OS"

case $OS in
    "arch")
        print_step "Starting ChezWizper installation for Arch Linux"
        DISTRO_INSTALL_SCRIPT="$SCRIPT_DIR/install-arch.sh"
        ;;
    "debian"|"ubuntu")
        print_error "Ubuntu/Debian support coming soon. Please install manually for now."
        exit 1
        ;;
    "fedora")
        print_error "Fedora support coming soon. Please install manually for now."
        exit 1
        ;;
    *)
        print_error "Unsupported OS: $OS"
        print_warning "Please install manually or contribute an install script for your OS"
        exit 1
        ;;
esac

# Check if distro-specific installer exists
if [ ! -f "$DISTRO_INSTALL_SCRIPT" ]; then
    print_error "Distribution-specific installer not found: $DISTRO_INSTALL_SCRIPT"
    exit 1
fi

# Make the distro script executable and run it with all arguments
chmod +x "$DISTRO_INSTALL_SCRIPT"
exec "$DISTRO_INSTALL_SCRIPT" "$@"