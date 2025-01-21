mod constants;
mod terrain;

use bevy::prelude::*;
use bevy::reflect::List;
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

    let t_strips = generate_terrain_triangle_strips_from_vertices(
        GRID_SIZE_DIVISIONS_X,
        GRID_SIZE_DIVISIONS_Z,
    )
    .unwrap();
    let t_coords = generate_terrain_vertices(
        GRID_SIZE_X,
        GRID_SIZE_Z,
        GRID_HEIGHT_MAX,
        GRID_SIZE_DIVISIONS_X,
        GRID_SIZE_DIVISIONS_Z,
        1,
    ).unwrap();

    let t_meshes = generate_terrain_mesh_strips(&t_coords, &t_strips).unwrap();

    let mut strip_number = 0;
    for t_mesh in t_meshes {
        let strip_mesh = commands
            .spawn((
                Mesh3d(meshes.add(t_mesh)),
                MeshMaterial3d(materials.add(Color::srgb(0.2, 0.8, 0.3))),
                // MeshMaterial3d(materials.add(rando_color())),
                Terrain,
            ))
            .id();

        strip_number += 1;
    }

    // let mut rng_target = rand::thread_rng();
    // let location_samples: Vec<Vec3> = t_coords.iter().choose_multiple(&mut rng_target, GRID_TARGET_COUNT).iter().collect();
    // for loc in location_samples {
    //     commands.spawn((
    //         Mesh3d(meshes.add(Sphere::new(GRID_TARGET_SIZE))),
    //         MeshMaterial3d(materials.add(Color::srgb(1.0, 0.2, 0.2))),
    //         Transform::from_translation(Vec3::new(loc.x, loc.y.abs() + GRID_TARGET_SIZE / 2.0, loc.z)),
    //         TargetBall
    //     ));
    // }

    // Water
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(GRID_SIZE_X, GRID_SIZE_Z))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.5))),
        Transform::from_xyz(GRID_SIZE_X / 2.0, 0.0, GRID_SIZE_Z / 2.0),
        Water,
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::new(0.0, (GRID_HEIGHT_MAX + 8.0), 0.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // pan orbit camera
    commands.spawn((
        Transform::from_translation(Vec3::new(
            (GRID_SIZE_X / 2.0),
            (GRID_HEIGHT_MAX + 5.0),
            (GRID_SIZE_Z / 2.0),
        )),
        PanOrbitCamera::default(),
    ));
}
