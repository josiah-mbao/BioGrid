use crate::components::*;
use crate::resources::{TILE_SIZE, CHUNK_SIZE};
use bevy::prelude::*;

/// Handles player input for movement and spawning.
///
/// Maps WASD keys to GridPosition updates (1 unit per press).
/// Maps Space to spawn a Plant at the player's position.
pub fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut GridPosition, With<PlayerTag>>,
    mut commands: Commands,
) {
    let mut pos = query.single_mut();

    // WASD movement - 1 grid unit per keypress
    if keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp) {
        pos.0.y += 1;
    }
    if keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown) {
        pos.0.y -= 1;
    }
    if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft) {
        pos.0.x -= 1;
    }
    if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight) {
        pos.0.x += 1;
    }

    // Spawn Plant (Creator Action) - press Space
    if keys.just_pressed(KeyCode::Space) {
        let plant_pos = pos.0;
        commands.spawn((
            Plant,
            GridPosition(plant_pos),
            ChunkPosition(IVec2::new(plant_pos.x / CHUNK_SIZE, plant_pos.y / CHUNK_SIZE)),
            NutritionalValue(30.0),
            VisualLayer(0.2), // Plant layer (behind friends)
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.6, 0.2),
                    custom_size: Some(Vec2::splat(20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    plant_pos.x as f32 * TILE_SIZE,
                    plant_pos.y as f32 * TILE_SIZE,
                    0.2,
                ),
                ..default()
            },
        ));
    }
}
