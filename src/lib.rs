//! # switchbrew_bevy
//!
//! A crate to help port Bevy games to Nintendo Switch using emulators.
//!
//! This library provides platform abstractions, input handling, and utilities
//! to make Bevy games run on Nintendo Switch without requiring NDAs.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use switchbrew_bevy::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(SwitchPlugin)
//!         .run();
//! }
//! ```
//!
//! ## Features
//!
//! - `desktop` (default): Build for desktop development/testing
//! - `switch`: Build for Nintendo Switch target

pub mod input;
pub mod platform;
pub mod window;

/// Prelude module - import commonly used items
pub mod prelude {
    pub use crate::input::{SwitchButton, SwitchController, SwitchInput};
    pub use crate::platform::{Platform, SwitchConfig};
    pub use crate::window::{handheld_window, switch_window, SwitchDisplay, SwitchWindowPlugin};
    pub use crate::SwitchPlugin;
}

use bevy::prelude::*;

/// Main plugin that sets up all Switch compatibility features.
///
/// Add this plugin to your Bevy app to enable Switch compatibility:
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use switchbrew_bevy::SwitchPlugin;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(SwitchPlugin)
///     .run();
/// ```
pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(platform::SwitchConfig::default())
            .add_plugins(window::SwitchWindowPlugin)
            .add_plugins(input::SwitchInputPlugin)
            .add_systems(Startup, log_platform_info);
    }
}

fn log_platform_info(config: Res<platform::SwitchConfig>) {
    info!("switchbrew_bevy initialized");
    info!("Platform: {}", config.platform.name());
    info!(
        "Resolution: {}x{}",
        config.resolution.0, config.resolution.1
    );
}
