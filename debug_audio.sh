#!/bin/bash

echo "🔍 ChezWizper Audio Debug Tool"
echo "=============================="

# Find recent ChezWizper audio files
echo "📁 Recent ChezWizper recordings in /tmp:"
ls -la /tmp/chezwizper_*.wav 2>/dev/null | tail -10

# Get the most recent file
LATEST=$(ls -t /tmp/chezwizper_*.wav 2>/dev/null | head -1)

if [ -z "$LATEST" ]; then
    echo "❌ No ChezWizper audio files found in /tmp"
    echo "   Make sure delete_audio_files = false in your config"
    exit 1
fi

echo ""
echo "📊 Analyzing most recent: $LATEST"

# Check file size
SIZE=$(stat -f%z "$LATEST" 2>/dev/null || stat -c%s "$LATEST" 2>/dev/null)
echo "   Size: $SIZE bytes"

# Check duration and format with ffprobe if available
if command -v ffprobe &> /dev/null; then
    echo "   Format info:"
    ffprobe -v error -show_format -show_streams "$LATEST" 2>&1 | grep -E "(duration|sample_rate|channels|codec_name)" | sed 's/^/     /'
fi

# Check if file has actual audio content
if command -v sox &> /dev/null; then
    echo "   Audio statistics:"
    sox "$LATEST" -n stat 2>&1 | grep -E "(Maximum amplitude|RMS amplitude)" | sed 's/^/     /'
fi

echo ""
echo "🎯 To transcribe this file manually:"
echo "   ./target/release/transcribe_file \"$LATEST\""

# Offer to transcribe
read -p "📝 Transcribe now? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ -z "$OPENAI_API_KEY" ]; then
        echo "❌ OPENAI_API_KEY not set"
        exit 1
    fi
    ./target/release/transcribe_file "$LATEST" --verbose
fi