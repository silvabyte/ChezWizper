#!/bin/bash

echo "🔧 Setting up KDE shortcut for ChezWizper"
echo "========================================"

# Create KDE shortcut configuration
SHORTCUT_NAME="ChezWizper Toggle"
SHORTCUT_KEY="Ctrl+Shift+R"
COMMAND="curl -X POST http://127.0.0.1:3737/toggle"

echo "Creating KDE shortcut:"
echo "  Name: $SHORTCUT_NAME"
echo "  Key: $SHORTCUT_KEY"
echo "  Command: $COMMAND"

# KDE shortcut configuration directory
SHORTCUT_DIR="$HOME/.config"
SHORTCUT_FILE="$SHORTCUT_DIR/kglobalshortcutsrc"

# Backup existing shortcuts
if [ -f "$SHORTCUT_FILE" ]; then
    cp "$SHORTCUT_FILE" "$SHORTCUT_FILE.backup.$(date +%s)"
    echo "✅ Backed up existing shortcuts"
fi

# Create the shortcut entry
cat << EOF >> "$HOME/.config/khotkeysrc"

[Data_3_1]
Comment=ChezWizper voice transcription toggle
Enabled=true
Name=ChezWizper Toggle
Type=SIMPLE_ACTION_DATA

[Data_3_1Actions]
ActionsCount=1

[Data_3_1Actions0]
CommandURL=$COMMAND
Type=COMMAND_URL

[Data_3_1Conditions]
Comment=
ConditionsCount=0

[Data_3_1Triggers]
Comment=Simple_action
TriggersCount=1

[Data_3_1Triggers0]
Key=Ctrl+Shift+R
Type=SHORTCUT
Uuid={12345678-1234-5678-9012-123456789abc}

EOF

echo "✅ KDE shortcut configuration added"
echo ""
echo "🔄 Restarting KDE hotkeys service..."
kquitapp5 khotkeys 2>/dev/null || kquitapp6 khotkeys 2>/dev/null
sleep 1
khotkeys 2>/dev/null &

echo "✅ Setup complete!"
echo ""
echo "📝 To use:"
echo "   1. Make sure ChezWizper is running:"
echo "      ./target/release/chezwizper --config example_config_api.toml"
echo "   2. Press Ctrl+Shift+R to toggle recording"
echo ""
echo "🔧 If the shortcut doesn't work:"
echo "   1. Go to System Settings → Shortcuts → Custom Shortcuts"
echo "   2. Manually add the shortcut with command: $COMMAND"
echo "   3. Or try a different key combination"