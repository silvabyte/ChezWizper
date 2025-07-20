#!/bin/bash

# Simple script to check Whispy status

API_URL="http://127.0.0.1:3737"

# Check if Whispy is running
if ! curl -s "$API_URL/" > /dev/null; then
    echo "Error: Whispy is not running (API not responding at $API_URL)"
    exit 1
fi

echo "Whispy Status:"
curl -s "$API_URL/status" | python3 -m json.tool 2>/dev/null || curl -s "$API_URL/status"