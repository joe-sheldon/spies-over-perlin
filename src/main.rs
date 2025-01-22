mod constants;
mod terrain;

use bevy::prelude::*;
use bevy::reflect::List;
use bevy::tasks::futures_lite::StreamExt;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use rand::prelude::*;

use crate::terrain::{
    generate_terrain_mesh_strips, generate_terrain_triangle_strips_from_vertices,
    generate_terrain_vertices,
};
use constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Water;

#[derive(Component)]
struct Terrain;

#[derive(Component)]
struct TargetBall;

fn rando_color() -> Color {
    let mut rng = rand::thread_rng();

    Color::srgb(rng.gen(), rng.gen(), rng.gen())
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    // Terrain
    let t_strips = generate_terrain_triangle_strips_from_vertices(
        WORLD_GRID_DIVISIONS_X,
        WORLD_GRID_DIVISIONS_Z,
    )
    .unwrap();
    let t_coords = generate_terrain_vertices(
        WORLD_SIZE_X,
        WORLD_SIZE_Z,
        WORLD_MAX_HEIGHT,
        WORLD_GRID_DIVISIONS_X,
        WORLD_GRID_DIVISIONS_Z,
        1,
    ).unwrap();

    for t_mesh in generate_terrain_mesh_strips(&t_coords, &t_strips).unwrap() {
        commands.spawn((
            Mesh3d(meshes.add(t_mesh)),
            // MeshMaterial3d(materials.add(rando_color())),
            MeshMaterial3d(materials.add(TERRAIN_COLOR)),
            Terrain,
        ));
    }

    //targets
    let mut rng_target = rand::thread_rng();
    let mut n_targets_generated = 0;
    while n_targets_generated < TARGET_COUNT {
        let rand_loc = t_coords[rng_target.gen_range(0..t_coords.len())];
        if rand_loc.y > (WORLD_WATER_HEIGHT + TARGET_SIZE) {
            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(TARGET_SIZE))),
                MeshMaterial3d(materials.add(TARGET_COLOR)),
                Transform::from_translation(Vec3::new(rand_loc.x, rand_loc.y.abs() + (TARGET_SIZE / 2.0), rand_loc.z)),
                TargetBall
            ));
            n_targets_generated += 1;
        }
    }

    // Water
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(WORLD_SIZE_X, WORLD_SIZE_Z))),
        MeshMaterial3d(materials.add(WATER_COLOR)),
        Transform::from_xyz(WORLD_SIZE_X / 2.0, WORLD_WATER_HEIGHT, WORLD_SIZE_Z / 2.0),
        Water,
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::new(0.0, (WORLD_MAX_HEIGHT + 8.0), 0.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // pan orbit camera
    commands.spawn((
        Transform::from_translation(Vec3::new(
            (WORLD_SIZE_X / 2.0),
            (WORLD_MAX_HEIGHT + 5.0),
            (WORLD_SIZE_Z / 2.0),
        )),
        PanOrbitCamera::default(),
    ));
}
