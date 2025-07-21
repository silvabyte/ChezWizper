#!/bin/bash

# Simple script to toggle Whispy (chezwizper) recording

API_URL="http://127.0.0.1:3737"

# Function to check if Whispy API is up
is_whispy_running() {
    curl -s "$API_URL/" > /dev/null
}

# Check if Whispy is running, if not, start it
if ! is_whispy_running; then
    echo "Whispy is not running. Attempting to start chezwizper..."
    nohup chezwizper > /dev/null 2>&1 &
    # Wait for API to become available (timeout after 10 seconds)
    for i in {1..10}; do
        sleep 1
        if is_whispy_running; then
            echo "chezwizper started successfully."
            break
        fi
    done
    if ! is_whispy_running; then
        echo "Error: Failed to start chezwizper (API not responding at $API_URL)"
        exit 1
    fi
fi

echo "Toggling Whispy recording..."
curl -X POST "$API_URL/toggle" -s | grep -q "success" && echo "Recording toggled successfully!" || echo "Failed to toggle recording"
