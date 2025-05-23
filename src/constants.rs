use std::f32::consts::PI;
use bevy::color::Color;

//
// GLOBAL
//
pub(crate) const SKY_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

//
// TERRAIN
//
pub(crate) const WORLD_SIZE_X: f32 = 1024.0;
pub(crate) const WORLD_SIZE_Z: f32 = 1024.0;
pub(crate) const WORLD_GRID_DIVISIONS_X: u32 = 512;
pub(crate) const WORLD_GRID_DIVISIONS_Z: u32 = 512;
pub(crate) const WORLD_MAX_HEIGHT: f32 = 12.0;
// NOISE FREQUENCY SCALES -- HIGHER VALUE -- WIDER NOISE
pub(crate) const TERRAIN_LOW_FREQ_NOISE_SCALE: f32 = 200.0;
pub(crate) const TERRAIN_MID_FREQ_NOISE_SCALE: f32 = 30.0;
pub(crate) const TERRAIN_HIGH_FREQ_NOISE_SCALE: f32 = 5.0;
// RATIO OF LOW - MID - HIGH FREQUENCY NOISE; NORMALIZED OVER SUM OF PARTS
pub(crate) const WORLD_NOISE_RATIO_LOW: f32 = 5.0;
pub(crate) const WORLD_NOISE_RATIO_MID: f32 = 4.0;
pub(crate) const WORLD_NOISE_RATIO_HIGH: f32 = 2.0;
// WATER LEVEL (FLAT)
pub(crate) const WORLD_WATER_HEIGHT: f32 = 1.0;


//
// CITIES (BASIC)
//
pub(crate) const CITY_COUNT: usize = 8;
pub(crate) const CITY_MAX_BUILDING_COUNT: usize = 25;
pub(crate) const CITY_MIN_RADIUS: f32 = 10.0;
pub(crate) const CITY_MAX_RADIUS: f32 = 25.0;
pub(crate) const CITY_MIN_BUILDING_HEIGHT: f32 = 1.0;
pub(crate) const CITY_MAX_BUILDING_HEIGHT: f32 = 10.0;
pub(crate) const CITY_MIN_BUILDING_EDGE_SIZE: f32 = 0.5;
pub(crate) const CITY_MAX_BUILDING_EDGE_SIZE: f32 = 2.0;
pub(crate) const BUILDING_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);


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

//
// PLAYER INIT CONDITION
//
pub(crate) const PLAYER_COLOR: Color = Color::srgb(0.2, 0.2, 0.8);
pub(crate) const PLAYER_SIZE: f32 = 1.0;
pub(crate) const PLAYER_SPEED_INCREMENT: f32 = 1.0;
pub(crate) const PLAYER_MIN_SPEED: f32 = 4.0;
pub(crate) const PLAYER_MAX_SPEED: f32 = 18.0;
pub(crate) const PLAYER_ROTATION_SPEED: f32 = PI/24.0;
pub(crate) const PLAYER_INITIAL_HEIGHT: f32 = 15.0;
pub(crate) const PLAYER_LIDAR_RASTER_SCAN_MAX_DISTANCE: f32 = 1500.0;
pub(crate) const PLAYER_LIDAR_RASTER_SCAN_ANGLE_FROM_FORWARD: f32 = PI/4.0; // 45deg
pub(crate) const PLAYER_LIDAR_RASTER_HORI_SCAN_ANGLE: f32 = PI/4.0; //45deg
pub(crate) const PLAYER_LIDAR_RASTER_VERT_SCAN_ANGLE: f32 = PI/4.0; //45deg
pub(crate) const PLAYER_LIDAR_RASTER_HORI_DIVISIONS: u32 = 240;
pub(crate) const PLAYER_LIDAR_RASTER_VERT_DIVISIONS: u32 = 480;
