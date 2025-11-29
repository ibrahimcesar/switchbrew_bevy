//! Crab Crossing - Example game using switchbrew_bevy
//!
//! A simple 3D game demonstrating switchbrew_bevy capabilities.
//! Run with: `cargo run --example crab_crossing`

use bevy::prelude::*;
use switchbrew_bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(switch_window("Crab Crossing")),
            ..default()
        }))
        .add_plugins(SwitchPlugin)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, rotate_cube))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct RotatingCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a crab-colored cube as the player
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.5, 1.2))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.3, 0.2))), // Crab red!
        Transform::from_xyz(0.0, 0.25, 0.0),
        Player,
        RotatingCube,
    ));

    // Spawn a ground plane (the road to cross)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.25))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn road markings
    for i in -4..=4 {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.3, 0.01, 1.5))),
            MeshMaterial3d(materials.add(Color::srgb(0.9, 0.9, 0.8))),
            Transform::from_xyz(i as f32 * 2.0, 0.01, 0.0),
        ));
    }

    // Spawn a light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 2_000_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 12.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    info!("Crab Crossing - Game initialized!");
    info!("Controls: WASD/Arrow keys or gamepad to move");
    info!("Switch buttons: Z=B, X=A, Arrows=D-Pad");
}

/// Move player using switchbrew_bevy input abstraction
fn move_player(
    switch_input: Res<SwitchInput>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let speed = 5.0;
    let movement = switch_input.movement();

    if movement.length() > 0.0 {
        let direction = Vec3::new(movement.x, 0.0, -movement.y);

        for mut transform in &mut query {
            transform.translation += direction * speed * time.delta_secs();

            // Clamp to play area
            transform.translation.x = transform.translation.x.clamp(-9.0, 9.0);
            transform.translation.z = transform.translation.z.clamp(-9.0, 9.0);
        }
    }
}

fn rotate_cube(
    switch_input: Res<SwitchInput>,
    mut query: Query<&mut Transform, With<RotatingCube>>,
    time: Res<Time>,
) {
    // Rotate based on right stick or just slowly spin
    let rotation = if switch_input.right_stick.length() > 0.1 {
        switch_input.right_stick.x * 3.0
    } else {
        0.5
    };

    for mut transform in &mut query {
        transform.rotate_y(rotation * time.delta_secs());
    }
}
