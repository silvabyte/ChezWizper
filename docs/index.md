# ChezWizper Documentation

Welcome to the ChezWizper documentation. This directory contains detailed guides for configuring and using ChezWizper.

## Available Documentation

### Installation & Setup

- [Installation Guide](./installation.md) - Complete installation instructions for all platforms
- [Text Injection Setup](./text-injection-setup.md) - Set up automatic text injection methods for different environments

### Configuration Guides

- [Audio Configuration](./audio-configuration.md) - Configure audio input devices, sampling settings, and UX sound feedback

### Development

ChezWizper includes a Makefile for common development tasks:

```bash
make help       # Show all available commands
make build      # Build debug binary
make release    # Build optimized release
make test       # Run tests
make lint       # Run clippy linter
make fmt        # Check formatting
make start      # Enable and start service
make logs       # Show service logs
make status     # Check service status
```

### Coming Soon

- Whisper Models Guide - Choosing and configuring different Whisper models
- Keyboard Shortcuts - Setting up custom keybindings
- Troubleshooting Guide - Common issues and solutions
- API Reference - HTTP API endpoints and usage

## Quick Links

- [Main README](../README.md) - Project overview and quick start
- [GitHub Repository](https://github.com/silvabyte/ChezWizper) - Source code and issue tracker