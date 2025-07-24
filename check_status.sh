#!/bin/bash

echo "📊 ChezWizper Status Monitor"
echo "==========================="
echo "Press Ctrl+C to exit"
echo ""

# Function to get and display status
check_status() {
    STATUS=$(curl -s http://127.0.0.1:3737/status)
    if [ $? -eq 0 ]; then
        RECORDING=$(echo "$STATUS" | jq -r '.recording')
        STATE=$(echo "$STATUS" | jq -r '.status')
        
        # Clear line and print status
        printf "\r🎤 Recording: %-5s | State: %-10s" "$RECORDING" "$STATE"
    else
        printf "\r❌ Server not responding"
    fi
}

# Continuous monitoring
while true; do
    check_status
    sleep 0.5
done