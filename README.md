# ChezWizper

Voice transcription tool for Wayland/Hyprland. Press a keybind to toggle recording, get automatic transcription via Whisper, and inject text into the focused application.

📚 **[View Documentation](./docs/index.md)** - Detailed guides and configuration

## Quick Install (Omarchy + Arch Linux)

```bash
git clone https://github.com/silvabyte/ChezWizper.git
cd ChezWizper
./scripts/install.sh
```

This automatically installs dependencies, builds ChezWizper with optimized Whisper, sets up services, and configures keybinds.

**After installation:**
1. Start the service: `systemctl --user enable --now chezwizper.service`
2. Add to Hyprland config: `bind = SUPER, R, exec, curl -X POST http://127.0.0.1:3737/toggle`
3. Press Super+R to start recording!

## Features

- 🎤 Keybind-activated voice recording
- 🔴 Visual recording indicators  
- 🎯 Automatic text injection into focused apps
- 📋 Intelligent clipboard fallback
- ⚡ Optimized for Wayland/Hyprland
- 🔧 Configurable via TOML

## Manual Installation

For other distributions or custom setups, see the [Installation Guide](./docs/installation.md).

## Configuration

Default config at `~/.config/chezwizper/config.toml`. See [Configuration Guide](./docs/audio-configuration.md) for details.

## Troubleshooting

- **Recording issues**: Check [Audio Configuration](./docs/audio-configuration.md)
- **Text injection fails**: See [Text Injection Setup](./docs/text-injection-setup.md)
- **Service problems**: View logs with `journalctl --user -u chezwizper.service -f`

## Updates

```bash
chezwizper-update                    # Update ChezWizper
chezwizper-update --whisper          # Update both ChezWizper and Whisper
chezwizper-update --check            # Check for updates
```

## License

MIT