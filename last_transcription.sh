#!/bin/bash

echo "🎯 Last Transcription Results"
echo "============================"

# Check if ChezWizper is running and get logs
if pgrep -f chezwizper > /dev/null; then
    echo "✅ ChezWizper is running"
    
    # Try to get recent transcription from journalctl
    echo "📝 Recent transcription activity:"
    journalctl --since "10 minutes ago" -t chezwizper 2>/dev/null | \
        grep -E "(Transcription complete|transcription.*chars)" | \
        tail -5 || echo "   No journalctl logs found"
    
    echo ""
    echo "📋 Current clipboard contents:"
    if command -v wl-paste &> /dev/null; then
        wl-paste 2>/dev/null | head -3
    elif command -v xclip &> /dev/null; then
        xclip -o -selection clipboard 2>/dev/null | head -3
    else
        echo "   No clipboard tool found"
    fi
    
else
    echo "❌ ChezWizper not running"
fi

echo ""
echo "💡 To see what was transcribed:"
echo "   1. The text should be in your clipboard (try Ctrl+V in a text editor)"
echo "   2. Check the server logs for 'Transcription complete' messages"
echo "   3. If text injection failed, the paste shortcut was simulated"