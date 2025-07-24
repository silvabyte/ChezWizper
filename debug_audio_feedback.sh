#!/bin/bash

echo "🔍 Debugging ChezWizper Audio Feedback"
echo "======================================"

echo "1. Checking audio system availability:"
echo "   paplay: $(command -v paplay >/dev/null && echo "✅ Available" || echo "❌ Not found")"
echo "   aplay: $(command -v aplay >/dev/null && echo "✅ Available" || echo "❌ Not found")"
echo "   beep: $(command -v beep >/dev/null && echo "✅ Available" || echo "❌ Not found")"
echo ""

echo "2. Testing each sound system manually:"

# Test paplay with generated tone
if command -v paplay >/dev/null; then
    echo "🔊 Testing paplay with generated tone..."
    
    # Generate a simple 800Hz tone for 0.1 seconds
    python3 -c "
import math
import sys
samples = 4410  # 0.1 seconds at 44100 Hz
freq = 800.0
for i in range(samples):
    t = i / 44100.0
    sample = math.sin(2.0 * math.pi * freq * t)
    sample_i16 = int(sample * 16384)
    sys.stdout.buffer.write(sample_i16.to_bytes(2, 'little', signed=True))
" 2>/dev/null | paplay --raw --format=s16le --rate=44100 --channels=1 2>/dev/null

    if [ $? -eq 0 ]; then
        echo "   ✅ paplay test successful"
    else
        echo "   ❌ paplay test failed"
    fi
else
    echo "   ⚠️  paplay not available"
fi

# Test aplay with system sounds
if command -v aplay >/dev/null; then
    echo "🔊 Testing aplay with system sounds..."
    
    # Check for common system sound files
    for sound_file in "/usr/share/sounds/alsa/Front_Left.wav" "/usr/share/sounds/Oxygen-Sys-Log-In.ogg" "/usr/share/sounds/freedesktop/stereo/bell.oga"; do
        if [ -f "$sound_file" ]; then
            echo "   Found sound file: $sound_file"
            aplay "$sound_file" 2>/dev/null &
            SOUND_PID=$!
            sleep 0.2
            kill $SOUND_PID 2>/dev/null
            echo "   ✅ aplay test with system sound"
            break
        fi
    done
    
    if [ ! -f "/usr/share/sounds/alsa/Front_Left.wav" ]; then
        echo "   ⚠️  No system sound files found"
    fi
else
    echo "   ⚠️  aplay not available"
fi

# Test beep
if command -v beep >/dev/null; then
    echo "🔊 Testing beep..."
    if beep -f 800 -l 100 2>/dev/null; then
        echo "   ✅ beep test successful"
    else
        echo "   ❌ beep test failed (may need root or special permissions)"
    fi
else
    echo "   ⚠️  beep not available"
fi

echo ""
echo "3. Checking PulseAudio status:"
if command -v pulseaudio >/dev/null; then
    if pulseaudio --check; then
        echo "   ✅ PulseAudio is running"
    else
        echo "   ❌ PulseAudio is not running"
    fi
else
    echo "   ⚠️  PulseAudio not installed"
fi

echo ""
echo "4. Alternative sound test with speaker-test:"
if command -v speaker-test >/dev/null; then
    echo "🔊 Testing with speaker-test (2 seconds)..."
    timeout 2 speaker-test -t sine -f 800 -c 1 2>/dev/null || echo "   speaker-test available but may have failed"
else
    echo "   ⚠️  speaker-test not available"
fi

echo ""
echo "5. ChezWizper log check:"
if pgrep -f chezwizper > /dev/null; then
    echo "   ✅ ChezWizper is running"
    echo "   Recent audio-related logs:"
    journalctl --since "5 minutes ago" -t chezwizper 2>/dev/null | grep -i -E "(sound|audio|beep)" | tail -3 || echo "   No audio logs found"
else
    echo "   ❌ ChezWizper not running"
fi

echo ""
echo "💡 Recommendations:"
echo "   1. Install missing audio tools: sudo pacman -S alsa-utils pulseaudio-utils"
echo "   2. Check audio output is not muted: pactl list sinks"
echo "   3. Test manual sound: pactl play-sample bell-window-system"
echo "   4. Enable debug logging: RUST_LOG=debug ./target/release/chezwizper --config example_config_api.toml"