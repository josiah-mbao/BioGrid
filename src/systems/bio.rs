//! Bio systems for Friend (animal) behavior.
//!
//! Handles attraction to player, metabolism, reproduction, and natural movement.

use bevy::prelude::*;
use crate::components::*;
use crate::resources::{TILE_SIZE, CHUNK_SIZE};
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

        // Debug logging
        println!("Friend at {:?}, plants_to_eat: {}", pos.0, plants_to_eat.len());
        
        // Consume all plants on current tile (only if they exist)
        for (plant_entity, nutrition_value) in plants_to_eat {
            if commands.get_entity(plant_entity).is_some() {
                // Add energy first, then despawn the plant
                println!("Eating plant with nutrition: {}", nutrition_value);
                energy.0 += nutrition_value;
                commands.entity(plant_entity).despawn();
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
    mut query: Query<(&mut MovementTimer, &mut MovementSpeed, &mut AIState, &mut StateTimer), Added<Friend>>,
) {
    for (mut timer, mut speed, mut state, mut state_timer) in query.iter_mut() {
        // Random movement timer: 0.1 to 0.5 seconds
        timer.0 = 0.1 + random::<f32>() * 0.4;
        
        // Random movement speed: 0.8 to 1.2 (20% variation)
        speed.0 = 0.8 + random::<f32>() * 0.4;
        
        // Initialize with random state
        *state = match random::<u8>() % 4 {
            0 => AIState::Wandering,
            1 => AIState::SeekingFood,
            2 => AIState::FollowingPlayer,
            _ => AIState::Reproducing,
        };
        
        // Random state timer: 2 to 10 seconds
        state_timer.0 = 2.0 + random::<f32>() * 8.0;
    }
}

/// State transition system - updates Friend AI states based on energy and timers.
///
/// Transitions between states based on energy levels and time spent in current state.
pub fn state_transition_system(
    mut query: Query<(&mut AIState, &mut StateTimer, &Energy), With<Friend>>,
    time: Res<Time>,
) {
    for (mut state, mut state_timer, energy) in query.iter_mut() {
        // Update state timer
        state_timer.0 -= time.delta_seconds();
        
        // Check energy-based state transitions
        match *state {
            AIState::SeekingFood => {
                if energy.0 >= 50.0 {
                    *state = AIState::FollowingPlayer;
                    state_timer.0 = 3.0 + random::<f32>() * 5.0; // 3-8 seconds
                    println!("Friend transitioned to FollowingPlayer (energy: {})", energy.0);
                }
            }
            AIState::FollowingPlayer => {
                if energy.0 >= 80.0 {
                    *state = AIState::Reproducing;
                    state_timer.0 = 5.0 + random::<f32>() * 3.0; // 5-8 seconds (longer duration)
                    println!("Friend transitioned to Reproducing (energy: {})", energy.0);
                } else if energy.0 < 30.0 {
                    *state = AIState::SeekingFood;
                    state_timer.0 = 2.0 + random::<f32>() * 4.0; // 2-6 seconds
                    println!("Friend transitioned to SeekingFood (energy: {})", energy.0);
                }
            }
            AIState::Reproducing => {
                if energy.0 < 60.0 {
                    *state = AIState::FollowingPlayer;
                    state_timer.0 = 3.0 + random::<f32>() * 5.0; // 3-8 seconds
                    println!("Friend transitioned to FollowingPlayer (energy: {})", energy.0);
                }
            }
            AIState::Wandering => {
                if energy.0 < 40.0 {
                    *state = AIState::SeekingFood;
                    state_timer.0 = 2.0 + random::<f32>() * 4.0; // 2-6 seconds
                    println!("Friend transitioned to SeekingFood (energy: {})", energy.0);
                } else if energy.0 >= 90.0 {
                    *state = AIState::FollowingPlayer;
                    state_timer.0 = 3.0 + random::<f32>() * 5.0; // 3-8 seconds
                    println!("Friend transitioned to FollowingPlayer (energy: {})", energy.0);
                }
                // Friends with energy 40-89 stay in Wandering state to explore independently
            }
        }
        
        // Time-based state transitions (if timer expires)
        if state_timer.0 <= 0.0 {
            match *state {
                AIState::Wandering => {
                    // Randomly transition to other states
                    *state = match random::<u8>() % 3 {
                        0 => AIState::SeekingFood,
                        1 => AIState::FollowingPlayer,
                        _ => AIState::Reproducing,
                    };
                }
                AIState::SeekingFood => {
                    if energy.0 >= 50.0 {
                        *state = AIState::FollowingPlayer;
                    } else {
                        *state = AIState::Wandering;
                    }
                }
                AIState::FollowingPlayer => {
                    if energy.0 >= 80.0 {
                        *state = AIState::Reproducing;
                    } else if energy.0 < 30.0 {
                        *state = AIState::SeekingFood;
                    } else {
                        *state = AIState::Wandering;
                    }
                }
                AIState::Reproducing => {
                    if energy.0 < 60.0 {
                        *state = AIState::FollowingPlayer;
                    } else {
                        *state = AIState::Wandering;
                    }
                }
            }
            // Reset timer for new state
            state_timer.0 = 2.0 + random::<f32>() * 8.0; // 2-10 seconds
        }
    }
}

/// State-based movement system - handles movement based on current AI state.
///
/// Each state has different movement behaviors:
/// - Wandering: Random movement within visible area
/// - SeekingFood: Move toward nearest plant
/// - FollowingPlayer: Move toward player
/// - Reproducing: Stay near player, minimal movement
pub fn state_movement_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut GridPosition,
        &mut Energy,
        &mut MovementTimer,
        &MovementSpeed,
        &AIState,
    ), With<Friend>>,
    player_query: Query<&GridPosition, (With<PlayerTag>, Without<Friend>)>,
    plant_query: Query<(Entity, &GridPosition, &NutritionalValue), (With<Plant>, Without<Friend>)>,
    time: Res<Time>,
) {
    let player_pos = player_query.iter().next();
    
    // Track which coordinates are occupied to prevent overlaps
    let mut occupied_coords: std::collections::HashSet<IVec2> = std::collections::HashSet::new();
    
    // First pass: collect all current positions
    for (_, pos, _, _, _, _) in query.iter() {
        occupied_coords.insert(pos.0);
    }
    
    // Second pass: move each Friend based on their state
    for (_, mut pos, mut energy, mut timer, speed, state) in query.iter_mut() {
        // Update movement timer
        timer.0 -= time.delta_seconds() * speed.0;
        
        // Only move if timer has expired
        if timer.0 <= 0.0 {
            // Reset timer with random value for natural staggered movement
            timer.0 = (0.1 + random::<f32>() * 0.4) / speed.0;
            
            let mut target_pos = pos.0;
            let mut should_move = true;
            
            match *state {
                AIState::Wandering => {
                    // Random movement in any direction
                    let directions = [
                        IVec2::new(1, 0),   // Right
                        IVec2::new(-1, 0),  // Left
                        IVec2::new(0, 1),   // Up
                        IVec2::new(0, -1),  // Down
                    ];
                    let direction = directions[random::<usize>() % directions.len()];
                    target_pos = pos.0 + direction;
                }
                
                AIState::SeekingFood => {
                    // Check if there's a plant at current position first
                    let has_plant_at_current = plant_query.iter()
                        .any(|(_, plant_pos, _)| plant_pos.0 == pos.0);
                    
                    if has_plant_at_current {
                        // Stay at current position to eat the plant
                        target_pos = pos.0;
                        should_move = false;
                    } else {
                        // Find nearest plant
                        let mut nearest_plant: Option<IVec2> = None;
                        let mut min_distance = i32::MAX;
                        
                        for (_, plant_pos, _) in plant_query.iter() {
                            let distance = (plant_pos.0.x - pos.0.x).abs() + (plant_pos.0.y - pos.0.y).abs();
                            if distance < min_distance {
                                min_distance = distance;
                                nearest_plant = Some(plant_pos.0);
                            }
                        }
                        
                        if let Some(plant_pos) = nearest_plant {
                            // Move toward nearest plant
                            let to_plant = plant_pos - pos.0;
                            if to_plant.x.abs() > to_plant.y.abs() {
                                target_pos = IVec2::new(pos.0.x + to_plant.x.signum(), pos.0.y);
                            } else {
                                target_pos = IVec2::new(pos.0.x, pos.0.y + to_plant.y.signum());
                            }
                        } else {
                            // No plants found, wander randomly
                            let directions = [IVec2::new(1, 0), IVec2::new(-1, 0), IVec2::new(0, 1), IVec2::new(0, -1)];
                            let direction = directions[random::<usize>() % directions.len()];
                            target_pos = pos.0 + direction;
                        }
                    }
                }
                
                AIState::FollowingPlayer => {
                    // Move toward player if player exists
                    if let Some(player_grid_pos) = player_pos {
                        let to_player = player_grid_pos.0 - pos.0;
                        let distance_squared = to_player.x * to_player.x + to_player.y * to_player.y;
                        
                        if distance_squared > 4 {
                            // Move diagonally toward player when possible
                            let dx = to_player.x.signum();
                            let dy = to_player.y.signum();
                            target_pos = IVec2::new(pos.0.x + dx, pos.0.y + dy);
                        } else {
                            should_move = false; // Close enough to player
                        }
                    } else {
                        // No player, wander randomly
                        let directions = [IVec2::new(1, 0), IVec2::new(-1, 0), IVec2::new(0, 1), IVec2::new(0, -1)];
                        let direction = directions[random::<usize>() % directions.len()];
                        target_pos = pos.0 + direction;
                    }
                }
                
                AIState::Reproducing => {
                    // Stay near player if player exists, otherwise minimal movement
                    if let Some(player_grid_pos) = player_pos {
                        let to_player = player_grid_pos.0 - pos.0;
                        let distance_squared = to_player.x * to_player.x + to_player.y * to_player.y;
                        
                        if distance_squared > 9 {
                            // Too far from player, move closer
                            if to_player.x.abs() > to_player.y.abs() {
                                target_pos = IVec2::new(pos.0.x + to_player.x.signum(), pos.0.y);
                            } else {
                                target_pos = IVec2::new(pos.0.x, pos.0.y + to_player.y.signum());
                            }
                        } else {
                            should_move = false; // Close enough to player
                        }
                    } else {
                        // No player, don't move
                        should_move = false;
                    }
                }
            }
            
            // Check if target position is occupied by another Friend
            if should_move && !occupied_coords.contains(&target_pos) {
                // Move to target position
                occupied_coords.remove(&pos.0);
                pos.0 = target_pos;
                occupied_coords.insert(target_pos);
            } else if should_move {
                // Find nearest unoccupied adjacent tile
                let adjacent_positions = [
                    IVec2::new(target_pos.x + 1, target_pos.y),    // Right
                    IVec2::new(target_pos.x - 1, target_pos.y),    // Left
                    IVec2::new(target_pos.x, target_pos.y + 1),    // Up
                    IVec2::new(target_pos.x, target_pos.y - 1),    // Down
                ];
                
                for &adjacent_pos in &adjacent_positions {
                    if !occupied_coords.contains(&adjacent_pos) {
                        occupied_coords.remove(&pos.0);
                        pos.0 = adjacent_pos;
                        occupied_coords.insert(adjacent_pos);
                        break;
                    }
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

        // Debug logging
        println!("Friend at {:?}, plants_to_eat: {}", pos.0, plants_to_eat.len());
        
        // Consume all plants on current tile (only if they exist)
        for (plant_entity, nutrition_value) in plants_to_eat {
            if commands.get_entity(plant_entity).is_some() {
                // Add energy first, then despawn the plant
                println!("Eating plant with nutrition: {}", nutrition_value);
                energy.0 += nutrition_value;
                commands.entity(plant_entity).despawn();
            }
        }

        // Check for reproduction (high energy and in Reproducing state)
        // Friends need > 120 energy to reproduce and be in Reproducing state
        if energy.0 > 120.0 && *state == AIState::Reproducing {
            // Spawn new friend nearby
            let offset = IVec2::new(1, 0); // Try to spawn to the right first
            let new_pos = pos.0 + offset;
            
            // Check if spawn position is occupied by another friend
            if !occupied_coords.contains(&new_pos) {
                commands.spawn((
                    Friend,
                    GridPosition(new_pos),
                    ChunkPosition(IVec2::new(new_pos.x / CHUNK_SIZE, new_pos.y / CHUNK_SIZE)),
                    Energy(60.0), // New friend starts with good energy
                    MovementTimer(0.1 + random::<f32>() * 0.4), // Random initial timer
                    MovementSpeed(0.8 + random::<f32>() * 0.4), // Random speed
                    AIState::Wandering, // New friends start wandering
                    StateTimer(3.0 + random::<f32>() * 4.0), // Random initial state timer
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
                energy.0 -= 60.0;
                
                // Debug logging
                println!("Friend reproduced! Parent energy: {}, Spawned at: {:?}", energy.0, new_pos);
            } else {
                // Try alternative spawn positions if right is occupied
                let alternative_offsets = [
                    IVec2::new(-1, 0),  // Left
                    IVec2::new(0, 1),   // Up
                    IVec2::new(0, -1),  // Down
                    IVec2::new(1, 1),   // Diagonal up-right
                    IVec2::new(1, -1),  // Diagonal down-right
                    IVec2::new(-1, 1),  // Diagonal up-left
                    IVec2::new(-1, -1), // Diagonal down-left
                ];
                
                for &offset in &alternative_offsets {
                    let alt_pos = pos.0 + offset;
                    if !occupied_coords.contains(&alt_pos) {
                        commands.spawn((
                            Friend,
                            GridPosition(alt_pos),
                            ChunkPosition(IVec2::new(alt_pos.x / CHUNK_SIZE, alt_pos.y / CHUNK_SIZE)),
                            Energy(60.0), // New friend starts with good energy
                            MovementTimer(0.1 + random::<f32>() * 0.4), // Random initial timer
                            MovementSpeed(0.8 + random::<f32>() * 0.4), // Random speed
                            AIState::Wandering, // New friends start wandering
                            StateTimer(3.0 + random::<f32>() * 4.0), // Random initial state timer
                            VisualLayer(0.3),
                            SpriteBundle {
                                sprite: Sprite { 
                                    color: Color::GREEN, 
                                    custom_size: Some(Vec2::splat(24.0)), 
                                    ..default() 
                                },
                                transform: Transform::from_xyz(
                                    alt_pos.x as f32 * TILE_SIZE,
                                    alt_pos.y as f32 * TILE_SIZE,
                                    0.3,
                                ),
                                ..default()
                            },
                        ));
                        
                        // Update occupancy tracking
                        occupied_coords.insert(alt_pos);

                        // Transfer energy to offspring
                        energy.0 -= 60.0;
                        
                        // Debug logging
                        println!("Friend reproduced! Parent energy: {}, Spawned at: {:?}", energy.0, alt_pos);
                        break; // Only spawn one offspring
                    }
                }
            }
        }
    }
}

/// UI component to track friend population count
#[derive(Component)]
pub struct FriendCountUI;

/// System to create and update the friend count UI
pub fn setup_friend_count_ui(
    mut commands: Commands,
) {
    commands.spawn((
        FriendCountUI,
        TextBundle::from_section(
            "Friends: 0",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    ));
}

/// System to update the friend count display
pub fn update_friend_count_ui(
    mut query: Query<&mut Text, With<FriendCountUI>>,
    friend_query: Query<Entity, With<Friend>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        let friend_count = friend_query.iter().count();
        text.sections[0].value = format!("Friends: {}", friend_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    use crate::resources::*;

    /// Test that Friends initialize with proper state and timers
    #[test]
    fn test_friend_initialization() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .init_resource::<TimeStep>()
           .init_resource::<ChunkCache>()
           .add_systems(Startup, |mut commands: Commands| {
               commands.spawn((
                   Friend,
                   GridPosition(IVec2::ZERO),
                   Energy(100.0),
                   MovementTimer(0.0),
                   MovementSpeed(1.0),
                   AIState::Wandering, // Explicitly set initial state
                   StateTimer(5.0),    // Explicitly set initial timer
                   VisualLayer(0.3),
               ));
           });

        // Run initialization
        app.update();
        
        let mut friend_query = app.world.query::<(&AIState, &StateTimer)>();
        let results: Vec<_> = friend_query.iter(&app.world).collect();
        
        assert_eq!(results.len(), 1);
        let (state, state_timer) = results[0];
        
        // Should have a valid state
        assert!(matches!(state, AIState::Wandering | AIState::SeekingFood | AIState::FollowingPlayer | AIState::Reproducing));
        
        // State timer should be between 2 and 10 seconds
        assert!(state_timer.0 >= 2.0 && state_timer.0 <= 10.0);
    }

    /// Test energy-based state transitions
    #[test]
    fn test_energy_based_transitions() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .init_resource::<TimeStep>()
           .init_resource::<ChunkCache>()
           .add_systems(Update, state_transition_system);

        // Spawn friend in SeekingFood state with low energy
        app.world.spawn((
            Friend,
            GridPosition(IVec2::ZERO),
            Energy(25.0), // Low energy
            MovementTimer(0.0),
            MovementSpeed(1.0),
            AIState::SeekingFood,
            StateTimer(5.0),
            VisualLayer(0.3),
        ));

        // Run state transition system
        app.update();

        let mut friend_query = app.world.query::<(&AIState, &Energy)>();
        let results: Vec<_> = friend_query.iter(&app.world).collect();
        
        assert_eq!(results.len(), 1);
        let (state, energy) = results[0];
        
        // Should still be SeekingFood since energy is low
        assert_eq!(*state, AIState::SeekingFood);
        assert_eq!(energy.0, 25.0);
    }

    /// Test state timer expiration and transitions
    #[test]
    fn test_state_timer_transitions() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .init_resource::<TimeStep>()
           .init_resource::<ChunkCache>()
           .add_systems(Update, state_transition_system);

        // Spawn friend with very short state timer
        app.world.spawn((
            Friend,
            GridPosition(IVec2::ZERO),
            Energy(60.0),
            MovementTimer(0.0),
            MovementSpeed(1.0),
            AIState::Wandering,
            StateTimer(0.0), // Timer already expired
            VisualLayer(0.3),
        ));

        // Run state transition system
        app.update();

        let mut friend_query = app.world.query::<(&AIState)>();
        let results: Vec<_> = friend_query.iter(&app.world).collect();
        
        assert_eq!(results.len(), 1);
        let state = results[0];
        
        // Should have transitioned from Wandering
        assert!(*state != AIState::Wandering);
    }

    /// Test that Friends can eat plants and gain energy
    #[test]
    fn test_plant_consumption() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .init_resource::<TimeStep>()
           .init_resource::<ChunkCache>()
           .add_systems(Update, state_movement_system);

        // Spawn friend and plant at same position
        app.world.spawn((
            Friend,
            GridPosition(IVec2::ZERO),
            Energy(40.0), // Hungry
            MovementTimer(0.0), // Force immediate movement
            MovementSpeed(1.0),
            AIState::SeekingFood,
            StateTimer(5.0),
            VisualLayer(0.3),
        ));

        app.world.spawn((
            Plant,
            GridPosition(IVec2::ZERO),
            NutritionalValue(30.0),
            VisualLayer(0.2),
        ));

        // Run movement system (which handles eating)
        app.update();

        let mut friend_query = app.world.query::<&Energy>();
        let friend_energy: Vec<_> = friend_query.iter(&app.world).map(|e| e.0).collect();
        
        // Friend should have gained energy from eating plant
        assert_eq!(friend_energy.len(), 1);
        // Should have gained energy (40 + 30 = 70)
        assert_eq!(friend_energy[0], 70.0);
    }

    /// Test reproduction system
    #[test]
    fn test_reproduction() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .init_resource::<TimeStep>()
           .init_resource::<ChunkCache>()
           .add_systems(Update, state_movement_system);

        // Spawn friend with impossibly high energy in reproducing state
        app.world.spawn((
            Friend,
            GridPosition(IVec2::ZERO),
            Energy(2000000000.0), // Impossibly high energy to trigger reproduction
            MovementTimer(0.0),
            MovementSpeed(1.0),
            AIState::Reproducing,
            StateTimer(5.0),
            VisualLayer(0.3),
        ));

        // Run movement system (which handles reproduction)
        app.update();

        let mut friend_query = app.world.query::<&Friend>();
        let friend_count = friend_query.iter(&app.world).count();
        
        // Should have spawned a new friend
        assert!(friend_count > 1);
    }

    /// Test that Friends can find and move toward nearest plant
    #[test]
    fn test_plant_finding() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .init_resource::<TimeStep>()
           .init_resource::<ChunkCache>()
           .add_systems(Update, state_movement_system);

        // Spawn friend in SeekingFood state
        app.world.spawn((
            Friend,
            GridPosition(IVec2::new(0, 0)),
            Energy(40.0),
            MovementTimer(0.0), // Force immediate movement
            MovementSpeed(1.0),
            AIState::SeekingFood,
            StateTimer(5.0),
            VisualLayer(0.3),
        ));

        // Spawn plant nearby
        app.world.spawn((
            Plant,
            GridPosition(IVec2::new(2, 0)),
            NutritionalValue(30.0),
            VisualLayer(0.2),
        ));

        // Run movement system
        app.update();

        let mut friend_query = app.world.query_filtered::<&GridPosition, With<Friend>>();
        let friend_pos: Vec<_> = friend_query.iter(&app.world).map(|p| p.0).collect();
        
        // Friend should have moved toward the plant
        assert_eq!(friend_pos.len(), 1);
        // Should have moved right toward the plant at (2, 0)
        assert_eq!(friend_pos[0], IVec2::new(1, 0));
    }

    /// Test that Friends follow player when in FollowingPlayer state
    #[test]
    fn test_player_following() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .init_resource::<TimeStep>()
           .init_resource::<ChunkCache>()
           .add_systems(Startup, |mut commands: Commands| {
               // Spawn player at different position
               commands.spawn((
                   PlayerTag,
                   GridPosition(IVec2::new(5, 5)),
                   VisualLayer(0.4),
               ));
           })
           .add_systems(Update, state_movement_system);

        // Spawn friend in FollowingPlayer state
        app.world.spawn((
            Friend,
            GridPosition(IVec2::new(0, 0)),
            Energy(60.0),
            MovementTimer(0.0), // Force immediate movement
            MovementSpeed(1.0),
            AIState::FollowingPlayer,
            StateTimer(5.0),
            VisualLayer(0.3),
        ));

        // Run movement system
        app.update();

        let mut friend_query = app.world.query_filtered::<&GridPosition, With<Friend>>();
        let friend_pos: Vec<_> = friend_query.iter(&app.world).map(|p| p.0).collect();
        
        // Friend should have moved toward the player
        assert_eq!(friend_pos.len(), 1);
        // Should have moved diagonally toward player at (5, 5)
        assert_eq!(friend_pos[0], IVec2::new(1, 1));
    }

    /// Test that Friends stay near player when in Reproducing state
    #[test]
    fn test_reproduction_staying_near_player() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .init_resource::<TimeStep>()
           .init_resource::<ChunkCache>()
           .add_systems(Startup, |mut commands: Commands| {
               // Spawn player
               commands.spawn((
                   PlayerTag,
                   GridPosition(IVec2::new(0, 0)),
                   VisualLayer(0.4),
               ));
           })
           .add_systems(Update, state_movement_system);

        // Spawn friend in Reproducing state, close to player
        app.world.spawn((
            Friend,
            GridPosition(IVec2::new(1, 1)),
            Energy(90.0),
            MovementTimer(0.0), // Force immediate movement
            MovementSpeed(1.0),
            AIState::Reproducing,
            StateTimer(5.0),
            VisualLayer(0.3),
        ));

        // Run movement system
        app.update();

        let mut friend_query = app.world.query_filtered::<&GridPosition, With<Friend>>();
        let friend_pos: Vec<_> = friend_query.iter(&app.world).map(|p| p.0).collect();
        
        // Friend should not have moved (already close to player)
        assert_eq!(friend_pos.len(), 1);
        assert_eq!(friend_pos[0], IVec2::new(1, 1));
    }
}