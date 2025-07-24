#!/bin/bash

echo "🔊 Manual Sound Testing"
echo "======================"

echo "Testing the exact commands ChezWizper should use..."

echo "1. Testing pactl bell (what ChezWizper tries first):"
if pactl play-sample bell-window-system 2>/dev/null; then
    echo "   ✅ pactl bell worked"
else
    echo "   ❌ pactl bell failed"
fi

echo ""
echo "2. Testing aplay with system sound:"
if aplay /usr/share/sounds/alsa/Front_Left.wav 2>/dev/null; then
    echo "   ✅ aplay system sound worked"  
else
    echo "   ❌ aplay system sound failed"
fi

echo ""
echo "3. Testing speaker-test (ChezWizper's fallback):"
echo "   Playing 800Hz tone for 0.2 seconds..."
timeout 0.2 speaker-test -t sine -f 800 -c 1 > /dev/null 2>&1
if [ $? -eq 0 ] || [ $? -eq 124 ]; then  # 124 is timeout success
    echo "   ✅ speaker-test worked"
else
    echo "   ❌ speaker-test failed"
fi

echo ""
echo "If you heard sounds above, then the issue is in ChezWizper's code."
echo "If you didn't hear sounds, then we need to fix the system audio setup."