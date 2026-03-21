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
    /// Accumulator for delta time
    accumulator: f32,
    /// Last time a tick occurred
    last_tick: Instant,
}

impl TimeStep {
    /// Creates a new TimeStep with the specified tick rate.
    pub fn new(ticks_per_second: f32) -> Self {
        Self {
            tick_rate: ticks_per_second,
            accumulator: 0.0,
            last_tick: Instant::now(),
        }
    }

    /// Returns true if a tick should occur based on elapsed time.
    ///
    /// This should be called every frame with the frame delta time.
    pub fn should_tick(&mut self, delta: Duration) -> bool {
        self.accumulator += delta.as_secs_f32();
        let tick_interval = 1.0 / self.tick_rate;
        
        if self.accumulator >= tick_interval {
            self.accumulator -= tick_interval;
            return true;
        }
        false
    }

    /// Resets the accumulator (useful when pausing).
    pub fn reset(&mut self) {
        self.accumulator = 0.0;
        self.last_tick = Instant::now();
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

/// Total chunks to load: (VISIBLE_RADIUS + PRELOAD_RADIUS) * 2 + 1
pub const TOTAL_CHUNK_RADIUS: i32 = VISIBLE_RADIUS + PRELOAD_RADIUS;

/// Cached chunk data for persistence.
///
/// Stores the tile data for each chunk so player modifications persist.
#[derive(Resource, Default)]
pub struct ChunkCache {
    /// Maps (chunk_x, chunk_y) to chunk tile data
    data: HashMap<(i32, i32), ChunkData>,
    /// Set of visited chunk coordinates
    visited: HashMap<(i32, i32), bool>,
}

impl ChunkCache {
    /// Creates a new empty ChunkCache.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
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

    /// Gets chunk data if it exists.
    pub fn get_chunk(&self, chunk_pos: (i32, i32)) -> Option<&ChunkData> {
        self.data.get(&chunk_pos)
    }

    /// Inserts or updates chunk data.
    pub fn insert_chunk(&mut self, chunk_pos: (i32, i32), data: ChunkData) {
        self.visited.insert(chunk_pos, true);
        self.data.insert(chunk_pos, data);
    }

    /// Returns the number of cached chunks.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

/// Data for a single chunk (16x16 tiles).
#[derive(Clone, Debug)]
pub struct ChunkData {
    /// The chunk coordinates
    pub position: (i32, i32),
    /// Tile IDs for each position in the chunk (0 = grass, 1 = dirt, 2 = water, etc.)
    pub tiles: [[i32; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
    /// Entity modifications in this chunk (e.g., placed plants)
    pub modifications: Vec<ChunkModification>,
}

impl ChunkData {
    /// Creates a new empty ChunkData.
    pub fn new(chunk_x: i32, chunk_y: i32) -> Self {
        Self {
            position: (chunk_x, chunk_y),
            tiles: [[0; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
            modifications: Vec::new(),
        }
    }

    /// Gets tile at local chunk coordinates.
    pub fn get_tile(&self, local_x: i32, local_y: i32) -> Option<i32> {
        if local_x >= 0 && local_x < CHUNK_SIZE && local_y >= 0 && local_y < CHUNK_SIZE {
            Some(self.tiles[local_x as usize][local_y as usize])
        } else {
            None
        }
    }

    /// Sets tile at local chunk coordinates.
    pub fn set_tile(&mut self, local_x: i32, local_y: i32, tile_id: i32) {
        if local_x >= 0 && local_x < CHUNK_SIZE && local_y >= 0 && local_y < CHUNK_SIZE {
            self.tiles[local_x as usize][local_y as usize] = tile_id;
        }
    }
}

/// A modification made to a chunk (e.g., placed plant).
#[derive(Clone, Debug)]
pub struct ChunkModification {
    /// Position in world coordinates
    pub position: IVec2,
    /// Type of modification
    pub modification_type: ModificationType,
}

/// Types of chunk modifications.
#[derive(Clone, Debug)]
pub enum ModificationType {
    /// A plant was placed here
    Plant { nutritional_value: f32 },
    /// A decoration was placed here
    Decoration { decoration_type: i32 },
}
