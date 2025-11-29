# bevy_switch 🦀🎮

A Rust crate to help port [Bevy](https://bevyengine.org/) games to Nintendo Switch using emulators - no NDAs required.

## Background

As noted in the [Bevy Cheatbook](https://bevy-cheatbook.github.io/platforms.html):

> The Rust Programming Language aims to make Nintendo Switch a supported target, but that work is in its early days and has not progressed enough to be useful for Bevy yet. It should be possible to work on Nintendo Switch support in the open, without NDAs, using emulators.

This crate aims to bridge that gap by providing:
- Platform abstractions for Switch vs Desktop
- Joy-Con input mapping and abstractions
- Window/display management for Switch resolutions
- Utilities for cross-platform game development

## Status

**Highly experimental.** The `aarch64-nintendo-switch-freestanding` target is Tier 3 in Rust, meaning:
- Not automatically tested
- Limited standard library support
- May not build at any given time

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy_switch = { git = "https://github.com/ibrahimcesar/rust-on-nintendo" }
```

## Usage

```rust
use bevy::prelude::*;
use bevy_switch::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(switch_window("My Game")),
            ..default()
        }))
        .add_plugins(SwitchPlugin)
        .add_systems(Update, handle_input)
        .run();
}

fn handle_input(switch_input: Res<SwitchInput>) {
    // Unified input from keyboard, gamepad, or Joy-Cons
    let movement = switch_input.movement();

    if switch_input.just_pressed(SwitchButton::A) {
        // Handle A button (or mapped keyboard key)
    }
}
```

## Features

- `desktop` (default) - Build for desktop development/testing
- `switch` - Build for Nintendo Switch target

## Project Structure

```
bevy_switch/
├── src/
│   ├── lib.rs          # Main plugin and prelude
│   ├── platform.rs     # Platform detection & config
│   ├── input.rs        # Joy-Con input abstractions
│   └── window.rs       # Display management
├── examples/
│   └── crab_crossing.rs  # Demo game
└── ...
```

## Running the Example

```bash
# Run Crab Crossing demo
cargo run --example crab_crossing
```

## Controls

| Switch | Keyboard | Action |
|--------|----------|--------|
| Left Stick | WASD / IJKL | Movement |
| D-Pad | Arrow Keys | Movement |
| A | X | Confirm |
| B | Z | Cancel |
| L/R | Q/W | Shoulders |
| ZL/ZR | 1/2 | Triggers |
| + | Enter | Start/Plus |
| - | Backspace | Select/Minus |

## Prerequisites

### Rust Toolchain

```bash
# Nightly required for build-std
rustup default nightly
rustup component add rust-src

# Optional: cargo-nx for Switch builds
cargo install cargo-nx --git https://github.com/aarch64-switch-rs/cargo-nx
```

### Emulators for Testing

Since Yuzu and Ryujinx were shut down by Nintendo in 2024, use community forks:

- **[Ryubing](https://github.com/Ryubing/Ryujinx)** - Fork of Ryujinx, most stable
- **[Citron](https://github.com/pkgforge-dev/Citron)** - Fork of Yuzu, good performance
- **[Sudachi](https://github.com/sudachi-emu/sudachi)** - Cross-platform Yuzu fork

## API Overview

### `SwitchPlugin`
Main plugin - adds input handling, window management, and platform detection.

### `SwitchInput`
Resource for unified input across keyboard/gamepad:
- `movement()` - Get movement vector from stick or D-pad
- `pressed(button)` / `just_pressed(button)` - Check button state
- `left_stick` / `right_stick` - Raw stick positions

### `SwitchConfig`
Resource for platform configuration:
- `platform` - Current platform (Desktop/SwitchDocked/SwitchHandheld)
- `resolution` - Target resolution
- `display_mode` - Docked/Handheld/Tabletop

### `SwitchButton`
Enum mapping all Joy-Con buttons with keyboard equivalents.

## Roadmap

- [x] Platform abstraction layer
- [x] Joy-Con button mapping
- [x] Unified input system
- [x] Window/resolution helpers
- [x] Investigate Bevy `no_std` compatibility (see [docs/NO_STD_ANALYSIS.md](docs/NO_STD_ANALYSIS.md))
- [ ] Test with actual Switch emulator homebrew
- [ ] Switch-specific rendering backend
- [ ] Audio subsystem abstraction
- [ ] File I/O abstraction
- [ ] HD Rumble support

## Bevy `no_std` Status

**Good news!** As of Bevy 0.16 (April 2025), many core crates support `no_std`:

| Status | Crates |
|--------|--------|
| ✅ Ready | `bevy_ecs`, `bevy_app`, `bevy_math`, `bevy_input`, `bevy_transform`, `bevy_color`, `bevy_state`, `bevy_time`, `bevy_hierarchy`, `bevy_reflect` |
| ❌ Not Planned | `bevy_render`, `bevy_audio`, `bevy_asset`, `bevy_winit` (platform-specific) |

This means game logic using ECS can run on Switch! But rendering/audio need custom backends.

See [docs/NO_STD_ANALYSIS.md](docs/NO_STD_ANALYSIS.md) for full analysis.

## Challenges

### Graphics
Switch uses NVN (NVIDIA proprietary) or OpenGL ES. Bevy uses wgpu (Vulkan/Metal/DX12/WebGPU). A custom backend would be needed.

### no_std
Switch target is `freestanding`. As of Bevy 0.16+, core ECS is `no_std` compatible! Rendering still requires `std`.

### Audio
Switch has its own audio subsystem requiring a custom Bevy backend.

## Resources

- [Rust aarch64-nintendo-switch-freestanding](https://doc.rust-lang.org/rustc/platform-support/aarch64-nintendo-switch-freestanding.html)
- [aarch64-switch-rs](https://github.com/aarch64-switch-rs) - Rust Switch homebrew ecosystem
- [nx crate](https://github.com/aarch64-switch-rs/nx) - Switch userland library
- [cargo-nx](https://github.com/aarch64-switch-rs/cargo-nx) - Build tool
- [Bevy Engine](https://bevyengine.org/)

## Legal

This project is for educational purposes. It uses only open-source tools and does not require Nintendo NDAs or proprietary SDKs.

## License

MIT
