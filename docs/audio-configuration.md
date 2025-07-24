# Audio Configuration Guide

This guide explains how to configure audio settings and UX sounds in ChezWizper.

## Configuration File

Audio settings are configured in the `~/.config/chezwizper/config.toml` file. Below is an example configuration with all available audio options:

```toml
[audio]
device = "default"
sample_rate = 16000
channels = 1

[whisper]
model = "base"
language = "en"

[ui]
indicator_position = "top-right"
indicator_size = 20
show_notifications = true
layer_shell_anchor = "top | right"
layer_shell_margin = 10

[wayland]
input_method = "wtype"
use_hyprland_ipc = true

[behavior]
auto_paste = true
preserve_clipboard = false
delete_audio_files = true
audio_feedback = true  # Play distinctive beep tones for start/stop/complete
```

## Audio Settings

### [audio] Section

- **`device`**: Audio input device name (default: "default")
  - Use `"default"` for system default microphone
  - Run `arecord -l` to list available audio devices
  
- **`sample_rate`**: Audio sampling rate in Hz (default: 16000)
  - 16000 Hz is recommended for Whisper
  
- **`channels`**: Number of audio channels (default: 1)
  - 1 = mono recording (recommended)
  - 2 = stereo recording

### [behavior] Section

- **`audio_feedback`**: Enable/disable audio feedback sounds (default: true)
  - When enabled, plays distinctive beep tones for:
    - Recording start: Low tone beep
    - Recording stop: Mid tone beep  
    - Transcription complete: High tone beep

## UX Sound Feedback

The audio feedback system provides clear auditory cues for different recording states:

1. **Recording Start**: A low-frequency beep indicates recording has begun
2. **Recording Stop**: A mid-frequency beep confirms recording has stopped
3. **Transcription Complete**: A high-frequency beep signals the transcription is ready and text has been inserted

These sounds help you understand the application state without looking at visual indicators.

## Troubleshooting Audio

### No Sound from Microphone
- Check microphone permissions
- Verify the audio device with `arecord -l`
- Try changing the `device` setting to a specific device name

### Audio Feedback Not Working
- Ensure `audio_feedback = true` in the config
- Check system audio output is not muted
- Verify ALSA is properly installed

### Poor Recording Quality
- Ensure you're using the recommended 16000 Hz sample rate
- Use mono recording (channels = 1) for voice
- Check microphone gain settings in your system audio controls