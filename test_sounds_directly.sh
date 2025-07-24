#!/bin/bash

echo "🔊 Direct Sound System Test"
echo "==========================="

echo "Testing paplay with 800Hz tone (should sound like recording start)..."
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
" | paplay --raw --format=s16le --rate=44100 --channels=1

echo "Testing paplay with 400Hz tone (should sound like recording stop)..."
python3 -c "
import math
import sys
samples = 8820  # 0.2 seconds at 44100 Hz
freq = 400.0
for i in range(samples):
    t = i / 44100.0
    sample = math.sin(2.0 * math.pi * freq * t)
    sample_i16 = int(sample * 16384)
    sys.stdout.buffer.write(sample_i16.to_bytes(2, 'little', signed=True))
" | paplay --raw --format=s16le --rate=44100 --channels=1

echo "Testing paplay with 600Hz tone (should sound like completion)..."
python3 -c "
import math
import sys
samples = 4410  # 0.1 seconds at 44100 Hz
freq = 600.0
for i in range(samples):
    t = i / 44100.0
    sample = math.sin(2.0 * math.pi * freq * t)
    sample_i16 = int(sample * 16384)
    sys.stdout.buffer.write(sample_i16.to_bytes(2, 'little', signed=True))
" | paplay --raw --format=s16le --rate=44100 --channels=1

echo "✅ Direct sound tests complete!"
echo ""
echo "If you heard the three different beeps above, the sound system works."
echo "If ChezWizper isn't making sounds, the issue is in the code integration."