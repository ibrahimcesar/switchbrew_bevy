//! Platform detection and configuration for Nintendo Switch.

use bevy::prelude::*;

/// Target platform for the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Platform {
    /// Desktop development mode (Windows, macOS, Linux)
    #[default]
    Desktop,
    /// Nintendo Switch in docked mode (1920x1080)
    SwitchDocked,
    /// Nintendo Switch in handheld mode (1280x720)
    SwitchHandheld,
}

impl Platform {
    /// Detect current platform at compile time.
    pub const fn current() -> Self {
        #[cfg(feature = "switch")]
        {
            Platform::SwitchDocked
        }
        #[cfg(not(feature = "switch"))]
        {
            Platform::Desktop
        }
    }

    /// Get human-readable name for the platform.
    pub const fn name(&self) -> &'static str {
        match self {
            Platform::Desktop => "Desktop",
            Platform::SwitchDocked => "Nintendo Switch (Docked)",
            Platform::SwitchHandheld => "Nintendo Switch (Handheld)",
        }
    }

    /// Get the native resolution for this platform.
    pub const fn resolution(&self) -> (u32, u32) {
        match self {
            Platform::Desktop => (1920, 1080),
            Platform::SwitchDocked => (1920, 1080),
            Platform::SwitchHandheld => (1280, 720),
        }
    }

    /// Check if running on Switch (any mode).
    pub const fn is_switch(&self) -> bool {
        matches!(self, Platform::SwitchDocked | Platform::SwitchHandheld)
    }
}

/// Display mode for the Switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Resource)]
pub enum DisplayMode {
    /// Docked mode - TV output at 1080p
    #[default]
    Docked,
    /// Handheld mode - built-in screen at 720p
    Handheld,
    /// Tabletop mode - using kickstand, same as handheld resolution
    Tabletop,
}

impl DisplayMode {
    /// Get resolution for this display mode.
    pub const fn resolution(&self) -> (u32, u32) {
        match self {
            DisplayMode::Docked => (1920, 1080),
            DisplayMode::Handheld | DisplayMode::Tabletop => (1280, 720),
        }
    }
}

/// Configuration resource for Switch compatibility.
#[derive(Debug, Clone, Resource)]
pub struct SwitchConfig {
    /// Current platform.
    pub platform: Platform,
    /// Current display mode.
    pub display_mode: DisplayMode,
    /// Target resolution.
    pub resolution: (u32, u32),
    /// Target frame rate (Switch typically targets 30 or 60 FPS).
    pub target_fps: u32,
    /// Enable performance profiling overlay.
    pub show_perf_overlay: bool,
}

impl Default for SwitchConfig {
    fn default() -> Self {
        let platform = Platform::current();
        let display_mode = DisplayMode::default();
        Self {
            platform,
            display_mode,
            resolution: platform.resolution(),
            target_fps: 60,
            show_perf_overlay: false,
        }
    }
}

impl SwitchConfig {
    /// Create config for docked mode.
    pub fn docked() -> Self {
        Self {
            platform: Platform::SwitchDocked,
            display_mode: DisplayMode::Docked,
            resolution: (1920, 1080),
            ..Default::default()
        }
    }

    /// Create config for handheld mode.
    pub fn handheld() -> Self {
        Self {
            platform: Platform::SwitchHandheld,
            display_mode: DisplayMode::Handheld,
            resolution: (1280, 720),
            ..Default::default()
        }
    }

    /// Set target frame rate.
    pub fn with_fps(mut self, fps: u32) -> Self {
        self.target_fps = fps;
        self
    }
}
