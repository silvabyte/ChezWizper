# Text Injection Setup Guide

ChezWizper supports multiple methods for automatically injecting transcribed text into your applications. This guide explains the available methods and how to set them up on different Linux distributions and desktop environments.

## Automatic Method Selection

ChezWizper automatically detects the best available text injection method based on:

1. **User preference** (if specified in config)
2. **Available tools** on your system  
3. **Desktop environment** (X11 vs Wayland)

The detection priority is:
- If user specifies a method in config, try that first (with fallback if it fails)
- Otherwise, try ydotool (most reliable on Wayland)
- On Wayland systems with wl-copy available, prefer clipboard method
- Try wtype (may fall back to clipboard if it fails)
- Final fallback to clipboard-only mode

## Supported Methods

### 1. ydotool (Recommended for Wayland)

**Best for**: KDE Plasma, GNOME, and other Wayland compositors

**Installation**:
```bash
# Arch Linux
sudo pacman -S ydotool

# Ubuntu/Debian  
sudo apt install ydotool

# Fedora
sudo dnf install ydotool
```

**Setup**:
```bash
# Enable and start the daemon
systemctl --user enable --now ydotool.service

# Set environment variable (add to your shell profile)
export YDOTOOL_SOCKET="/run/user/$(id -u)/.ydotool_socket"
```

**Configuration**:
```toml
[wayland]
input_method = "ydotool"
```

### 2. wtype (Limited Wayland Support)

**Best for**: Sway and some other Wayland compositors

**Note**: Does NOT work reliably with KDE Plasma or GNOME due to security restrictions. ChezWizper will automatically fall back to clipboard paste if wtype fails.

**Installation**:
```bash
# Arch Linux
sudo pacman -S wtype

# Ubuntu/Debian
sudo apt install wtype

# Build from source if not in repos
git clone https://github.com/atx/wtype
cd wtype && make && sudo make install
```

**Configuration**:
```toml
[wayland] 
input_method = "wtype"
```

### 3. Clipboard + Paste (Universal Fallback)

**Best for**: Any environment where direct injection fails

**Requirements**: 
- **Wayland**: `wl-copy` and `wl-paste` (from `wl-clipboard`)
- **X11**: `xclip` or `xsel`

**Installation**:
```bash
# Wayland clipboard tools
sudo pacman -S wl-clipboard     # Arch
sudo apt install wl-clipboard   # Ubuntu/Debian

# X11 clipboard tools  
sudo pacman -S xclip xsel       # Arch
sudo apt install xclip xsel     # Ubuntu/Debian
```

**How it works**:
1. Copies text to system clipboard
2. Verifies clipboard content was set correctly (with retry)
3. Simulates Ctrl+V keypress to paste using available tools (ydotool, wtype, or xdotool)
4. If paste simulation fails, text remains in clipboard for manual paste

**Note**: ChezWizper automatically falls back to this method if direct text injection (ydotool/wtype) fails.

## Distribution-Specific Notes

### Arch Linux / Manjaro
- All tools available in official repos
- ydotool works best with proper systemd service setup

### Ubuntu / Debian
- May need to enable universe repository for some tools
- Consider using snap packages for latest versions

### Fedora / CentOS
- Some tools may need EPEL repository
- SELinux may need configuration for ydotool

### NixOS
- Add packages to your configuration.nix
- systemd service needs to be declared explicitly

## Desktop Environment Compatibility

| Method | KDE Plasma | GNOME | Sway | Hyprland | i3/X11 |
|--------|------------|-------|------|----------|---------|
| ydotool | ✅ Best | ✅ Good | ✅ Good | ✅ Good | ✅ Good |
| wtype | ❌ No | ❌ No | ✅ Good | ✅ Good | ❌ N/A |
| Clipboard | ✅ Fallback | ✅ Fallback | ✅ Fallback | ✅ Fallback | ✅ Good |
| xdotool | ❌ N/A | ❌ N/A | ❌ N/A | ❌ N/A | ✅ Good |

## Troubleshooting

### ydotool Issues

**"Permission denied" or "No such device"**:
```bash
# Check if daemon is running
systemctl --user status ydotool

# Restart the service  
systemctl --user restart ydotool

# Check socket exists
ls -la /run/user/$(id -u)/.ydotool_socket
```

**"ydotool: uinput main"**:
```bash
# Load uinput module
sudo modprobe uinput

# Make it permanent
echo 'uinput' | sudo tee /etc/modules-load.d/uinput.conf
```

### wtype Issues

**"Compositor does not support the virtual keyboard protocol"**:
- This means your compositor (KDE/GNOME) doesn't support wtype
- Use ydotool or clipboard method instead

### Clipboard Issues

**Text copies but doesn't paste**:
- Check if the target application accepts Ctrl+V
- Some applications may need focus or special paste handling
- Try manual Ctrl+V to verify clipboard contents

## Configuration Examples

### Minimal (auto-detection)
```toml
[wayland]
# input_method not specified - auto-detect best method
```

### Explicit ydotool
```toml
[wayland]
input_method = "ydotool"
```

### Force clipboard mode
```toml
[wayland]
# To force clipboard-only mode, set an invalid method name
# or simply don't install ydotool/wtype
input_method = "clipboard"  # Will fall back to clipboard
```

### Disable auto-paste
```toml
[behavior]
auto_paste = false  # Only copy to clipboard, no automatic paste
```

## Advanced Configuration

### Custom ydotool socket
```bash
export YDOTOOL_SOCKET="/tmp/my-ydotool-socket"
```

### Multiple display setup
Some tools may need display specification:
```bash
export DISPLAY=:0  # For X11 tools on specific display
```

### Security considerations
- ydotool requires access to `/dev/uinput`
- Some distributions may need additional permissions
- Consider security implications of automated input

## Testing Your Setup

Test text injection manually:
```bash
# Test ydotool
echo "Hello World" | wl-copy && ydotool key 29:1 47:1 47:0 29:0

# Test wtype  
echo "Hello World" | wl-copy && wtype -M ctrl -P v -m ctrl -p v

# Test clipboard tools
echo "Hello World" | wl-copy && wl-paste
```

Run ChezWizper with verbose logging:
```bash
./chezwizper --verbose
```

This will show which text injection method was selected and any errors encountered.