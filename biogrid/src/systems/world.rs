//! World systems for chunk management and procedural generation.
//!
//! Handles infinite chunk loading around the player.

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use crate::components::*;
use crate::resources::{ChunkCache, CHUNK_SIZE, VISIBLE_RADIUS, TILE_SIZE, PRELOAD_RADIUS};

/// Tag component for chunk tile entities.
#[derive(Component)]
pub struct ChunkTileEntity {
    pub chunk_pos: IVec2,
}

/// System to manage chunk loading around the player.
///
/// Calculates which chunks should be loaded based on player position,
/// spawns new chunks, and despawns distant ones.
pub fn chunk_manager_system(
    mut commands: Commands,
    player_query: Query<&GridPosition, With<PlayerTag>>,
    mut chunk_cache: ResMut<ChunkCache>,
    mut tile_query: Query<(Entity, &ChunkTileEntity)>,
) {
    let player_pos = player_query.single();
    
    // Calculate player's chunk coordinates
    let player_chunk_x = player_pos.0.x / CHUNK_SIZE;
    let player_chunk_y = player_pos.0.y / CHUNK_SIZE;
    
    // Calculate visible chunk range (including preload buffer)
    let total_radius = VISIBLE_RADIUS + PRELOAD_RADIUS;
    let min_chunk_x = player_chunk_x - total_radius;
    let max_chunk_x = player_chunk_x + total_radius;
    let min_chunk_y = player_chunk_y - total_radius;
    let max_chunk_y = player_chunk_y + total_radius;
    
    // Track which chunks should exist
    let mut chunks_to_keep: Vec<(i32, i32)> = Vec::new();
    
    // Check each visible chunk position
    for cy in min_chunk_y..=max_chunk_y {
        for cx in min_chunk_x..=max_chunk_x {
            chunks_to_keep.push((cx, cy));
            
            // If chunk doesn't exist in cache, generate it
            if !chunk_cache.is_visited((cx, cy)) {
                // Mark as visited and spawn the chunk
                chunk_cache.mark_visited((cx, cy));
                spawn_chunk(&mut commands, cx, cy);
            }
        }
    }
    
    // Despawn chunks that are too far away
    for (entity, chunk_tile) in tile_query.iter() {
        let cx = chunk_tile.chunk_pos.x;
        let cy = chunk_tile.chunk_pos.y;
        
        // Check if this chunk is still in range
        let should_keep = chunks_to_keep.iter().any(|(x, y)| *x == cx && *y == cy);
        
        if !should_keep {
            commands.entity(entity).despawn_recursive();
        }
    }
    
    // Print entity count for debugging (Sprint 2 acceptance criteria)
    info!("Visible chunks: {}, Total entities query needed for debugging", chunks_to_keep.len());
}

/// Spawns a chunk at the given coordinates.
///
/// Uses Perlin noise to generate terrain tiles.
fn spawn_chunk(commands: &mut Commands, chunk_x: i32, chunk_y: i32) {
    // Create Perlin noise generator (deterministic based on chunk coords)
    // Use wrapping operations to handle negative chunk coordinates
    let seed = (chunk_x as u32).wrapping_add((chunk_y as u32).wrapping_mul(65536));
    let noise = Perlin::new(seed);
    
    // Generate tiles for this chunk
    for local_y in 0..CHUNK_SIZE {
        for local_x in 0..CHUNK_SIZE {
            // World coordinates
            let world_x = chunk_x * CHUNK_SIZE + local_x;
            let world_y = chunk_y * CHUNK_SIZE + local_y;
            
            // Get noise value (-1 to 1)
            let noise_value = noise.get([world_x as f64 / 16.0, world_y as f64 / 16.0]) as f32;
            
            // Map noise to tile type
            // noise < -0.2 = Water (blue)
            // noise < 0.0 = Dirt (brown)
            // noise >= 0.0 = Grass (green)
            let (color, z_layer) = if noise_value < -0.2 {
                (Color::rgb(0.2, 0.3, 0.8), 0.0) // Water - blue, ground layer
            } else if noise_value < 0.0 {
                (Color::rgb(0.5, 0.35, 0.2), 0.0) // Dirt - brown
            } else {
                (Color::rgb(0.2, 0.6, 0.2), 0.0) // Grass - green
            };
            
            // Spawn tile entity
            commands.spawn((
                ChunkTileEntity {
                    chunk_pos: IVec2::new(chunk_x, chunk_y),
                },
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        world_x as f32 * TILE_SIZE,
                        world_y as f32 * TILE_SIZE,
                        z_layer,
                    ),
                    ..default()
                },
            ));
        }
    }
}
