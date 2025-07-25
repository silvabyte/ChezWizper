.PHONY: help build release check test clean install run logs start restart stop status update update-all lint fmt fix

# Default target
help:
	@echo "🦀 ChezWizper Development Commands"
	@echo ""
	@echo "  make build    - Build debug binary"
	@echo "  make release  - Build optimized release binary"
	@echo "  make check    - Run cargo check"
	@echo "  make test     - Run tests"
	@echo "  make lint     - Run clippy linter"
	@echo "  make fmt      - Check formatting"
	@echo "  make fix      - Fix formatting and simple lint issues"
	@echo ""
	@echo "  make install  - Install ChezWizper (Arch Linux)"
	@echo "  make run      - Run ChezWizper directly"
	@echo "  make start    - Enable and start service"
	@echo "  make logs     - Show service logs"
	@echo "  make restart  - Restart service"
	@echo "  make stop     - Stop service"
	@echo "  make status   - Check service status"
	@echo ""
	@echo "  make update   - Update ChezWizper"
	@echo "  make update-all - Update ChezWizper and Whisper"
	@echo ""
	@echo "  make clean    - Clean build artifacts"

# Build commands
build:
	cargo build

release:
	cargo build --release

check:
	cargo check

test:
	cargo test

# Code quality
lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt -- --check

fix:
	cargo fmt
	cargo fix --allow-dirty --allow-staged

# Installation and service management
install:
	./scripts/install.sh

run:
	RUST_LOG=info cargo run --release

logs:
	journalctl --user -u chezwizper.service -f

start:
	systemctl --user enable --now chezwizper.service
	@echo "✓ Service enabled and started"

restart:
	systemctl --user restart chezwizper.service
	@echo "✓ Service restarted"

stop:
	systemctl --user stop chezwizper.service
	@echo "✓ Service stopped"

status:
	@systemctl --user is-active chezwizper.service >/dev/null 2>&1 && echo "✓ Service is running" || echo "✗ Service is not running"
	@curl -s http://127.0.0.1:3737/status 2>/dev/null | python3 -m json.tool || echo "✗ API not responding"

# Update commands
update:
	@if command -v chezwizper-update >/dev/null 2>&1; then \
		chezwizper-update; \
	else \
		echo "Update command not found. Try: hash -r && chezwizper-update"; \
	fi

update-all:
	@if command -v chezwizper-update >/dev/null 2>&1; then \
		chezwizper-update --whisper; \
	else \
		echo "Update command not found. Try: hash -r && chezwizper-update --whisper"; \
	fi

# Cleanup
clean:
	cargo clean
	rm -f /tmp/chezwizper_*.wav