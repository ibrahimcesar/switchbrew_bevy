//! Window and display management for Nintendo Switch.

use bevy::prelude::*;

use crate::platform::{DisplayMode, SwitchConfig};

/// Plugin for Switch window/display management.
pub struct SwitchWindowPlugin;

impl Plugin for SwitchWindowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SwitchDisplay>()
            .add_systems(Update, handle_display_mode_change);
    }
}

/// Display configuration resource.
#[derive(Debug, Clone, Resource)]
pub struct SwitchDisplay {
    /// Current display mode.
    pub mode: DisplayMode,
    /// Current resolution.
    pub resolution: (u32, u32),
    /// Whether vsync is enabled.
    pub vsync: bool,
}

impl Default for SwitchDisplay {
    fn default() -> Self {
        Self {
            mode: DisplayMode::Docked,
            resolution: (1920, 1080),
            vsync: true,
        }
    }
}

impl SwitchDisplay {
    /// Create display config for handheld mode.
    pub fn handheld() -> Self {
        Self {
            mode: DisplayMode::Handheld,
            resolution: (1280, 720),
            vsync: true,
        }
    }

    /// Get resolution as f32 tuple.
    pub fn resolution_f32(&self) -> (f32, f32) {
        (self.resolution.0 as f32, self.resolution.1 as f32)
    }

    /// Get aspect ratio.
    pub fn aspect_ratio(&self) -> f32 {
        self.resolution.0 as f32 / self.resolution.1 as f32
    }
}

/// System to handle display mode changes.
fn handle_display_mode_change(
    config: Res<SwitchConfig>,
    mut display: ResMut<SwitchDisplay>,
    mut windows: Query<&mut Window>,
) {
    if !config.is_changed() {
        return;
    }

    display.mode = config.display_mode;
    display.resolution = config.resolution;

    // Update window if on desktop
    for mut window in &mut windows {
        window
            .resolution
            .set(config.resolution.0 as f32, config.resolution.1 as f32);
    }
}

/// Helper to create Switch-compatible window settings.
pub fn switch_window(title: impl Into<String>) -> Window {
    Window {
        title: title.into(),
        resolution: (1920, 1080).into(),
        resizable: false,
        ..default()
    }
}

/// Helper to create handheld-mode window settings.
pub fn handheld_window(title: impl Into<String>) -> Window {
    Window {
        title: title.into(),
        resolution: (1280, 720).into(),
        resizable: false,
        ..default()
    }
}
