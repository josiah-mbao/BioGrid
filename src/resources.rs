//! Game resources for BioGrid.
//!
//! Resources are singleton data that systems can access.

use bevy::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Time step controller for bio-simulation.
///
/// Controls the tick rate of the simulation (2 ticks per second).
/// This prevents the bio-sim from running too fast at 60 FPS.
#[derive(Resource)]
pub struct TimeStep {
    /// Target ticks per second
    pub tick_rate: f32,
}

impl TimeStep {
    /// Creates a new TimeStep with the specified tick rate.
    pub fn new(ticks_per_second: f32) -> Self {
        Self {
            tick_rate: ticks_per_second,
        }
    }
}

/// Default tick rate: 2 ticks per second for bio-simulation.
impl Default for TimeStep {
    fn default() -> Self {
        Self::new(2.0)
    }
}

/// Tile size in pixels (32x32).
pub const TILE_SIZE: f32 = 32.0;

/// Chunk size in tiles (16x16).
pub const CHUNK_SIZE: i32 = 16;

/// Visible radius in chunks (how many chunks around player to load).
pub const VISIBLE_RADIUS: i32 = 2;

/// Preload radius buffer (extra ring to prevent pop-in).
pub const PRELOAD_RADIUS: i32 = 1;


/// Cached chunk data for persistence.
///
/// Stores the tile data for each chunk so player modifications persist.
#[derive(Resource, Default)]
pub struct ChunkCache {
    /// Set of visited chunk coordinates
    visited: HashMap<(i32, i32), bool>,
}

impl ChunkCache {
    /// Creates a new empty ChunkCache.
    pub fn new() -> Self {
        Self {
            visited: HashMap::new(),
        }
    }

    /// Marks a chunk as visited.
    pub fn mark_visited(&mut self, chunk_pos: (i32, i32)) {
        self.visited.insert(chunk_pos, true);
    }

    /// Checks if a chunk has been visited.
    pub fn is_visited(&self, chunk_pos: (i32, i32)) -> bool {
        self.visited.contains_key(&chunk_pos)
    }
}


