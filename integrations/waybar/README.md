# ChezWizper Waybar Integration

Real-time visual indicators for ChezWizper voice transcription in your Waybar system panel.

## Features

- **Live Status Display**: Visual indicators for recording, processing, and completion states
- **Smooth Animations**: Pulsing recording indicator, processing animation, and fade effects
- **Smart Integration**: Automatically syncs with ChezWizper service events
- **Click Actions**: Toggle recording or view clipboard status with mouse clicks
- **Minimal Resource Usage**: Lightweight monitoring using systemd journal

## Visual States

| State | Indicator | Description |
|-------|-----------|-------------|
| Recording | 🔴 REC | Pulsing red indicator while recording audio |
| Processing | ⚡ Analyzing | Animated orange indicator during transcription |
| Complete | ✅ Done | Green success indicator for 10 seconds |
| Idle | (hidden) | No indicator when not in use |

## Installation

### Quick Install

```bash
cd integrations/waybar
./install.sh
```

### Manual Installation

1. **Copy scripts to waybar config:**
```bash
cp scripts/*.sh ~/.config/waybar/scripts/
chmod +x ~/.config/waybar/scripts/chezwizper-*.sh
```

2. **Add module to waybar config** (`~/.config/waybar/config.jsonc`):
```jsonc
{
  "modules-center": [
    "custom/chezwizper",  // Add this
    "clock",
    // ... other modules
  ],
  
  // Add this module configuration
  "custom/chezwizper": {
    "exec": "~/.config/waybar/scripts/chezwizper-status.sh",
    "interval": 1,
    "format": "{}",
    "return-type": "json",
    "on-click": "bash -c 'if [ \"$(cat /tmp/chezwizper_waybar_state 2>/dev/null)\" = \"complete\" ]; then ~/.config/waybar/scripts/chezwizper-copy-last.sh; else curl -X POST http://127.0.0.1:3737/toggle; fi'"
  }
}
```

3. **Add styles to waybar CSS** (`~/.config/waybar/style.css`):
```css
/* Copy contents from config/chezwizper-style.css */
```

4. **Install and start monitor service:**
```bash
cp systemd/chezwizper-waybar.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now chezwizper-waybar.service
```

5. **Restart waybar:**
```bash
pkill waybar && waybar &
```

## Configuration

### Module Placement

The module can be placed in any waybar section:
- `modules-left`: Left side of bar
- `modules-center`: Center of bar (recommended)
- `modules-right`: Right side of bar

### Customizing Styles

Edit the CSS classes in your `style.css`:

```css
#custom-chezwizper.chezwizper-recording {
  color: #ff3333;  /* Change recording color */
}

#custom-chezwizper.chezwizper-processing {
  color: #ffaa33;  /* Change processing color */
}

#custom-chezwizper.chezwizper-complete {
  color: #33ff33;  /* Change complete color */
}
```

### Adjusting Timings

In `chezwizper-status.sh`, you can modify:
- Completion display duration (default: 10 seconds)
- Update interval in waybar config (default: 1 second)

## How It Works

1. **Monitor Service**: `chezwizper-waybar.service` watches ChezWizper logs for state changes
2. **State Tracking**: Writes current state to `/tmp/chezwizper_waybar_state`
3. **Status Script**: Waybar polls `chezwizper-status.sh` every second
4. **Visual Update**: Returns JSON with text, CSS class, and tooltip
5. **Click Handler**: Toggles recording or shows clipboard notification

## Files

```
integrations/waybar/
├── scripts/
│   ├── chezwizper-status.sh      # Main waybar status script
│   ├── chezwizper-monitor.sh     # Service log monitor
│   └── chezwizper-copy-last.sh   # Clipboard notification
├── config/
│   ├── chezwizper-module.jsonc   # Waybar module config
│   └── chezwizper-style.css      # CSS styles
├── systemd/
│   └── chezwizper-waybar.service # Monitor service unit
├── install.sh                     # Installation script
└── README.md                      # This file
```

## Troubleshooting

### Indicator not showing

1. Check if monitor service is running:
```bash
systemctl --user status chezwizper-waybar.service
```

2. Verify ChezWizper is running:
```bash
systemctl --user status chezwizper.service
```

3. Test the status script manually:
```bash
~/.config/waybar/scripts/chezwizper-status.sh
```

4. Restart waybar:
```bash
pkill waybar && waybar &
```

### States not updating

1. Check state file:
```bash
cat /tmp/chezwizper_waybar_state
```

2. Monitor service logs:
```bash
journalctl --user -u chezwizper-waybar.service -f
```

3. Ensure scripts are executable:
```bash
chmod +x ~/.config/waybar/scripts/chezwizper-*.sh
```

### Styling issues

1. Check waybar logs for CSS errors
2. Verify CSS animations are properly closed with `}`
3. Ensure no duplicate CSS class definitions

## Dependencies

- **waybar**: System bar application
- **wl-clipboard**: Wayland clipboard utilities (for clipboard features)
- **systemd**: For service management
- **bash**: Shell scripts
- **jq**: JSON processing (usually included with waybar)

## Contributing

Improvements and bug fixes are welcome! Please test changes with:
1. Different waybar configurations
2. Multiple monitor setups
3. Various Wayland compositors (Hyprland, Sway, etc.)

## License

Same as ChezWizper (MIT)