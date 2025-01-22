use bevy::color::Color;

//
// TERRAIN
//
pub(crate) const WORLD_SIZE_X: f32 = 100.0;
pub(crate) const WORLD_SIZE_Z: f32 = 100.0;
pub(crate) const WORLD_GRID_DIVISIONS_X: u32 = 256;
pub(crate) const WORLD_GRID_DIVISIONS_Z: u32 = 256;
pub(crate) const WORLD_MAX_HEIGHT: f32 = 8.0;
// NOISE FREQUENCY SCALES -- HIGHER VALUE -- WIDER NOISE
pub(crate) const TERRAIN_LOW_FREQ_NOISE_SCALE: f32 = 15.0;
pub(crate) const TERRAIN_MID_FREQ_NOISE_SCALE: f32 = 8.0;
pub(crate) const TERRAIN_HIGH_FREQ_NOISE_SCALE: f32 = 5.0;
// RATIO OF LOW - MID - HIGH FREQUENCY NOISE; NORMALIZED OVER SUM OF PARTS
pub(crate) const WORLD_NOISE_RATIO_LOW: f32 = 5.0;
pub(crate) const WORLD_NOISE_RATIO_MID: f32 = 4.0;
pub(crate) const WORLD_NOISE_RATIO_HIGH: f32 = 2.0;
// WATER LEVEL (FLAT)
pub(crate) const WORLD_WATER_HEIGHT: f32 = 1.0;

//
// GROUND TARGETS (BASIC)
//
pub(crate) const TARGET_COUNT: usize = 25;
pub(crate) const TARGET_SIZE: f32 = 0.25;

//
// COLORS
//
pub(crate) const TERRAIN_COLOR: Color = Color::srgb(0.2, 0.8, 0.3);
pub(crate) const WATER_COLOR: Color = Color::srgb(0.2, 0.3, 0.5);
pub(crate) const TARGET_COLOR: Color = Color::srgb(1.0, 0.2, 0.2);