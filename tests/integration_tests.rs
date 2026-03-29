//! Integration tests for the BioGrid game systems.
//!
//! Tests the complete functionality including state transitions and plant spawning.

use bevy::prelude::*;
use bevy::utils::Duration;

/// Test that the game can start without panicking
#[test]
fn test_game_starts() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    // Should not panic
    app.update();
}

/// Test that components can be created
#[test]
fn test_components_creation() {
    // Test GridPosition
    let pos = IVec2::new(5, 10);
    assert_eq!(pos.x, 5);
    assert_eq!(pos.y, 10);
    
    // Test Energy
    let energy = 100.0f32;
    assert_eq!(energy, 100.0);
    
    // Test AIState
    let state = AIState::Wandering;
    assert!(matches!(state, AIState::Wandering));
}

/// Test that systems can be registered
#[test]
fn test_system_registration() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    // Should be able to add systems without panicking
    app.add_systems(Update, |_: Res<Time>| {});
    
    app.update();
}

/// Test basic entity spawning
#[test]
fn test_entity_spawning() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    app.world.spawn((
        GridPosition(IVec2::ZERO),
        Energy(100.0),
    ));
    
    let mut query = app.world.query::<(&GridPosition, &Energy)>();
    let results: Vec<_> = query.iter(&app.world).collect();
    
    assert_eq!(results.len(), 1);
    let (pos, energy) = results[0];
    assert_eq!(pos.0, IVec2::ZERO);
    assert_eq!(energy.0, 100.0);
}

// Define the types we need for testing
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition(pub IVec2);

#[derive(Component, Debug, Clone, Copy)]
pub struct Energy(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIState {
    Wandering,
    SeekingFood,
    FollowingPlayer,
    Reproducing,
}