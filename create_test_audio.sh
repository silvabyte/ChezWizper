#!/bin/bash

echo "🎵 Creating test audio files for ChezWizper"
echo "=========================================="

# Check for audio generation tools
if command -v espeak &> /dev/null; then
    echo "✅ Using espeak to generate speech"
    
    # Generate test speech
    espeak "Hello, this is a test of the ChezWizper transcription system. Testing one, two, three." \
           -w test_speech.wav -s 150
    
    # Convert to 16kHz mono if sox is available
    if command -v sox &> /dev/null; then
        sox test_speech.wav -r 16000 -c 1 test_speech_16k.wav
        rm test_speech.wav
        mv test_speech_16k.wav test_speech.wav
    fi
    
    echo "✅ Created: test_speech.wav"
    
elif command -v say &> /dev/null; then
    echo "✅ Using macOS 'say' command"
    say "Hello, this is a test of the ChezWizper transcription system. Testing one, two, three." \
        -o test_speech_raw.aiff
    
    # Convert to WAV
    if command -v ffmpeg &> /dev/null; then
        ffmpeg -i test_speech_raw.aiff -ar 16000 -ac 1 test_speech.wav -y
        rm test_speech_raw.aiff
    fi
    
    echo "✅ Created: test_speech.wav"
    
elif command -v ffmpeg &> /dev/null; then
    echo "⚠️  No text-to-speech tool found, creating tone instead"
    
    # Create a test tone that varies (more likely to be transcribed)
    ffmpeg -f lavfi -i "sine=frequency=440:duration=1,sine=frequency=880:duration=1,sine=frequency=440:duration=1" \
           -ar 16000 -ac 1 test_tone.wav -y
    
    echo "✅ Created: test_tone.wav (not speech, but can test API)"
else
    echo "❌ No audio generation tools found (espeak, say, or ffmpeg)"
    echo "   Install one of these tools to generate test audio"
    exit 1
fi

# Analyze the created file
if [ -f test_speech.wav ]; then
    echo ""
    echo "📊 Analyzing generated audio:"
    ./target/release/audio_info test_speech.wav
    
    echo ""
    echo "🎯 Ready to test! Run:"
    echo "   ./target/release/transcribe_file test_speech.wav"
elif [ -f test_tone.wav ]; then
    echo ""
    echo "📊 Analyzing generated audio:"
    ./target/release/audio_info test_tone.wav
    
    echo ""
    echo "🎯 Ready to test! Run:"
    echo "   ./target/release/transcribe_file test_tone.wav"
fi