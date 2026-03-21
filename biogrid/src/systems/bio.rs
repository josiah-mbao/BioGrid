//! Bio systems for Friend (animal) behavior.
//!
//! Handles wandering, metabolism, perception, and reproduction.

use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::resources::{TimeStep, TILE_SIZE};

/// Wander behavior - Friends move randomly in cardinal directions.
///
/// Uses a timer to limit movement frequency (not every frame).
/// Runs on the TimeStep tick (2 ticks per second).
pub fn wander_system(
    mut time_step: ResMut<TimeStep>,
    time: Res<Time>,
    // Friends that are NOT Plants (disjoint query using Without<Plant>)
    mut query: Query<(&mut GridPosition, &mut WanderTimer, &mut Energy), (With<Friend>, Without<Plant>)>,
    // Plants that are NOT Friends (disjoint query using Without<Friend>)
    plant_query: Query<(Entity, &GridPosition, &NutritionalValue), (With<Plant>, Without<Friend>)>,
    mut commands: Commands,
) {
    // Only run on tick, not every frame
    if !time_step.should_tick(time.delta()) {
        return;
    }

    let mut rng = rand::thread_rng();
    let plant_positions: Vec<IVec2> = plant_query.iter().map(|(_, p, _)| p.0).collect();

    for (mut pos, mut timer, mut energy) in query.iter_mut() {
        // Tick the wander timer
        timer.timer.tick(time.delta());

        // Check if it's time to move
        if timer.timer.just_finished() {
            let current_energy = energy.0;
            
            // If hungry (< 50), seek food
            if current_energy < 50.0 && !plant_positions.is_empty() {
                // Find nearest plant
                let nearest_plant = plant_positions.iter()
                    .min_by_key(|p| (p.x - pos.0.x).abs() + (p.y - pos.0.y).abs())
                    .copied();

                if let Some(plant_pos) = nearest_plant {
                    // Move toward plant
                    let dx = plant_pos.x - pos.0.x;
                    let dy = plant_pos.y - pos.0.y;
                    
                    // Move one step in the direction of food
                    if dx.abs() > dy.abs() {
                        pos.0.x += dx.signum();
                    } else {
                        pos.0.y += dy.signum();
                    }
                }
            } else {
                // Random wander in cardinal direction
                let direction = match rng.gen_range(0..4) {
                    0 => IVec2::new(1, 0),   // Right
                    1 => IVec2::new(-1, 0),  // Left
                    2 => IVec2::new(0, 1),  // Up
                    _ => IVec2::new(0, -1), // Down
                };
                pos.0 += direction;
            }

            // Reset timer with new random duration
            timer.reset();
        }

        // Energy decay (metabolism) - lose energy every tick
        energy.0 -= 1.0;

        // Check if we moved onto a plant - eat it!
        let plants_to_eat: Vec<(Entity, f32)> = plant_query.iter()
            .filter(|(_, plant_pos, _)| plant_pos.0 == pos.0)
            .map(|(entity, _, nutrition)| (entity, nutrition.0))
            .collect();

        // Consume all plants on current tile
        for (plant_entity, nutrition_value) in plants_to_eat {
            commands.entity(plant_entity).despawn();
            energy.0 += nutrition_value;
        }

        // Check for reproduction (> 80 energy)
        if energy.0 > 80.0 {
            // Spawn new friend nearby
            let offset = match rng.gen_range(0..4) {
                0 => IVec2::new(1, 0),
                1 => IVec2::new(-1, 0),
                2 => IVec2::new(0, 1),
                _ => IVec2::new(0, -1),
            };
            let new_pos = pos.0 + offset;
            
            commands.spawn((
                Friend,
                GridPosition(new_pos),
                Energy(50.0), // New friend starts with half energy
                WanderTimer::new(0.5, 1.5),
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
