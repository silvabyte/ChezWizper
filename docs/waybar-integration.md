# Waybar Integration

Add ChezWizper status indicators to your Waybar.

## Setup

### 1. Add ChezWizper Module to Waybar Config

Add the module to your modules list and configuration:

```jsonc
{
  "modules-center": ["custom/chezwizper", "clock"], // Add to any module list
  
  "custom/chezwizper": {
    "exec": "curl -s 'http://127.0.0.1:3737/status?style=waybar'",
    "interval": 1,
    "return-type": "json", 
    "on-click": "curl -X POST http://127.0.0.1:3737/toggle",
    "tooltip": true
  }
}
```

### 2. Restart Waybar

```bash
pkill waybar && waybar
```

## API Response

The endpoint returns JSON with different icons for each state:

- **Idle**: `󰑊` (circle with dot)
- **Recording**: `󰻃` (record button)  
- **Processing**: `󰦖` (spinner)

Example response:
```json
{
  "text": "󰑊",
  "class": "chezwizper-idle", 
  "tooltip": "Press Super+R to record"
}
```

## Customization

Customize icons and tooltips in your ChezWizper config (`~/.config/chezwizper/config.toml`):

```toml
[ui.waybar]
idle_text = "󰍬"                # Use microphone icon
recording_text = "●"            # Use simple filled circle  
processing_text = "⏳"          # Use hourglass emoji
idle_tooltip = "Click to record"
recording_tooltip = "Recording..."
processing_tooltip = "Processing..."
```

CSS styling (optional):
```css
#custom-chezwizper.chezwizper-recording {
  color: #ff6b6b;
  animation: pulse 2s infinite;
}
```

## Troubleshooting

**Module not appearing**: Ensure `"custom/chezwizper"` is added to a module list (modules-left, modules-center, or modules-right).

**Shows "N/A" or error**: Check ChezWizper is running: `curl http://127.0.0.1:3737/status`

**Click not working**: Test the command manually: `curl -X POST http://127.0.0.1:3737/toggle`

