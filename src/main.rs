mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use components::*;
use resources::*;
use systems::{bio, player, world};

/// Tile size in pixels - used for converting grid coords to world coords.
const TILE_SIZE: f32 = 32.0;

/// Lerp speed for camera following. Use 1.0 for instant snap, or higher for smoother but slower.
const CAMERA_LERP_SPEED: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<TimeStep>()
        .init_resource::<ChunkCache>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player::player_input,
                bio::attraction_system,
                bio::metabolism_system,
                world::chunk_manager_system,
                sync_grid_to_world,
                camera_follow_player,
            ),
        )
        .run();
}

/// Initial setup - spawns the player and camera.
fn setup(mut commands: Commands) {
    // Spawn camera at z=1000 so it can see sprites at lower z values
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // Higher order renders on top
                order: 10,
                clear_color: ClearColorConfig::Custom(Color::rgb(0.1, 0.1, 0.15)),
                ..default()
            },
            ..default()
        },
        CameraTag,
    ));

    // Spawn Creator (Player) - cyan square
    commands.spawn((
        PlayerTag,
        GridPosition(IVec2::ZERO),
        VisualLayer(0.4), // Player layer
        SpriteBundle {
            sprite: Sprite {
                color: Color::CYAN,
                custom_size: Some(Vec2::splat(28.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.4),
            ..default()
        },
    ));

    // Spawn initial Friends - green squares
    for i in 0..3 {
        for j in 0..3 {
            commands.spawn((
                Friend,
                GridPosition(IVec2::new(i * 2, j * 2)),
                Velocity(Vec2::ZERO),
                Energy(100.0),
                VisualLayer(0.3), // Friend layer
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::splat(24.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.3),
                    ..default()
                },
            ));
        }
    }
}

/// Tag component for the camera entity.
#[derive(Component)]
struct CameraTag;

/// Syncs GridPosition to Transform for rendering.
///
/// Converts integer grid coordinates to world pixel positions.
/// Each grid unit = TILE_SIZE pixels.
fn sync_grid_to_world(mut query: Query<(&GridPosition, &mut Transform)>) {
    for (grid_pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            grid_pos.0.x as f32 * TILE_SIZE,
            grid_pos.0.y as f32 * TILE_SIZE,
            transform.translation.z, // Preserve Z layer
        );
    }
}

/// Camera follows the player with smooth lerp interpolation.
///
/// Keeps the player centered on screen.
/// Reads GridPosition directly to avoid Transform query conflicts.
fn camera_follow_player(
    mut camera_query: Query<&mut Transform, With<CameraTag>>,
    player_query: Query<&GridPosition, With<PlayerTag>>,
    time: Res<Time>,
) {
    let player_pos = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    // Convert grid position to world pixels
    let target_x = player_pos.0.x as f32 * TILE_SIZE;
    let target_y = player_pos.0.y as f32 * TILE_SIZE;

    // Lerp camera position toward player
    camera_transform.translation.x = lerp(
        camera_transform.translation.x,
        target_x,
        CAMERA_LERP_SPEED * time.delta_seconds(),
    );
    camera_transform.translation.y = lerp(
        camera_transform.translation.y,
        target_y,
        CAMERA_LERP_SPEED * time.delta_seconds(),
    );
}

/// Linear interpolation helper.
fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}
