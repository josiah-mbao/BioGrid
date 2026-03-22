//! Bio systems for Friend (animal) behavior.
//!
//! Handles attraction to player, metabolism, reproduction, and natural movement.

use bevy::prelude::*;
use crate::components::*;
use crate::resources::TILE_SIZE;
use rand::random;

/// Natural movement system - Friends move with staggered timing and individual speeds.
///
/// Creates realistic, non-simultaneous movement patterns with:
/// - Individual movement timers (0.1-0.5 seconds)
/// - Variable movement speeds (0.8-1.2)
/// - Coordinate overlap prevention
/// - Smooth attraction to player
pub fn natural_movement_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut GridPosition,
        &mut Energy,
        &mut MovementTimer,
        &MovementSpeed,
    ), With<Friend>>,
    player_query: Query<&GridPosition, (With<PlayerTag>, Without<Friend>)>,
    plant_query: Query<(Entity, &GridPosition, &NutritionalValue), (With<Plant>, Without<Friend>)>,
    time: Res<Time>,
) {
    let player_pos = player_query.single();
    
    // Track which coordinates are occupied to prevent overlaps
    let mut occupied_coords: std::collections::HashSet<IVec2> = std::collections::HashSet::new();
    
    // First pass: collect all current positions
    for (_, pos, _, _, _) in query.iter() {
        occupied_coords.insert(pos.0);
    }
    
    // Second pass: move each Friend individually with staggered timing
    for (_, mut pos, mut energy, mut timer, speed) in query.iter_mut() {
        // Update movement timer
        timer.0 -= time.delta_seconds() * speed.0;
        
        // Only move if timer has expired
        if timer.0 <= 0.0 {
            // Reset timer with random value for natural staggered movement
            // Range: 0.1 to 0.5 seconds, adjusted by speed
            timer.0 = (0.1 + random::<f32>() * 0.4) / speed.0;
            
            // Debug logging
            println!("Friend moving! Timer: {}, Speed: {}, Position: {:?}", timer.0, speed.0, pos.0);
            
            // Calculate attraction to player
            let to_player = player_pos.0 - pos.0;
            let distance_squared = to_player.x * to_player.x + to_player.y * to_player.y;
            
            // Move toward player if not too close (distance > 2)
            if distance_squared > 4 {
                // Calculate target position toward player
                let dx = to_player.x;
                let dy = to_player.y;
                
                let target_pos = if dx.abs() > dy.abs() {
                    IVec2::new(pos.0.x + dx.signum(), pos.0.y)
                } else {
                    IVec2::new(pos.0.x, pos.0.y + dy.signum())
                };
                
                // Check if target position is occupied by another Friend
                // Use a temporary set that excludes current position
                let mut temp_occupied = occupied_coords.clone();
                temp_occupied.remove(&pos.0);
                
                if !temp_occupied.contains(&target_pos) {
                    // Move toward player if not occupied
                    // Update occupancy tracking - remove old position, add new position
                    occupied_coords.remove(&pos.0);
                    pos.0 = target_pos;
                    occupied_coords.insert(target_pos);
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
                        if !temp_occupied.contains(&adjacent_pos) {
                            // Update occupancy tracking - remove old position, add new position
                            occupied_coords.remove(&pos.0);
                            pos.0 = adjacent_pos;
                            occupied_coords.insert(adjacent_pos);
                            break;
                        }
                    }
                    // If all adjacent positions are occupied, don't move
                }
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
            
            // Check if spawn position is occupied
            if !occupied_coords.contains(&new_pos) {
                commands.spawn((
                    Friend,
                    GridPosition(new_pos),
                    Energy(50.0), // New friend starts with half energy
                    MovementTimer(0.1 + random::<f32>() * 0.4), // Random initial timer
                    MovementSpeed(0.8 + random::<f32>() * 0.4), // Random speed
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
                
                // Update occupancy tracking
                occupied_coords.insert(new_pos);

                // Transfer energy to offspring
                energy.0 -= 50.0;
            }
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

/// Initialize Friends with random movement timers and speeds for natural variation.
pub fn initialize_friend_movement(
    mut query: Query<(&mut MovementTimer, &mut MovementSpeed), Added<Friend>>,
) {
    for (mut timer, mut speed) in query.iter_mut() {
        // Random movement timer: 0.1 to 0.5 seconds
        timer.0 = 0.1 + random::<f32>() * 0.4;
        
        // Random movement speed: 0.8 to 1.2 (20% variation)
        speed.0 = 0.8 + random::<f32>() * 0.4;
    }
}
