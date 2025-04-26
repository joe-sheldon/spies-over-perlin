mod constants;
mod terrain;

use std::ops::{Add, Mul};
use bevy::math::Affine3A;
use bevy::prelude::*;
use bevy::reflect::List;
use bevy::tasks::futures_lite::StreamExt;
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};
use rand::prelude::*;

use crate::terrain::{
    generate_terrain_mesh_strips, generate_terrain_triangle_strips_from_vertices,
    generate_terrain_vertices,
};
use constants::*;

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(ClearColor(SKY_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugins(LookTransformPlugin)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            move_player
        ))
        .add_systems(Update, (
            move_camera_system,
        ))
        .run();
}

struct LidarModule {
    aim_at: Vec3,
    loc_origin: Vec3,
    loc_target: Vec3,
    scan_x: u32,
    scan_y: u32,
    distance: f32,
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    loc: Vec3,
    forward: Vec3,
    vel: f32,
    afterburner: bool,
    laser_on: bool,
    move_cooldown: Timer,
    lidar: Option<LidarModule>,
}

#[derive(Resource, Default)]
struct Game {
    world_size_x: f32,
    world_size_z: f32,
    world_mesh: Vec<Vec3>,
    city_centers: Vec<Vec3>,
    player: Player,
}


#[derive(Component)]
struct Water;

#[derive(Component)]
struct Terrain;

#[derive(Component)]
struct TargetBall;

#[derive(Component)]
struct Building;

fn rando_color() -> Color {
    let mut rng = rand::thread_rng();

    Color::srgb(rng.gen(), rng.gen(), rng.gen())
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>
) {

    //
    // Terrain / Water
    //
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
            MeshMaterial3d(materials.add(TERRAIN_COLOR)),
            Terrain,
        ));
    }
    // Water
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(WORLD_SIZE_X, WORLD_SIZE_Z))),
        MeshMaterial3d(materials.add(WATER_COLOR)),
        Transform::from_xyz(WORLD_SIZE_X / 2.0, WORLD_WATER_HEIGHT, WORLD_SIZE_Z / 2.0),
        Water,
    ));

    //
    // Cities (naive, need to do shift to ground level below spawn point and reject water-spawned
    // buildings)
    //
    let mut rng_cities = rand::thread_rng();
    let mut n_cities_generated = 0;
    while n_cities_generated < CITY_COUNT {
        let city_center = t_coords[rng_cities.gen_range(0..t_coords.len())];
        if city_center.y > (WORLD_WATER_HEIGHT) {
            game.city_centers.push(city_center);

            let city_radius = rng_cities.gen_range(CITY_MIN_RADIUS..CITY_MAX_RADIUS);
            let mut n_buildings_created = 0;
            while n_buildings_created < CITY_MAX_BUILDING_COUNT {
                // Not really a radius, but good enough for now
                let building_x = city_center.x + rng_cities.gen_range(-city_radius..city_radius);
                let building_z = city_center.z + rng_cities.gen_range(-city_radius..city_radius);
                let building_width_x = rng_cities.gen_range(CITY_MIN_BUILDING_EDGE_SIZE..CITY_MAX_BUILDING_EDGE_SIZE);
                let building_width_z = rng_cities.gen_range(CITY_MIN_BUILDING_EDGE_SIZE..CITY_MAX_BUILDING_EDGE_SIZE);
                let building_height_y = rng_cities.gen_range(CITY_MIN_BUILDING_HEIGHT..CITY_MAX_BUILDING_HEIGHT);

                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(building_width_x, building_height_y, building_width_z))),
                    MeshMaterial3d(materials.add(BUILDING_COLOR)),
                    Transform::from_translation(Vec3::new(building_x, city_center.y + (building_height_y / 2.0), building_z)),
                    Building
                ));

                n_buildings_created += 1;
            }


            n_cities_generated += 1;
        }
    }

    //
    // Targets (just red balls at the moment spawned randomly at ground level world vertices)
    //
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

    // Lighting
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::new(0.0, (WORLD_MAX_HEIGHT + 8.0), 0.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Player Setup
    game.player.loc = Vec3::new(WORLD_SIZE_X / 2.0, PLAYER_INITIAL_HEIGHT, WORLD_SIZE_Z / 2.0);
    game.player.forward = Vec3::Z;
    game.player.vel = 5.0;
    game.player.move_cooldown = Timer::from_seconds(0.1, TimerMode::Once);
    game.player.entity = Some(
        commands
            .spawn(
                (
                    Mesh3d(meshes.add(Sphere::new(PLAYER_SIZE))),
                    MeshMaterial3d(materials.add(PLAYER_COLOR)),
                    Transform::from_translation(game.player.loc),
                ),
            )
            .id(),
    );

    let target = game.player.loc;
    let eye = Vec3::new(
        target.x,
        target.y + 25.0,
        target.z - 25.0
    );

    commands
        .spawn(LookTransformBundle {
            transform: LookTransform::new(eye, target, Vec3::Y),
            smoother: Smoother::new(0.9), // Value between 0.0 and 1.0, higher is smoother.
        }).insert(Camera3d::default());
}

fn move_camera_system(mut cameras: Query<&mut LookTransform>, mut game: ResMut<Game>) {
    // Later, another system will update the `Transform` and apply smoothing automatically.
    for mut c in cameras.iter_mut() {
        let ploc = game.player.loc.clone();
        c.target = ploc;
        c.eye = Vec3::new(
            ploc.x,
            ploc.y + 25.0,
            ploc.z - 25.0
        );
        c.up = Vec3::Y;
    }
}

fn move_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    //TODO https://bevy-cheatbook.github.io/cookbook/smooth-movement.html

    if game.player.move_cooldown.tick(time.delta()).finished() {
        let mut rot = Quat::IDENTITY;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            if (game.player.vel < PLAYER_MAX_SPEED){
                game.player.vel = game.player.vel + PLAYER_SPEED_INCREMENT;
                println!("Speed Up: {:?}", game.player.vel);
            }
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            if (game.player.vel > PLAYER_MIN_SPEED){
                game.player.vel = game.player.vel - PLAYER_SPEED_INCREMENT;
                println!("Speed Down: {:?}", game.player.vel);
            }
        };
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            rot = Affine3A::from_rotation_y(-PLAYER_ROTATION_SPEED).to_scale_rotation_translation().1;
            game.player.forward = rot * game.player.forward;
            println!("Turn Right: {:?}", game.player.forward);
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            rot = Affine3A::from_rotation_y(PLAYER_ROTATION_SPEED).to_scale_rotation_translation().1;
            game.player.forward = rot * game.player.forward;
            println!("Turn Left: {:?}", game.player.forward);
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            game.player.loc.y = game.player.loc.y - 1.0;
            println!("Lowered: {:?}", game.player.loc);
        }
        if keyboard_input.pressed(KeyCode::Space) {
            game.player.loc.y = game.player.loc.y + 1.0;
            println!("Raised: {:?}", game.player.loc);
        }

        // Tick velocity up by vel vector
        game.player.loc = game.player.loc.add(game.player.forward.mul(game.player.vel * time.delta().as_secs_f32()));

        game.player.move_cooldown.reset();
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: game.player.loc,
            rotation: rot,
            ..default()
        };
    }
}