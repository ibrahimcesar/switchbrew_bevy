# Bevy `no_std` Compatibility Analysis for Nintendo Switch

## Executive Summary

**Good news:** As of Bevy 0.16 (April 2025), significant `no_std` support has been merged. Many core crates now work without the standard library, making a Switch port theoretically possible.

**Challenge:** The rendering, audio, and asset systems remain `std`-only, requiring custom implementations for Switch.

## Bevy `no_std` Status (as of 0.16+)

### Crates with `no_std` Support ✅

| Crate | Status | Notes |
|-------|--------|-------|
| bevy_ptr | ✅ Done | Core pointer utilities |
| bevy_utils | ✅ Done | HashMaps, Instant type |
| bevy_tasks | ✅ Done | Task scheduling |
| bevy_ecs | ✅ Done | Entity Component System |
| bevy_app | ✅ Done | App & Plugin abstractions |
| bevy_reflect | ✅ Done | Runtime reflection |
| bevy_math | ✅ Done | glam math types |
| bevy_color | ✅ Done | Color types |
| bevy_transform | ✅ Done | Transform components |
| bevy_hierarchy | ✅ Done | Parent/child relationships |
| bevy_input | ✅ Done | Input abstractions |
| bevy_state | ✅ Done | State machines |
| bevy_time | ✅ Done | Time management |
| bevy_window | ✅ Done | Window abstractions |
| bevy_diagnostic | ✅ Done | Diagnostics |
| bevy_a11y | ✅ Done | Accessibility |

### Crates Blocked/Not Planned ❌

| Crate | Status | Blocker |
|-------|--------|---------|
| bevy_asset | ❌ Blocked | Filesystem operations |
| bevy_render | ❌ Not planned | Platform-specific |
| bevy_pbr | ❌ Not planned | Depends on render |
| bevy_sprite | ❌ Not planned | Depends on render |
| bevy_ui | ❌ Not planned | Depends on render |
| bevy_text | ❌ Not planned | Depends on render |
| bevy_audio | ❌ Not planned | Platform-specific |
| bevy_winit | ❌ Not planned | Platform-specific |
| bevy_gilrs | ❌ Not planned | Platform-specific |
| bevy_gltf | ❌ Not planned | Depends on asset |

## Requirements for `no_std` Bevy

1. **Rust nightly** - Required for `build-std`
2. **`alloc` crate** - Dynamic memory allocation
3. **32-bit or higher platform** - Safety requirements in bevy_ecs
4. **Atomic CAS support** - Required for ECS internals

### Feature Flags for `no_std`

```toml
[dependencies]
bevy = { version = "0.16", default-features = false, features = [
    "libm",              # Math without std
    "critical-section",  # Synchronization primitive
] }
```

### Minimal Plugin Set for Embedded

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(TaskPoolPlugin::default())
        .add_plugins(TypeRegistrationPlugin)
        .add_plugins(FrameCountPlugin)
        .add_plugins(TimePlugin)
        .add_plugins(ScheduleRunnerPlugin::default())
        // Your game systems...
        .run();
}
```

## Nintendo Switch Specifics

### Target: `aarch64-nintendo-switch-freestanding`

- **Tier 3** in Rust (not automatically tested)
- **Horizon OS** microkernel
- Requires custom runtime (no std)
- Has `alloc` support via custom allocator

### Available via `nx` crate (aarch64-switch-rs)

| Feature | Status |
|---------|--------|
| Memory allocation | ✅ Custom allocator |
| IPC/Services | ✅ Implemented |
| Input (HID) | 🔄 In progress |
| Graphics (NVN) | ❌ Not implemented |
| Audio | ❌ Not implemented |
| Filesystem | 🔄 Partial |

## Architecture for Switch Port

```
┌─────────────────────────────────────────────────┐
│                   Game Code                      │
│            (Platform-agnostic ECS)               │
├─────────────────────────────────────────────────┤
│                 bevy_switch                      │
│    (This crate - abstractions & plugins)         │
├─────────────────┬───────────────────────────────┤
│   Desktop HAL   │      Switch HAL               │
│   (bevy_winit)  │   (custom via nx crate)       │
│   (bevy_render) │   (custom NVN renderer)       │
│   (bevy_audio)  │   (custom audio backend)      │
├─────────────────┴───────────────────────────────┤
│              Bevy Core (no_std)                  │
│  bevy_ecs, bevy_app, bevy_math, bevy_input...   │
└─────────────────────────────────────────────────┘
```

## Recommended Approach

### Phase 1: Core ECS on Switch (Achievable Now)
1. Use `bevy_ecs`, `bevy_app`, `bevy_math` in `no_std` mode
2. Integrate with `nx` crate for Switch syscalls
3. Implement custom allocator wrapper
4. No graphics - console/debug output only

### Phase 2: Custom Renderer (Major Effort)
Options:
- **OpenGL ES** via Mesa (if available on Switch homebrew)
- **NVN** (Nintendo's proprietary API) - requires reverse engineering
- **Software renderer** - slow but portable

### Phase 3: Full Game Support (Long-term)
- Custom asset loading from Switch filesystem
- Audio via Switch audio services
- HD Rumble support
- Touch screen input

## Code Example: Minimal `no_std` Bevy for Switch

```rust
#![no_std]
#![no_main]

extern crate alloc;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

// Switch entry point (via nx crate)
#[no_mangle]
pub extern "C" fn main() -> ! {
    // Initialize Switch services
    nx::init();

    // Create minimal Bevy app
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, game_loop)
        .run();

    loop {}
}

fn setup(mut commands: Commands) {
    commands.spawn(Player { x: 0.0, y: 0.0 });
}

fn game_loop(/* systems */) {
    // Game logic using ECS
}

#[derive(Component)]
struct Player {
    x: f32,
    y: f32,
}
```

## Existing Work & Resources

- [Bevy no_std Tracking Issue](https://github.com/bevyengine/bevy/issues/15460)
- [ESP32 Spooky Maze Game](https://github.com/georgik/esp32-spooky-maze-game) - Real `no_std` Bevy example
- [aarch64-switch-rs/nx](https://github.com/aarch64-switch-rs/nx) - Switch userland library
- [Espressif Bevy ECS Guide](https://developer.espressif.com/blog/2025/04/bevy-ecs-on-esp32-with-rust-no-std/)

## Conclusion

**Feasibility: Moderate to High for ECS, Low for Full Game**

- ✅ Bevy's core (ECS, App, Math, Input) now supports `no_std`
- ✅ Switch has Rust target and homebrew ecosystem
- ⚠️ No existing graphics backend for Switch in Rust
- ⚠️ Would need custom renderer (significant undertaking)
- ⚠️ Asset loading needs reimplementation

**Recommended Next Steps:**
1. Create a minimal `no_std` Bevy app that compiles for Switch
2. Test on Switch emulator (Ryubing/Citron)
3. Implement basic software renderer as proof of concept
4. Explore NVN or OpenGL ES for hardware acceleration
