//! Plant spawning system for automatic world generation.
//!
//! Handles periodic spawning of new plants in visible chunks.

use crate::components::*;
use crate::resources::{CHUNK_SIZE, VISIBLE_RADIUS, TILE_SIZE, ChunkCache};
use bevy::prelude::*;
use rand::random;

/// System to handle automatic plant spawning.
///
/// Spawns plants every few seconds in visible chunks, respecting density limits.
pub fn plant_spawn_system(
    mut commands: Commands,
    mut spawn_timer: ResMut<PlantSpawnTimer>,
    time: Res<Time>,
    player_query: Query<&GridPosition, With<PlayerTag>>,
    chunk_cache: Res<ChunkCache>,
    plant_query: Query<&ChunkPosition, With<Plant>>,
) {
    // Update global spawn timer
    spawn_timer.timer -= time.delta_seconds();
    
    if spawn_timer.timer <= 0.0 {
        spawn_timer.timer = spawn_timer.spawn_interval;
        
        // Get player position to determine visible chunks
        if let Ok(player_pos) = player_query.get_single() {
            let player_chunk_x = player_pos.0.x / CHUNK_SIZE;
            let player_chunk_y = player_pos.0.y / CHUNK_SIZE;
            
            // Count plants per chunk
            let mut plants_per_chunk: std::collections::HashMap<(i32, i32), usize> = std::collections::HashMap::new();
            for chunk_pos in plant_query.iter() {
                let key = (chunk_pos.0.x, chunk_pos.0.y);
                *plants_per_chunk.entry(key).or_insert(0) += 1;
            }
            
            // Try to spawn in a random visible chunk
            let spawn_chunk_x = player_chunk_x + (random::<i32>() % (VISIBLE_RADIUS * 2 + 1)) - VISIBLE_RADIUS;
            let spawn_chunk_y = player_chunk_y + (random::<i32>() % (VISIBLE_RADIUS * 2 + 1)) - VISIBLE_RADIUS;
            
            let chunk_key = (spawn_chunk_x, spawn_chunk_y);
            let current_plants = *plants_per_chunk.get(&chunk_key).unwrap_or(&0);
            
            // Only spawn if under the limit and chunk exists
            if current_plants < spawn_timer.max_plants_per_chunk && chunk_cache.is_visited(chunk_key) {
                // Spawn at random position within the chunk
                let local_x = random::<i32>() % CHUNK_SIZE;
                let local_y = random::<i32>() % CHUNK_SIZE;
                
                let world_x = spawn_chunk_x * CHUNK_SIZE + local_x;
                let world_y = spawn_chunk_y * CHUNK_SIZE + local_y;
                
                let plant_pos = IVec2::new(world_x, world_y);
                
                commands.spawn((
                    Plant,
                    GridPosition(plant_pos),
                    ChunkPosition(IVec2::new(spawn_chunk_x, spawn_chunk_y)),
                    NutritionalValue(25.0 + random::<f32>() * 10.0), // Random nutrition 25-35
                    VisualLayer(0.2),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.2 + random::<f32>() * 0.2, 0.8, 0.2 + random::<f32>() * 0.2),
                            custom_size: Some(Vec2::splat(18.0)),
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
                
                info!("Spawned plant at chunk ({}, {}), world ({}, {})", 
                      spawn_chunk_x, spawn_chunk_y, world_x, world_y);
            }
        }
    }
}

/// System to handle plant death when eaten.
///
/// Removes plants that have been consumed by Friends.
pub fn plant_death_system(
    mut commands: Commands,
    plant_query: Query<(Entity, &GridPosition), With<Plant>>,
    friend_query: Query<&GridPosition, With<Friend>>,
) {
    // Check each plant to see if it's been eaten
    for (plant_entity, plant_pos) in plant_query.iter() {
        let mut should_despawn = false;
        
        // Check if any friend is on the same position
        for friend_pos in friend_query.iter() {
            if friend_pos.0 == plant_pos.0 {
                should_despawn = true;
                break;
            }
        }
        
        if should_despawn {
            commands.entity(plant_entity).despawn();
        }
    }
}