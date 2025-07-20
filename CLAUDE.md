# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Hyprlux is a Rust-based Hyprland utility that automatically switches between GPU shaders based on window focus and time. It provides two main features:
- Night light (blue light filter) - automatic based on location/sunset or manual time ranges
- Vibrance enhancement - toggles digital vibrance for specific applications (useful for gaming)

## Commands

### Build and Development
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version
- `cargo run` - Run the application (requires Hyprland)
- `cargo test` - Run unit tests
- `cargo fmt` - Format code (enforced in CI)
- `cargo clippy` - Run linting (recommended before commits)

### Installation
- `cargo install hyprlux` - Install from crates.io
- Available through Nix flakes and AUR

## Architecture

### Core Structure
- `main.rs` - Event loop, config hot-reloading, window change handling
- `config.rs` - Configuration loading and TOML parsing
- `shaders/` - Shader system implementation
  - `shader.rs` - Core shader trait and Hyprland integration
  - `night_light.rs` - Blue light filter with sunset/sunrise or manual timing
  - `vibrance.rs` - Digital vibrance for specific windows
- `utils.rs` - Time utilities and helper functions

### Event-Driven Architecture
The application uses Hyprland's event system to respond to window changes:
1. Listens for `active_window_changed` events
2. Evaluates which shader should apply based on current window and time
3. Applies/removes shaders via Hyprland's `decoration:screen_shader` keyword
4. Hot-reloads configuration when config file changes (if enabled)

### Shader System
- Shaders implement the `Shader` trait with `should_apply()`, `get()`, and `hash()` methods
- Shaders are compiled to temporary files and applied via Hyprland keywords
- Priority: vibrance shaders override night light when both conditions match
- Each shader generates a unique hash for efficient comparison

### Configuration
- TOML-based configuration with hot-reload support
- Config lookup priority: CLI argument → `$XDG_CONFIG_HOME/hypr/hyprlux.toml` → `/etc/hyprlux/config.toml`
- Supports both location-based (lat/lon) and manual time-based night light
- Vibrance configs match windows by class and/or title with configurable strength

### Dependencies
- `hyprland` crate for Wayland compositor integration
- `notify` for file watching (config hot-reload)
- `chrono` for time handling
- `sunrise` crate for sunset/sunrise calculations
- `serde`/`toml` for configuration parsing

## Testing
- Unit tests exist in `utils.rs` for time and range utilities
- Test with `cargo test --verbose`
- Manual testing requires running Hyprland compositor