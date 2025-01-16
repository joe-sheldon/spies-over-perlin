mod constants;
mod terrain;

use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css::ORANGE;
use bevy::prelude::*;
use bevy::reflect::List;
use bevy::render::mesh::PrimitiveTopology::TriangleStrip;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use noise::{NoiseFn, Perlin};
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

#[derive(Component)]
struct StripLabel {
    entity: Entity,
}

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
    // Labeling system
    let text_style = TextFont {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        ..default()
    };
    let label_text_style = (text_style.clone(), TextColor(ORANGE.into()));

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
    );
    let t_meshes = generate_terrain_mesh_strips(&t_coords, &t_strips);

    let mut strip_number = 0;
    for t_mesh in t_meshes {
        let strip_mesh = commands
            .spawn((
                Mesh3d(meshes.add(t_mesh)),
                MeshMaterial3d(materials.add(rando_color())),
                Terrain,
            ))
            //     .with_children(|parent| {
            //     parent.spawn((Mesh3d(meshes.add(Sphere::new(GRID_CUBOID_SIZE_X / 2.0))),
            //                   MeshMaterial3d(materials.add(Color::srgb(1.0, 0.2, 0.2))),
            //                   TargetBall));
            // })
            .id();

        // commands
        //     .spawn((
        //         Node {
        //             position_type: PositionType::Absolute,
        //             ..default()
        //         },
        //         StripLabel { entity: strip_mesh },
        //     ))
        //     .with_children(|parent| {
        //         parent.spawn((
        //             Text::new(format!("┌─ Strip {} \n", strip_number)),
        //             label_text_style.clone(),
        //             Node {
        //                 position_type: PositionType::Absolute,
        //                 bottom: Val::ZERO,
        //                 ..default()
        //             },
        //             TextLayout::default().with_no_wrap(),
        //         ));
        //     });

        strip_number += 1;
    }

    // let perlin = Perlin::new(1);
    // let mut rng_target = rand::thread_rng();
    // for xp in 0..GRID_SIZE_DIVISIONS {
    //     let x = GRID_CUBOID_SIZE_X * xp as f32;
    //     for zp in 0..GRID_SIZE_DIVISIONS {
    //         let z = GRID_CUBOID_SIZE_Z * zp as f32;
    //
    //         let y = GRID_HEIGHT_MAX * perlin.get([x as f64, z as f64]) as f32;
    //         commands.spawn((
    //             Mesh3d(meshes.add(Cuboid::new(GRID_CUBOID_SIZE_X, y.abs(), GRID_CUBOID_SIZE_Z))),
    //             MeshMaterial3d(materials.add(Color::srgb(0.2, 0.8, 0.3))),
    //             Transform::from_translation(Vec3::new(x, y.abs() / 2.0, z)),
    //             TerrainPoint
    //         ));
    //
    //         let target_value: f32 = rng_target.gen();
    //         if target_value < GRID_CHANCE_SPAWN_TARGET {
    //             commands.spawn((
    //                 Mesh3d(meshes.add(Sphere::new(GRID_CUBOID_SIZE_X / 2.0))),
    //                 MeshMaterial3d(materials.add(Color::srgb(1.0, 0.2, 0.2))),
    //                 Transform::from_translation(Vec3::new(x, y.abs() + GRID_CUBOID_SIZE_X / 2.0, z)),
    //                 TargetBall
    //             ));
    //         }
    //     }
    // }

    // Water
    // commands.spawn((
    //     Mesh3d(meshes.add(Plane3d::default().mesh().size(GRID_SIZE_X, GRID_SIZE_Z))),
    //     MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.5))),
    //     Transform::from_xyz(GRID_SIZE_X / 2.0, 0.0, GRID_SIZE_Z / 2.0),
    //     Water,
    // ));

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
