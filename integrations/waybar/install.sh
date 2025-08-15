#!/bin/bash

# ChezWizper Waybar Integration Installer
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_DIR="$HOME/.config"

echo "🎯 ChezWizper Waybar Integration Installer"
echo "==========================================="

# Check dependencies
echo "Checking dependencies..."
if ! command -v waybar &> /dev/null; then
    echo "❌ Waybar is not installed. Please install waybar first."
    exit 1
fi

if ! command -v wl-paste &> /dev/null; then
    echo "⚠️  Warning: wl-clipboard not found. Clipboard features may not work."
    echo "   Install with: sudo pacman -S wl-clipboard"
fi

if ! systemctl --user is-active chezwizper.service &> /dev/null; then
    echo "⚠️  Warning: ChezWizper service is not running."
    echo "   Start with: systemctl --user start chezwizper.service"
fi

# Install scripts
echo "Installing waybar scripts..."
mkdir -p "$CONFIG_DIR/waybar/scripts"
cp "$SCRIPT_DIR/scripts/"*.sh "$CONFIG_DIR/waybar/scripts/"
chmod +x "$CONFIG_DIR/waybar/scripts/chezwizper-"*.sh

# Install systemd service
echo "Installing monitor service..."
mkdir -p "$CONFIG_DIR/systemd/user"
cp "$SCRIPT_DIR/systemd/chezwizper-waybar.service" "$CONFIG_DIR/systemd/user/"
systemctl --user daemon-reload

# Show config snippets
echo ""
echo "✅ Installation complete!"
echo ""
echo "📝 Next steps:"
echo ""
echo "1. Add the ChezWizper module to your waybar config (~/.config/waybar/config.jsonc):"
echo "   - Add \"custom/chezwizper\" to modules-center (recommended) or modules-left/right"
echo "   - Add the module configuration from: $SCRIPT_DIR/config/chezwizper-module.jsonc"
echo ""
echo "2. Add the styles to your waybar CSS (~/.config/waybar/style.css):"
echo "   - Copy styles from: $SCRIPT_DIR/config/chezwizper-style.css"
echo ""
echo "3. Enable and start the monitor service:"
echo "   systemctl --user enable --now chezwizper-waybar.service"
echo ""
echo "4. Restart waybar:"
echo "   pkill waybar && waybar &"
echo ""
echo "5. Test with Super+R to start recording!"
echo ""

# Offer automatic config
read -p "Would you like to automatically add the module to waybar config? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    CONFIG_FILE="$CONFIG_DIR/waybar/config.jsonc"
    if [ -f "$CONFIG_FILE" ]; then
        # Backup original
        cp "$CONFIG_FILE" "$CONFIG_FILE.bak.$(date +%s)"
        echo "Backed up config to $CONFIG_FILE.bak.*"
        
        # Check if module already exists
        if grep -q "custom/chezwizper" "$CONFIG_FILE"; then
            echo "Module already exists in config."
        else
            echo "⚠️  Please manually add \"custom/chezwizper\" to your modules and the module config."
            echo "   See $SCRIPT_DIR/config/chezwizper-module.jsonc for the configuration."
        fi
    else
        echo "Config file not found at $CONFIG_FILE"
    fi
fi

echo ""
echo "🎉 Enjoy voice transcription with ChezWizper + Waybar!"