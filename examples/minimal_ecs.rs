//! Minimal ECS Example
//!
//! Demonstrates the core Bevy ECS patterns that work in both
//! std and no_std environments. This is the foundation for
//! Switch-compatible game logic.
//!
//! Run with: `cargo run --example minimal_ecs`

use bevy::prelude::*;

fn main() {
    App::new()
        // Minimal plugins - these would work on Switch too
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_entities, print_positions))
        .run();
}

// Components - these are 100% no_std compatible
#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

// Resources - also no_std compatible
#[derive(Resource, Default)]
struct GameState {
    frame_count: u64,
    player_score: u32,
}

fn setup(mut commands: Commands) {
    println!("Minimal ECS Example - Switch Compatible Core");
    println!("============================================");

    // Spawn player
    commands.spawn((
        Player,
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 1.0, y: 0.5 },
    ));

    // Spawn some enemies
    for i in 0..3 {
        commands.spawn((
            Enemy,
            Position {
                x: 10.0 + i as f32 * 5.0,
                y: 5.0,
            },
            Velocity {
                x: -0.5,
                y: 0.0,
            },
        ));
    }

    commands.insert_resource(GameState::default());

    println!("Spawned 1 player and 3 enemies");
}

fn move_entities(mut query: Query<(&mut Position, &Velocity)>, mut state: ResMut<GameState>) {
    // Simulate delta time (in real game, use Time resource)
    let dt = 0.016; // ~60 FPS

    for (mut pos, vel) in &mut query {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    }

    state.frame_count += 1;

    // Stop after 100 frames for demo
    if state.frame_count >= 100 {
        std::process::exit(0);
    }
}

fn print_positions(
    player_query: Query<&Position, With<Player>>,
    enemy_query: Query<&Position, With<Enemy>>,
    state: Res<GameState>,
) {
    // Only print every 20 frames
    if state.frame_count % 20 != 0 {
        return;
    }

    println!("\n--- Frame {} ---", state.frame_count);

    for pos in &player_query {
        println!("Player: ({:.2}, {:.2})", pos.x, pos.y);
    }

    for (i, pos) in enemy_query.iter().enumerate() {
        println!("Enemy {}: ({:.2}, {:.2})", i, pos.x, pos.y);
    }
}
