//! Bio systems for Friend (animal) behavior.
//!
//! Handles attraction to player, metabolism, and reproduction.

use bevy::prelude::*;
use crate::components::*;
use crate::resources::TILE_SIZE;

/// How far a Friend can "see" to find food (in tiles).
const PERCEPTION_RANGE: i32 = 5;

/// Movement speed for Friends (tiles per second).
const MOVEMENT_SPEED: f32 = 1.0;

/// Attraction system - Friends move toward the Player.
///
/// Uses simple boid-inspired attraction with separation.
/// Runs every frame for smooth movement.
pub fn attraction_system(
    mut query: Query<(&mut GridPosition, &mut Energy), With<Friend>>,
    player_query: Query<&GridPosition, (With<PlayerTag>, Without<Friend>)>,
    // Plants that are NOT Friends (disjoint query using Without<Friend>)
    plant_query: Query<(Entity, &GridPosition, &NutritionalValue), (With<Plant>, Without<Friend>)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let player_pos = player_query.single();
    
    // Get all Friend positions for separation checking
    let friend_positions: Vec<IVec2> = query.iter().map(|(pos, _)| pos.0).collect();
    
    for (mut pos, mut energy) in query.iter_mut() {
        // Calculate attraction to player
        let to_player = player_pos.0 - pos.0;
        let distance_squared = to_player.x * to_player.x + to_player.y * to_player.y;
        
        // Move toward player if not too close (distance > 1)
        if distance_squared > 1 {
            // Calculate target position toward player
            let dx = to_player.x;
            let dy = to_player.y;
            
            let target_pos = if dx.abs() > dy.abs() {
                IVec2::new(pos.0.x + dx.signum(), pos.0.y)
            } else {
                IVec2::new(pos.0.x, pos.0.y + dy.signum())
            };
            
            // Check if target position is occupied by another Friend
            let is_occupied = friend_positions.iter().any(|&other_pos| other_pos == target_pos && other_pos != pos.0);
            
            if !is_occupied {
                // Move toward player if not occupied
                pos.0 = target_pos;
            } else {
                // Find nearest unoccupied adjacent tile
                let adjacent_positions = [
                    IVec2::new(target_pos.x + 1, target_pos.y),    // Right
                    IVec2::new(target_pos.x - 1, target_pos.y),    // Left
                    IVec2::new(target_pos.x, target_pos.y + 1),    // Up
                    IVec2::new(target_pos.x, target_pos.y - 1),    // Down
                ];
                
                // Find the first unoccupied adjacent position
                for &adjacent_pos in &adjacent_positions {
                    let is_adjacent_occupied = friend_positions.iter().any(|&other_pos| other_pos == adjacent_pos && adjacent_pos != pos.0);
                    if !is_adjacent_occupied {
                        pos.0 = adjacent_pos;
                        break;
                    }
                }
                // If all adjacent positions are occupied, don't move
            }
        }

        // Energy decay (metabolism) - lose energy every frame
        energy.0 -= 0.5 * time.delta_seconds();

        // Check if we moved onto a plant - eat it!
        let plants_to_eat: Vec<(Entity, f32)> = plant_query.iter()
            .filter(|(_, plant_pos, _)| plant_pos.0 == pos.0)
            .map(|(entity, _, nutrition)| (entity, nutrition.0))
            .collect();

        // Consume all plants on current tile (only if they exist)
        for (plant_entity, nutrition_value) in plants_to_eat {
            if commands.get_entity(plant_entity).is_some() {
                commands.entity(plant_entity).despawn();
                energy.0 += nutrition_value;
            }
        }

        // Check for reproduction (> 80 energy)
        if energy.0 > 80.0 {
            // Spawn new friend nearby
            let offset = IVec2::new(1, 0); // Always spawn to the right
            let new_pos = pos.0 + offset;
            
            commands.spawn((
                Friend,
                GridPosition(new_pos),
                Energy(50.0), // New friend starts with half energy
                VisualLayer(0.3),
                SpriteBundle {
                    sprite: Sprite { 
                        color: Color::GREEN, 
                        custom_size: Some(Vec2::splat(24.0)), 
                        ..default() 
                    },
                    transform: Transform::from_xyz(
                        new_pos.x as f32 * TILE_SIZE,
                        new_pos.y as f32 * TILE_SIZE,
                        0.3,
                    ),
                    ..default()
                },
            ));

            // Transfer energy to offspring
            energy.0 -= 50.0;
        }
    }
}

/// Metabolism system - handles death when energy is depleted.
///
/// Uses PostUpdate to ensure all other systems have finished modifying energy.
pub fn metabolism_system(
    mut commands: Commands,
    query: Query<(Entity, &Energy), With<Friend>>,
) {
    for (entity, energy) in query.iter() {
        if energy.0 <= 0.0 {
            // Despawn dead friend
            commands.entity(entity).despawn();
        }
    }
}
