//! Core ECS components for BioGrid.
//!
//! This module defines all game entities' data components.
//! Components are pure data - they hold no logic.

use bevy::prelude::*;

/// Grid position using integer coordinates to prevent floating-point drift.
///
/// # Example
/// ```
/// GridPosition(IVec2::new(5, 10)) // x=5, y=10 on the grid
/// ```
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition(pub IVec2);

/// Legacy alias for GridPosition (for compatibility with existing code).
/// Prefer using GridPosition directly.
#[deprecated(note = "Use GridPosition instead")]
pub type GridPos = GridPosition;

/// Marker tag for the player (Creator) entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerTag;

/// Legacy alias for PlayerTag (for compatibility).
/// Prefer using PlayerTag directly.
#[deprecated(note = "Use PlayerTag instead")]
pub type Player = PlayerTag;

/// Marker tag for Friend (animal) entities.
#[derive(Component, Debug, Clone, Copy)]
pub struct Friend;

/// Marker tag for Plant (food) entities.
#[derive(Component, Debug, Clone, Copy)]
pub struct Plant;

/// Marker tag for the placement cursor.
#[derive(Component, Debug, Clone, Copy)]
pub struct PlacementCursor;

/// Energy component for Friends - determines hunger and reproduction.
///
/// Values:
/// - > 80: Can reproduce
/// - < 50: Seeks food
/// - <= 0: Dies
#[derive(Component, Debug, Clone, Copy)]
pub struct Energy(pub f32);

/// Timer for wander behavior - controls when a Friend moves randomly.
#[derive(Component, Debug, Clone)]
pub struct WanderTimer {
    /// The underlying timer
    pub timer: Timer,
    /// Minimum wait time in seconds
    pub min_duration: f32,
    /// Maximum wait time in seconds
    pub max_duration: f32,
}

impl WanderTimer {
    /// Creates a new WanderTimer with random duration between min and max.
    pub fn new(min: f32, max: f32) -> Self {
        let duration = rand::random::<f32>() * (max - min) + min;
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            min_duration: min,
            max_duration: max,
        }
    }

    /// Resets the timer with a new random duration.
    pub fn reset(&mut self) {
        let duration = rand::random::<f32>() * (self.max_duration - self.min_duration) + self.min_duration;
        self.timer = Timer::from_seconds(duration, TimerMode::Once);
    }
}

/// Nutritional value of a Plant - energy gained when consumed.
#[derive(Component, Debug, Clone, Copy)]
pub struct NutritionalValue(pub f32);

/// Visual layer for Z-ordering (render depth).
///
/// Lower values render first (behind), higher values render last (front).
/// Ground: 0.0, Plants: 0.2, Friends: 0.3, Player: 0.4, Cursor: 0.5
#[derive(Component, Debug, Clone, Copy)]
pub struct VisualLayer(pub f32);

/// Chunk position for tracking which chunk an entity belongs to.
///
/// Used for chunk management and unloading distant entities.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPosition(pub IVec2);

/// Tag for chunk tile entities.
#[derive(Component, Debug, Clone, Copy)]
pub struct ChunkTile;

/// Tag for shadow entities (child entities for visual depth).
#[derive(Component, Debug, Clone, Copy)]
pub struct Shadow;

/// Legacy alias for Energy (for compatibility).
/// Prefer using Energy directly.
#[deprecated(note = "Use Energy instead")]
pub type Stats = Energy;

/// Legacy alias for WanderTimer (for compatibility).
/// Prefer using WanderTimer directly.
#[deprecated(note = "Use WanderTimer instead")]
pub type MovementTimer = WanderTimer;
