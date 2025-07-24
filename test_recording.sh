#!/bin/bash

echo "🎤 Testing ChezWizper Audio Recording"
echo "===================================="

# Check if server is running
if ! curl -s http://127.0.0.1:3737/status > /dev/null 2>&1; then
    echo "❌ ChezWizper server not running on port 3737"
    echo "   Start it with: ./target/release/chezwizper --config example_config_api.toml"
    exit 1
fi

echo "✅ Server is running"

# Get initial status
echo "📊 Current status:"
curl -s http://127.0.0.1:3737/status | jq . 2>/dev/null || curl -s http://127.0.0.1:3737/status

# Start recording
echo ""
echo "🔴 Starting recording..."
curl -X POST http://127.0.0.1:3737/toggle

# Record for a specific duration
DURATION=3
echo "⏱️  Recording for $DURATION seconds..."
echo "🎙️  SPEAK NOW!"
sleep $DURATION

# Stop recording
echo "⏹️  Stopping recording..."
curl -X POST http://127.0.0.1:3737/toggle

# Wait for processing
echo "⏳ Waiting for transcription..."
sleep 5

# Check for audio files
echo ""
echo "📁 Looking for audio files in /tmp:"
ls -lah /tmp/chezwizper_*.wav 2>/dev/null | tail -5 || echo "   No audio files found"

# Check system audio
echo ""
echo "🔊 Audio system check:"
if command -v pactl &> /dev/null; then
    echo "   Default source: $(pactl info | grep 'Default Source' | cut -d: -f2)"
    echo "   Sources:"
    pactl list short sources | sed 's/^/     /'
elif command -v arecord &> /dev/null; then
    echo "   Recording devices:"
    arecord -l | grep card | sed 's/^/     /'
fi

echo ""
echo "💡 Tips:"
echo "   - Make sure your microphone is not muted"
echo "   - Check audio levels in your system settings"
echo "   - Try speaking louder or closer to the mic"
echo "   - The file size limit is 25MB (~26 minutes at 16kHz mono)"