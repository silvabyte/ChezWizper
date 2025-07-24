#!/bin/bash

echo "🧹 Cleaning up ChezWizper processes"
echo "=================================="

# Kill all ChezWizper related processes
echo "Stopping ChezWizper processes..."
pkill -f "chezwizper" 2>/dev/null || true
pkill -f "3737" 2>/dev/null || true

# Wait a moment
sleep 1

# Force kill any remaining processes
pids=$(pgrep -f "chezwizper" 2>/dev/null || true)
if [ ! -z "$pids" ]; then
    echo "Force killing remaining processes: $pids"
    kill -9 $pids 2>/dev/null || true
fi

# Check if port 3737 is still in use
port_check=$(ss -tulpn | grep 3737 || true)
if [ ! -z "$port_check" ]; then
    echo "⚠️  Port 3737 still in use:"
    echo "$port_check"
else
    echo "✅ Port 3737 is free"
fi

# Final verification
remaining=$(ps aux | grep chezwizper | grep -v grep || true)
if [ ! -z "$remaining" ]; then
    echo "⚠️  Some processes still running:"
    echo "$remaining"
else
    echo "✅ All ChezWizper processes stopped"
fi

echo ""
echo "💡 You can now start ChezWizper with:"
echo "   ./target/release/chezwizper --config example_config_api.toml"