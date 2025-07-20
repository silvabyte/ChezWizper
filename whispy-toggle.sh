#!/bin/bash

# Simple script to toggle Whispy recording

API_URL="http://127.0.0.1:3737"

# Check if Whispy is running
if ! curl -s "$API_URL/" > /dev/null; then
    echo "Error: Whispy is not running (API not responding at $API_URL)"
    exit 1
fi

echo "Toggling Whispy recording..."
curl -X POST "$API_URL/toggle" -s | grep -q "success" && echo "Recording toggled successfully!" || echo "Failed to toggle recording"