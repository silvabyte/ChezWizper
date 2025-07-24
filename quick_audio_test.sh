#!/bin/bash

echo "🎵 Quick Audio Feedback Test"
echo "============================"

if [ -z "$OPENAI_API_KEY" ]; then
    echo "❌ OPENAI_API_KEY not set"
    echo "   export OPENAI_API_KEY='your-key'"
    exit 1
fi

echo "🚀 Starting ChezWizper with debug logging..."
RUST_LOG=debug ./target/release/chezwizper --config example_config_api.toml &
CHEZWIZPER_PID=$!

# Wait for startup
sleep 3

echo "📡 Testing toggle (should hear start sound)..."
curl -s -X POST http://127.0.0.1:3737/toggle

sleep 2

echo "📡 Testing toggle again (should hear stop sound)..."
curl -s -X POST http://127.0.0.1:3737/toggle

echo "⏳ Waiting 5 seconds for processing..."
sleep 5

echo "🛑 Stopping ChezWizper..."
kill $CHEZWIZPER_PID

echo "✅ Test complete. Check the output above for debug messages about sound playing."