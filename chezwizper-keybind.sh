#!/bin/bash

# Script to set up Whispy keybind in Hyprland

echo "Setting up Whispy keybind for Hyprland..."

# Check if hyprctl is available
if ! command -v hyprctl &> /dev/null; then
    echo "Error: hyprctl not found. Are you running Hyprland?"
    exit 1
fi

# Add temporary keybind (will need to be added to config for persistence)
hyprctl keyword bind "CTRL SHIFT, A, exec, curl -X POST http://127.0.0.1:3737/toggle"

echo "Keybind added temporarily. To make it permanent, add this line to your Hyprland config:"
echo ""
echo "bind = CTRL SHIFT, A, exec, curl -X POST http://127.0.0.1:3737/toggle"
echo ""
echo "Location: ~/.config/hypr/hyprland.conf"
echo ""
echo "You can now use Ctrl+Shift+A to toggle recording!"
echo ""
echo "API endpoints available:"
echo "  POST http://127.0.0.1:3737/toggle - Toggle recording"
echo "  GET  http://127.0.0.1:3737/status  - Check recording status"