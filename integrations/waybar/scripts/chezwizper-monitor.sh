#!/bin/bash

# Monitor ChezWizper service logs and update state based on notifications
STATE_FILE="/tmp/chezwizper_waybar_state"
TEXT_FILE="/tmp/chezwizper_last_text"
COMPLETE_TIME_FILE="/tmp/chezwizper_complete_time"
TRANSCRIPTION_FILE="/tmp/chezwizper_last_transcription"

# Initialize state
echo "idle" > "$STATE_FILE"

# Monitor the ChezWizper service logs
journalctl --user -u chezwizper.service -f -n 0 2>/dev/null | while read -r line; do
    if echo "$line" | grep -q "Showing recording indicator"; then
        echo "recording" > "$STATE_FILE"
    elif echo "$line" | grep -q "Showing processing indicator"; then
        echo "processing" > "$STATE_FILE"
    elif echo "$line" | grep -q "Showing completion indicator"; then
        # Mark completion and save timestamp
        echo "complete" > "$STATE_FILE"
        date +%s > "$COMPLETE_TIME_FILE"
        # Wait a moment for clipboard to be updated, then save it
        (sleep 0.5 && wl-paste > "$TRANSCRIPTION_FILE" 2>/dev/null) &
    elif echo "$line" | grep -q "Starting recording"; then
        echo "recording" > "$STATE_FILE"
    elif echo "$line" | grep -q "Stopping recording"; then
        echo "processing" > "$STATE_FILE"
    elif echo "$line" | grep -q "Transcription successful:"; then
        # Extract character count for preview
        char_count=$(echo "$line" | grep -oP '\d+(?= chars)')
        echo "${char_count} chars transcribed" > "$TEXT_FILE"
    elif echo "$line" | grep -q "Injecting text:"; then
        # Save that text was injected
        char_count=$(echo "$line" | grep -oP '\d+(?= chars)')
        echo "${char_count} chars" > "$TEXT_FILE"
    fi
done