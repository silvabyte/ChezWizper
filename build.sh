#!/bin/bash
set -e

echo "Building ChezWizper..."

# Check for required dependencies
check_dependency() {
    if ! command -v $1 &> /dev/null; then
        echo "Error: $1 is required but not installed."
        echo "Please install it using your package manager."
        exit 1
    fi
}

echo "Checking dependencies..."
check_dependency "cargo"

# Check for whisper (optional - can be configured with custom path)
if ! command -v whisper &> /dev/null; then
    echo "Note: whisper not found in PATH. You can configure a custom path in config.toml"
fi

# Check for text injection tool
if ! command -v wtype &> /dev/null && ! command -v ydotool &> /dev/null; then
    echo "Error: Either wtype or ydotool is required for text injection."
    echo "Please install one of them."
    exit 1
fi

# Build release version
echo "Building release version..."
cargo build --release

echo "Build complete! Binary is at: target/release/chezwizper"
echo ""
echo "To install system-wide:"
echo "  sudo cp target/release/chezwizper /usr/local/bin/"
echo ""
echo "To run:"
echo "  ./target/release/chezwizper"