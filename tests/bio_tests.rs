//! Bio-sim tests for the Great Gardener game.
//!
//! These tests verify basic data structures and logic.

use std::collections::HashMap;

/// Simulated Energy component
#[derive(Debug, Clone, PartialEq)]
struct Energy(f32);

/// Simulated GridPosition component  
#[derive(Debug, Clone, PartialEq)]
struct GridPosition {
    x: i32,
    y: i32,
}

/// Simulated NutritionalValue component
#[derive(Debug, Clone, PartialEq)]
struct NutritionalValue(f32);

/// Test that energy decays correctly
#[test]
fn test_energy_decay() {
    let mut energy = Energy(100.0);

    // Simulate metabolism: lose 1 energy per tick
    energy.0 -= 1.0;
    assert_eq!(energy.0, 99.0);

    energy.0 -= 1.0;
    assert_eq!(energy.0, 98.0);
}

/// Test that energy can be gained by eating
#[test]
fn test_energy_gain() {
    let mut energy = Energy(50.0);
    let plant_value = NutritionalValue(30.0);

    // Simulate eating a plant
    energy.0 += plant_value.0;
    assert_eq!(energy.0, 80.0);
}

/// Test that friend dies when energy is 0
#[test]
fn test_starvation() {
    let energy = Energy(0.0);

    // Check if dead
    let is_dead = energy.0 <= 0.0;
    assert!(is_dead, "Organism should be dead at 0 energy");
}

/// Test that friend can reproduce at high energy
#[test]
fn test_reproduction_threshold() {
    let energy = Energy(85.0);

    // Check if can reproduce (> 80 energy)
    let can_reproduce = energy.0 > 80.0;
    assert!(can_reproduce, "Should be able to reproduce at 85 energy");
}

/// Test distance calculation for perception
#[test]
fn test_perception_distance() {
    let friend_pos = GridPosition { x: 10, y: 10 };
    let plant_pos = GridPosition { x: 13, y: 10 };

    // Calculate Manhattan distance
    let distance = (plant_pos.x - friend_pos.x).abs() + (plant_pos.y - friend_pos.y).abs();

    assert_eq!(distance, 3, "Distance should be 3 tiles");

    // Check if within perception range (5 tiles)
    let perception_range = 5;
    let can_see = distance <= perception_range;
    assert!(can_see, "Should be able to see plant within 5 tiles");
}

/// Test that distant plant is not visible
#[test]
fn test_perception_out_of_range() {
    let friend_pos = GridPosition { x: 10, y: 10 };
    let plant_pos = GridPosition { x: 20, y: 10 };

    // Calculate Manhattan distance
    let distance = (plant_pos.x - friend_pos.x).abs() + (plant_pos.y - friend_pos.y).abs();

    assert_eq!(distance, 10, "Distance should be 10 tiles");

    // Check if within perception range (5 tiles)
    let perception_range = 5;
    let can_see = distance <= perception_range;
    assert!(!can_see, "Should NOT be able to see plant outside 5 tiles");
}

/// Test simple chunk cache logic
#[test]
fn test_chunk_visited_tracking() {
    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();

    // Initially no chunks visited
    assert!(!visited.contains_key(&(0, 0)));

    // Mark chunk as visited
    visited.insert((0, 0), true);
    assert!(visited.contains_key(&(0, 0)));
    assert!(visited.get(&(0, 0)).unwrap());

    // Mark another chunk
    visited.insert((1, 2), true);
    assert!(visited.contains_key(&(1, 2)));
}

/// Test that reproduction transfers energy
#[test]
fn test_reproduction_energy_transfer() {
    let mut parent_energy = Energy(100.0);
    let offspring_start_energy = 50.0;

    // Transfer energy to offspring
    parent_energy.0 -= offspring_start_energy;

    assert_eq!(
        parent_energy.0, 50.0,
        "Parent should have 50 energy after reproduction"
    );
}

/// Test movement in cardinal directions
#[test]
fn test_cardinal_movement() {
    let mut pos = GridPosition { x: 5, y: 5 };

    // Move right
    pos.x += 1;
    assert_eq!(pos.x, 6);

    // Move left
    pos.x -= 1;
    assert_eq!(pos.x, 5);

    // Move up
    pos.y += 1;
    assert_eq!(pos.y, 6);

    // Move down
    pos.y -= 1;
    assert_eq!(pos.y, 5);
}
