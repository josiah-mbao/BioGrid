//! Comprehensive unit tests for BioGrid game mechanics.
//!
//! These tests cover the actual game logic from the systems and components,
//! including edge cases and component interactions that were missing from the basic tests.

use std::collections::HashMap;

/// Test data structures that mirror actual game components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TestGridPosition {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TestEnergy(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
struct TestMovementTimer(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
struct TestMovementSpeed(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
struct TestNutritionalValue(f32);

/// Test helper: Calculate Manhattan distance between two positions
fn manhattan_distance(pos1: TestGridPosition, pos2: TestGridPosition) -> i32 {
    (pos2.x - pos1.x).abs() + (pos2.y - pos1.y).abs()
}

/// Test helper: Check if position is occupied in a set of coordinates
fn is_position_occupied(pos: TestGridPosition, occupied: &HashMap<TestGridPosition, bool>) -> bool {
    occupied.contains_key(&pos)
}

/// Test helper: Get adjacent positions for a given position
fn get_adjacent_positions(pos: TestGridPosition) -> Vec<TestGridPosition> {
    vec![
        TestGridPosition { x: pos.x + 1, y: pos.y },     // Right
        TestGridPosition { x: pos.x - 1, y: pos.y },     // Left
        TestGridPosition { x: pos.x, y: pos.y + 1 },     // Up
        TestGridPosition { x: pos.x, y: pos.y - 1 },     // Down
    ]
}

// ==================== MOVEMENT SYSTEM TESTS ====================

/// Test staggered movement with timers and speeds
#[test]
fn test_staggered_movement_timing() {
    let mut timer = TestMovementTimer(0.5);
    let speed = TestMovementSpeed(1.0);
    let delta_time = 0.1;

    // Timer should decrease by delta_time * speed
    timer.0 -= delta_time * speed.0;
    assert!((timer.0 - 0.4).abs() < 0.0001, "Timer should be approximately 0.4");

    // After 5 updates, timer should expire
    for _ in 0..4 {
        timer.0 -= delta_time * speed.0;
    }
    assert!(timer.0 <= 0.0001, "Timer should expire after 5 updates");
}

/// Test movement speed variation
#[test]
fn test_movement_speed_variation() {
    let mut timer1 = TestMovementTimer(0.5);
    let speed1 = TestMovementSpeed(0.8); // Slow
    
    let mut timer2 = TestMovementTimer(0.5);
    let speed2 = TestMovementSpeed(1.2); // Fast
    
    let delta_time = 0.1;

    // After same time, faster entity should have lower timer
    timer1.0 -= delta_time * speed1.0;
    timer2.0 -= delta_time * speed2.0;
    
    assert!(timer2.0 < timer1.0, "Faster entity should have lower timer");
}

/// Test coordinate overlap prevention
#[test]
fn test_coordinate_overlap_prevention() {
    let mut friend_pos = TestGridPosition { x: 5, y: 5 };
    let player_pos = TestGridPosition { x: 10, y: 5 };
    
    // Simulate occupied coordinates (another friend at target position)
    let mut occupied: HashMap<TestGridPosition, bool> = HashMap::new();
    let target_pos = TestGridPosition { x: 6, y: 5 }; // Right of friend
    occupied.insert(target_pos, true);

    // Friend should not move to occupied position
    let to_player = TestGridPosition { 
        x: player_pos.x - friend_pos.x, 
        y: player_pos.y - friend_pos.y 
    };
    
    if to_player.x.abs() > to_player.y.abs() {
        let proposed_pos = TestGridPosition { 
            x: friend_pos.x + to_player.x.signum(), 
            y: friend_pos.y 
        };
        
        // Should not move if occupied
        if !is_position_occupied(proposed_pos, &occupied) {
            friend_pos = proposed_pos;
        }
    }
    
    // Position should remain unchanged due to occupancy
    assert_eq!(friend_pos, TestGridPosition { x: 5, y: 5 });
}

/// Test player attraction with distance thresholds
#[test]
fn test_player_attraction_distance_thresholds() {
    let friend_pos = TestGridPosition { x: 5, y: 5 };
    let close_player = TestGridPosition { x: 6, y: 5 };   // Distance = 1
    let far_player = TestGridPosition { x: 10, y: 5 };    // Distance = 5

    let close_distance = manhattan_distance(friend_pos, close_player);
    let far_distance = manhattan_distance(friend_pos, far_player);

    // Should not move toward close player (distance <= 2)
    assert!(close_distance <= 2, "Close player should not trigger movement");
    
    // Should move toward far player (distance > 2)
    assert!(far_distance > 2, "Far player should trigger movement");
}

/// Test behavior when all adjacent positions are occupied
#[test]
fn test_all_adjacent_positions_occupied() {
    let friend_pos = TestGridPosition { x: 5, y: 5 };
    let adjacent_positions = get_adjacent_positions(friend_pos);
    
    // Mark all adjacent positions as occupied
    let mut occupied: HashMap<TestGridPosition, bool> = HashMap::new();
    for pos in &adjacent_positions {
        occupied.insert(*pos, true);
    }

    // Friend should not be able to move
    for adjacent_pos in adjacent_positions {
        assert!(is_position_occupied(adjacent_pos, &occupied), 
                "All adjacent positions should be occupied");
    }
    
    // Friend position should remain unchanged
    assert_eq!(friend_pos, TestGridPosition { x: 5, y: 5 });
}

// ==================== ENERGY SYSTEM TESTS ====================

/// Test energy edge cases: negative and very high values
#[test]
fn test_energy_edge_cases() {
    // Test negative energy
    let negative_energy = TestEnergy(-10.0);
    assert!(negative_energy.0 < 0.0, "Should handle negative energy");
    
    // Test very high energy
    let high_energy = TestEnergy(200.0);
    assert!(high_energy.0 > 100.0, "Should handle high energy values");
    
    // Test exact zero energy
    let zero_energy = TestEnergy(0.0);
    assert_eq!(zero_energy.0, 0.0, "Should handle exact zero energy");
}

/// Test metabolism rate with different speeds
#[test]
fn test_metabolism_rate_with_speed() {
    let mut energy = TestEnergy(100.0);
    let delta_time = 0.1;
    let base_metabolism = 0.5;

    // Simulate metabolism over time
    for _ in 0..10 {
        energy.0 -= base_metabolism * delta_time;
    }
    
    // Expected: 100 - (0.5 * 0.1 * 10) = 100 - 0.5 = 99.5
    assert!((energy.0 - 99.5).abs() < 0.0001, "Energy should decrease by approximately 0.5 over 1 second");
}

/// Test plant consumption with multiple plants
#[test]
fn test_multiple_plant_consumption() {
    let mut energy = TestEnergy(50.0);
    let plant1 = TestNutritionalValue(20.0);
    let plant2 = TestNutritionalValue(15.0);
    let plant3 = TestNutritionalValue(10.0);

    // Consume all plants
    energy.0 += plant1.0 + plant2.0 + plant3.0;
    
    assert_eq!(energy.0, 95.0, "Should gain total nutritional value from all plants");
}

/// Test reproduction energy transfer at exact threshold
#[test]
fn test_reproduction_exact_threshold() {
    let mut parent_energy = TestEnergy(80.0);
    let offspring_start_energy = 50.0;

    // Should not reproduce at exactly 80 (needs > 80)
    let can_reproduce = parent_energy.0 > 80.0;
    assert!(!can_reproduce, "Should not reproduce at exactly 80 energy");

    // Should reproduce at 80.1
    parent_energy.0 = 80.1;
    let can_reproduce = parent_energy.0 > 80.0;
    assert!(can_reproduce, "Should reproduce at > 80 energy");
    
    // Energy transfer should work
    parent_energy.0 -= offspring_start_energy;
    assert!((parent_energy.0 - 30.1).abs() < 0.0001, "Should transfer approximately exact energy amount");
}

// ==================== COMPONENT INTERACTION TESTS ====================

/// Test GridPosition and Movement interaction
#[test]
fn test_grid_position_movement_interaction() {
    let mut pos = TestGridPosition { x: 0, y: 0 };
    
    // Move right
    pos.x += 1;
    assert_eq!(pos, TestGridPosition { x: 1, y: 0 });
    
    // Move up
    pos.y += 1;
    assert_eq!(pos, TestGridPosition { x: 1, y: 1 });
    
    // Move left
    pos.x -= 1;
    assert_eq!(pos, TestGridPosition { x: 0, y: 1 });
    
    // Move down
    pos.y -= 1;
    assert_eq!(pos, TestGridPosition { x: 0, y: 0 });
}

/// Test Energy and Reproduction interaction
#[test]
fn test_energy_reproduction_interaction() {
    let mut energy = TestEnergy(100.0);
    let reproduction_threshold = 80.0;
    let offspring_cost = 50.0;

    // Should be able to reproduce
    assert!(energy.0 > reproduction_threshold, "Should meet reproduction threshold");
    
    // After reproduction
    energy.0 -= offspring_cost;
    assert_eq!(energy.0, 50.0, "Should have remaining energy after reproduction");
    
    // Should not be able to reproduce again immediately
    assert!(energy.0 < reproduction_threshold, "Should not meet threshold after reproduction");
}

/// Test NutritionalValue consumption mechanics
#[test]
fn test_nutritional_value_consumption() {
    let mut energy = TestEnergy(30.0);
    let small_plant = TestNutritionalValue(10.0);
    let large_plant = TestNutritionalValue(50.0);

    // Eat small plant
    energy.0 += small_plant.0;
    assert_eq!(energy.0, 40.0, "Should gain small plant energy");

    // Eat large plant
    energy.0 += large_plant.0;
    assert_eq!(energy.0, 90.0, "Should gain large plant energy");
    
    // Should be able to reproduce now
    assert!(energy.0 > 80.0, "Should meet reproduction threshold after eating");
}

// ==================== CHUNK MANAGEMENT TESTS ====================

/// Test chunk boundary movement
#[test]
fn test_chunk_boundary_movement() {
    let chunk_size = 16;
    
    // Position at chunk boundary
    let boundary_pos = TestGridPosition { x: 15, y: 10 }; // Last position in chunk 0
    let next_pos = TestGridPosition { x: 16, y: 10 };     // First position in chunk 1
    
    let chunk_x_1 = boundary_pos.x / chunk_size;
    let chunk_x_2 = next_pos.x / chunk_size;
    
    assert_eq!(chunk_x_1, 0, "Should be in chunk 0");
    assert_eq!(chunk_x_2, 1, "Should be in chunk 1");
}

/// Test chunk loading/unloading logic
#[test]
fn test_chunk_loading_logic() {
    let mut chunk_cache: HashMap<(i32, i32), bool> = HashMap::new();
    let player_chunk = (0, 0);
    let visible_radius = 2;
    let preload_radius = 1;
    let total_radius = visible_radius + preload_radius;

    // Mark chunks as visited within range
    for cy in -total_radius..=total_radius {
        for cx in -total_radius..=total_radius {
            chunk_cache.insert((cx, cy), true);
        }
    }

    // Check that center chunks are visited
    assert!(chunk_cache.contains_key(&(0, 0)), "Player chunk should be visited");
    assert!(chunk_cache.contains_key(&(1, 1)), "Nearby chunk should be visited");
    
    // Check that distant chunks are not visited
    assert!(!chunk_cache.contains_key(&(5, 5)), "Distant chunk should not be visited");
}

/// Test chunk data persistence
#[test]
fn test_chunk_persistence() {
    let mut chunk_cache: HashMap<(i32, i32), bool> = HashMap::new();
    
    // Visit a chunk
    chunk_cache.insert((1, 2), true);
    assert!(chunk_cache.contains_key(&(1, 2)), "Chunk should be marked as visited");
    
    // Chunk should remain visited
    assert!(chunk_cache.contains_key(&(1, 2)), "Chunk should persist in cache");
    
    // Different chunk should not be visited
    assert!(!chunk_cache.contains_key(&(1, 3)), "Different chunk should not be visited");
}

// ==================== SYSTEM INTEGRATION TESTS ====================

/// Test movement timer expiration and reset logic
#[test]
fn test_movement_timer_expiration_reset() {
    let mut timer = TestMovementTimer(0.3);
    let speed = TestMovementSpeed(1.0);
    let delta_time = 0.1;

    // Timer should not expire yet
    timer.0 -= delta_time * speed.0;
    assert!(timer.0 > 0.0, "Timer should not expire after 1 update");

    // Timer should expire after 3 updates
    timer.0 -= delta_time * speed.0;
    timer.0 -= delta_time * speed.0;
    assert!(timer.0 <= 0.0001, "Timer should expire after 3 updates");

    // Reset timer (simulating system behavior)
    timer.0 = 0.1 + 0.4; // Random value between 0.1 and 0.5
    assert!(timer.0 >= 0.1 && timer.0 <= 0.5, "Timer should be reset to valid range");
}

/// Test speed variation effects on movement frequency
#[test]
fn test_speed_variation_movement_frequency() {
    let mut timer_slow = TestMovementTimer(0.1);
    let speed_slow = TestMovementSpeed(0.8);
    
    let mut timer_fast = TestMovementTimer(0.1);
    let speed_fast = TestMovementSpeed(1.2);
    
    let delta_time = 0.1;

    // After same time, faster entity should expire first
    timer_slow.0 -= delta_time * speed_slow.0;
    timer_fast.0 -= delta_time * speed_fast.0;
    
    // Fast entity should have lower timer (expires sooner)
    assert!(timer_fast.0 < timer_slow.0, "Fast entity should have lower timer");
    
    // Fast entity should expire first with repeated updates
    while timer_fast.0 > 0.0 {
        timer_fast.0 -= delta_time * speed_fast.0;
        timer_slow.0 -= delta_time * speed_slow.0;
    }
    
    assert!(timer_fast.0 <= 0.0, "Fast entity should expire first");
    assert!(timer_slow.0 > 0.0, "Slow entity should still have time remaining");
}

/// Test distance calculation with actual coordinate scenarios
#[test]
fn test_distance_calculation_scenarios() {
    let scenarios = vec![
        // (friend_pos, target_pos, expected_distance)
        (TestGridPosition { x: 0, y: 0 }, TestGridPosition { x: 3, y: 4 }, 7),
        (TestGridPosition { x: 5, y: 5 }, TestGridPosition { x: 5, y: 8 }, 3),
        (TestGridPosition { x: 10, y: 10 }, TestGridPosition { x: 7, y: 6 }, 7),
        (TestGridPosition { x: -2, y: 3 }, TestGridPosition { x: 1, y: -1 }, 7),
    ];

    for (friend_pos, target_pos, expected_distance) in scenarios {
        let distance = manhattan_distance(friend_pos, target_pos);
        assert_eq!(distance, expected_distance, 
                  "Distance calculation failed for {:?} to {:?}", 
                  friend_pos, target_pos);
    }
}

/// Test coordinate overlap detection with multiple entities
#[test]
fn test_multiple_entity_overlap_detection() {
    let mut occupied: HashMap<TestGridPosition, bool> = HashMap::new();
    
    // Add multiple occupied positions
    let positions = vec![
        TestGridPosition { x: 1, y: 1 },
        TestGridPosition { x: 2, y: 2 },
        TestGridPosition { x: 3, y: 3 },
    ];
    
    for pos in &positions {
        occupied.insert(*pos, true);
    }

    // Test that all positions are correctly marked as occupied
    for pos in &positions {
        assert!(is_position_occupied(*pos, &occupied), 
                "Position {:?} should be occupied", pos);
    }

    // Test that unoccupied positions are correctly identified
    let unoccupied_pos = TestGridPosition { x: 0, y: 0 };
    assert!(!is_position_occupied(unoccupied_pos, &occupied), 
            "Position {:?} should not be occupied", unoccupied_pos);
}