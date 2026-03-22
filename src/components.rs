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

/// Marker tag for the player (Creator) entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerTag;

/// Marker tag for Friend (animal) entities.
#[derive(Component, Debug, Clone, Copy)]
pub struct Friend;

/// Marker tag for Plant (food) entities.
#[derive(Component, Debug, Clone, Copy)]
pub struct Plant;

/// Energy component for Friends - determines hunger and reproduction.
///
/// Values:
/// - > 80: Can reproduce
/// - < 50: Seeks food
/// - <= 0: Dies
#[derive(Component, Debug, Clone, Copy)]
pub struct Energy(pub f32);

/// Nutritional value of a Plant - energy gained when consumed.
#[derive(Component, Debug, Clone, Copy)]
pub struct NutritionalValue(pub f32);

/// Velocity for smooth movement (boid-inspired attraction)
#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity(pub Vec2);

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
#[allow(dead_code)]
#[derive(Component, Debug, Clone, Copy)]
pub struct ChunkTile;

/// Tag for shadow entities (child entities for visual depth).
#[allow(dead_code)]
#[derive(Component, Debug, Clone, Copy)]
pub struct Shadow;
