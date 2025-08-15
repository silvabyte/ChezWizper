#!/bin/bash

# Check if ChezWizper service is running
if ! systemctl --user is-active chezwizper.service >/dev/null 2>&1; then
    echo '{"text": "", "class": "chezwizper-offline", "tooltip": "ChezWizper not running"}'
    exit 0
fi

STATE_FILE="/tmp/chezwizper_waybar_state"
TEXT_FILE="/tmp/chezwizper_last_text"
COMPLETE_TIME_FILE="/tmp/chezwizper_complete_time"

# Read the current state from monitor
state=$(cat "$STATE_FILE" 2>/dev/null || echo "idle")

case "$state" in
    "recording")
        echo '{"text": "🔴 REC", "class": "chezwizper-recording", "tooltip": "Recording... Press Super+R to stop"}'
        ;;
    "processing")
        echo '{"text": "⚡ Analyzing", "class": "chezwizper-processing", "tooltip": "Transcribing audio..."}'
        ;;
    "complete")
        # Check how long since completion
        if [ -f "$COMPLETE_TIME_FILE" ]; then
            current_time=$(date +%s)
            complete_time=$(cat "$COMPLETE_TIME_FILE")
            elapsed=$((current_time - complete_time))
            
            # Show complete indicator for 10 seconds
            if [ $elapsed -lt 10 ]; then
                text_info=$(cat "$TEXT_FILE" 2>/dev/null || echo "Transcribed")
                echo "{\"text\": \"✅ Done\", \"class\": \"chezwizper-complete\", \"tooltip\": \"${text_info}\\nText is in clipboard\"}"
            else
                # After 10 seconds, go back to idle
                echo "idle" > "$STATE_FILE"
                rm -f "$COMPLETE_TIME_FILE"
                echo '{"text": "", "class": "chezwizper-idle", "tooltip": "Press Super+R to record"}'
            fi
        else
            # No completion time, go idle
            echo "idle" > "$STATE_FILE"
            echo '{"text": "", "class": "chezwizper-idle", "tooltip": "Press Super+R to record"}'
        fi
        ;;
    *)
        echo '{"text": "", "class": "chezwizper-idle", "tooltip": "Press Super+R to record"}'
        ;;
esac