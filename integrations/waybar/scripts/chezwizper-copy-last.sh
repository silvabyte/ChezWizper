#!/bin/bash

# Since ChezWizper already puts text in clipboard, clicking Done doesn't need to do anything
# except show a notification that the text is already in clipboard

# Get current clipboard content
current_text=$(wl-paste 2>/dev/null)

if [ -n "$current_text" ]; then
    char_count=${#current_text}
    hyprctl notify 1 2000 "rgb(33ff33)" "📋 ${char_count} chars already in clipboard"
else
    hyprctl notify 1 2000 "rgb(ff3333)" "❌ Clipboard is empty"
fi