#!/bin/bash

echo "🔄 Restarting ChezWizper with new audio"
echo "======================================"

# 1. Kill ALL ChezWizper processes
echo "🛑 Stopping all ChezWizper processes..."
pkill -9 -f chezwizper 2>/dev/null || true
pkill -9 -f "3737" 2>/dev/null || true

# Wait for processes to fully stop
sleep 2

# 2. Check for any remaining processes
remaining=$(ps aux | grep chezwizper | grep -v grep || true)
if [ ! -z "$remaining" ]; then
    echo "⚠️  Warning: Some processes still running:"
    echo "$remaining"
    echo "Trying to force kill..."
    pids=$(pgrep -f chezwizper 2>/dev/null || true)
    if [ ! -z "$pids" ]; then
        kill -9 $pids 2>/dev/null || true
    fi
    sleep 1
fi

# 3. Verify port 3737 is free
port_check=$(ss -tulpn | grep 3737 || true)
if [ ! -z "$port_check" ]; then
    echo "❌ Port 3737 still in use:"
    echo "$port_check"
    exit 1
fi

# 4. Check API key
if [ -z "$OPENAI_API_KEY" ]; then
    echo "❌ OPENAI_API_KEY not set!"
    echo "   Please run: export OPENAI_API_KEY='your-key-here'"
    exit 1
fi

echo "✅ Environment ready"

# 5. Start fresh ChezWizper
echo "🚀 Starting ChezWizper with new audio feedback..."
echo "   (Press Ctrl+C to stop)"
echo ""

exec ./target/release/chezwizper --config example_config_api.toml