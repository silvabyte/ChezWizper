#!/bin/bash

echo "🎤 ChezWizper Toggle Test"
echo "========================"

# Function to check status
get_status() {
    curl -s http://127.0.0.1:3737/status | jq -r '.recording' 2>/dev/null || echo "error"
}

# Function to toggle
toggle() {
    curl -s -X POST http://127.0.0.1:3737/toggle > /dev/null
}

# Initial status
echo "Initial status: $(get_status)"
echo ""

# Test sequence
echo "Test 1: Single toggle (should start recording)"
toggle
sleep 0.5
echo "Status after toggle: $(get_status)"
echo ""

sleep 2

echo "Test 2: Second toggle (should stop recording)"
toggle  
sleep 0.5
echo "Status after toggle: $(get_status)"
echo ""

sleep 2

echo "Test 3: Rapid toggles"
for i in {1..5}; do
    toggle
    sleep 0.2
    echo "Toggle $i - Status: $(get_status)"
done

echo ""
echo "Final status: $(get_status)"
echo ""
echo "Check server logs to see if recording actually started/stopped correctly"