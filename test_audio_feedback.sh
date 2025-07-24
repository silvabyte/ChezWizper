#!/bin/bash

echo "🔊 Testing ChezWizper Audio Feedback"
echo "===================================="

# Check if ChezWizper is running
if ! pgrep -f chezwizper > /dev/null; then
    echo "❌ ChezWizper not running!"
    echo "   Start it with: ./target/release/chezwizper --config example_config_api.toml"
    exit 1
fi

echo "✅ ChezWizper is running"
echo ""
echo "🎵 Audio Feedback Test Sequence:"
echo "   - High beep (800Hz) = Recording started"
echo "   - Low beep (400Hz) = Recording stopped (processing)"  
echo "   - Medium beep (600Hz) = Transcription complete"
echo ""

# Test sequence
echo "🔴 Starting recording... (listen for high beep)"
curl -s -X POST http://127.0.0.1:3737/toggle > /dev/null
sleep 2

echo "⏹️  Recording something for 3 seconds..."
echo "🎙️  Say something now!"
sleep 3

echo "🔄 Stopping recording... (listen for low beep, then medium beep)"
curl -s -X POST http://127.0.0.1:3737/toggle > /dev/null

echo "⏳ Waiting for transcription to complete..."
sleep 8

echo ""
echo "✅ Test complete! You should have heard:"
echo "   1. High beep when recording started"
echo "   2. Low beep when recording stopped"
echo "   3. Medium beep when transcription finished"
echo ""
echo "💡 To disable audio feedback, set 'audio_feedback = false' in your config"
echo "🔧 To adjust volumes, modify the frequencies in src/ui/mod.rs"