//! Nintendo Switch input handling and Joy-Con abstractions.

use bevy::prelude::*;
use std::collections::HashSet;

/// Plugin for Switch-style input handling.
pub struct SwitchInputPlugin;

impl Plugin for SwitchInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SwitchInput>()
            .add_systems(Update, update_switch_input);
    }
}

/// Nintendo Switch button mappings.
///
/// These correspond to the physical buttons on Joy-Con controllers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchButton {
    // Face buttons (right Joy-Con)
    A,
    B,
    X,
    Y,

    // Shoulder buttons
    L,
    R,
    ZL,
    ZR,

    // Stick buttons
    LeftStick,
    RightStick,

    // System buttons
    Plus,
    Minus,
    Home,
    Capture,

    // D-pad (left Joy-Con)
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,

    // SL/SR buttons (Joy-Con sideways mode)
    SL,
    SR,
}

impl SwitchButton {
    /// Map to standard gamepad button.
    pub fn to_gamepad_button(&self) -> GamepadButton {
        match self {
            SwitchButton::A => GamepadButton::East,
            SwitchButton::B => GamepadButton::South,
            SwitchButton::X => GamepadButton::North,
            SwitchButton::Y => GamepadButton::West,
            SwitchButton::L => GamepadButton::LeftTrigger,
            SwitchButton::R => GamepadButton::RightTrigger,
            SwitchButton::ZL => GamepadButton::LeftTrigger2,
            SwitchButton::ZR => GamepadButton::RightTrigger2,
            SwitchButton::LeftStick => GamepadButton::LeftThumb,
            SwitchButton::RightStick => GamepadButton::RightThumb,
            SwitchButton::Plus => GamepadButton::Start,
            SwitchButton::Minus => GamepadButton::Select,
            SwitchButton::Home => GamepadButton::Mode,
            SwitchButton::Capture => GamepadButton::Mode, // No direct mapping
            SwitchButton::DPadUp => GamepadButton::DPadUp,
            SwitchButton::DPadDown => GamepadButton::DPadDown,
            SwitchButton::DPadLeft => GamepadButton::DPadLeft,
            SwitchButton::DPadRight => GamepadButton::DPadRight,
            SwitchButton::SL => GamepadButton::LeftTrigger,
            SwitchButton::SR => GamepadButton::RightTrigger,
        }
    }

    /// Map keyboard key to Switch button (for development).
    pub fn from_keycode(key: KeyCode) -> Option<Self> {
        match key {
            KeyCode::KeyZ => Some(SwitchButton::B),
            KeyCode::KeyX => Some(SwitchButton::A),
            KeyCode::KeyA => Some(SwitchButton::Y),
            KeyCode::KeyS => Some(SwitchButton::X),
            KeyCode::KeyQ => Some(SwitchButton::L),
            KeyCode::KeyW => Some(SwitchButton::R),
            KeyCode::Digit1 => Some(SwitchButton::ZL),
            KeyCode::Digit2 => Some(SwitchButton::ZR),
            KeyCode::Enter => Some(SwitchButton::Plus),
            KeyCode::Backspace => Some(SwitchButton::Minus),
            KeyCode::ArrowUp => Some(SwitchButton::DPadUp),
            KeyCode::ArrowDown => Some(SwitchButton::DPadDown),
            KeyCode::ArrowLeft => Some(SwitchButton::DPadLeft),
            KeyCode::ArrowRight => Some(SwitchButton::DPadRight),
            _ => None,
        }
    }
}

/// Represents a connected Switch controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchController {
    /// Both Joy-Cons attached or Pro Controller
    Combined,
    /// Left Joy-Con only
    LeftJoyCon,
    /// Right Joy-Con only
    RightJoyCon,
    /// Joy-Con held sideways (single player with one Joy-Con)
    Sideways,
}

/// Resource tracking Switch input state.
#[derive(Debug, Default, Resource)]
pub struct SwitchInput {
    /// Left stick position (-1.0 to 1.0)
    pub left_stick: Vec2,
    /// Right stick position (-1.0 to 1.0)
    pub right_stick: Vec2,
    /// Currently pressed buttons
    pressed: HashSet<SwitchButton>,
    /// Buttons just pressed this frame
    just_pressed: HashSet<SwitchButton>,
    /// Buttons just released this frame
    just_released: HashSet<SwitchButton>,
}

impl SwitchInput {
    /// Check if a button is currently pressed.
    pub fn pressed(&self, button: SwitchButton) -> bool {
        self.pressed.contains(&button)
    }

    /// Check if a button was just pressed this frame.
    pub fn just_pressed(&self, button: SwitchButton) -> bool {
        self.just_pressed.contains(&button)
    }

    /// Check if a button was just released this frame.
    pub fn just_released(&self, button: SwitchButton) -> bool {
        self.just_released.contains(&button)
    }

    /// Get movement direction from left stick or D-pad.
    pub fn movement(&self) -> Vec2 {
        let mut dir = self.left_stick;

        // Also check D-pad
        if self.pressed(SwitchButton::DPadUp) {
            dir.y += 1.0;
        }
        if self.pressed(SwitchButton::DPadDown) {
            dir.y -= 1.0;
        }
        if self.pressed(SwitchButton::DPadLeft) {
            dir.x -= 1.0;
        }
        if self.pressed(SwitchButton::DPadRight) {
            dir.x += 1.0;
        }

        dir.clamp_length_max(1.0)
    }
}

/// System to update Switch input from keyboard and gamepads.
fn update_switch_input(
    mut switch_input: ResMut<SwitchInput>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
) {
    // Clear frame-specific state
    switch_input.just_pressed.clear();
    switch_input.just_released.clear();

    // Update from keyboard (development mode)
    for key in keyboard.get_just_pressed() {
        if let Some(button) = SwitchButton::from_keycode(*key) {
            switch_input.pressed.insert(button);
            switch_input.just_pressed.insert(button);
        }
    }

    for key in keyboard.get_just_released() {
        if let Some(button) = SwitchButton::from_keycode(*key) {
            switch_input.pressed.remove(&button);
            switch_input.just_released.insert(button);
        }
    }

    // Keyboard movement (WASD or IJKL)
    let mut kb_movement = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::KeyI) {
        kb_movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::KeyK) {
        kb_movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::KeyJ) {
        kb_movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::KeyL) {
        kb_movement.x += 1.0;
    }

    // Update from gamepads
    for gamepad in gamepads.iter() {
        // Left stick
        let left_x = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
        let left_y = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);
        if left_x.abs() > 0.1 || left_y.abs() > 0.1 {
            switch_input.left_stick = Vec2::new(left_x, left_y);
        }

        // Right stick
        let right_x = gamepad.get(GamepadAxis::RightStickX).unwrap_or(0.0);
        let right_y = gamepad.get(GamepadAxis::RightStickY).unwrap_or(0.0);
        if right_x.abs() > 0.1 || right_y.abs() > 0.1 {
            switch_input.right_stick = Vec2::new(right_x, right_y);
        }
    }

    // Combine keyboard and gamepad for left stick
    if kb_movement.length() > 0.1 {
        switch_input.left_stick = kb_movement.normalize();
    } else if switch_input.left_stick.length() < 0.1 {
        switch_input.left_stick = Vec2::ZERO;
    }
}
