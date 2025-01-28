mod constants;
mod terrain;

use std::f32::consts::PI;
use std::ops::Mul;
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
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player))
        .run();
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    loc: Vec3,
    rot: Vec3, //roll pitch yaw
    vel: Vec3,
    afterburner: bool,
    laser_on: bool,
    move_cooldown: Timer,
}

#[derive(Resource, Default)]
struct Game {
    world_size_x: f32,
    world_size_z: f32,
    world_mesh: Vec<Vec3>,
    player: Player,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
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
    mut game: ResMut<Game>
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

    // Player Setup
    game.player.loc = Vec3::new(WORLD_SIZE_X / 2.0, PLAYER_INITIAL_HEIGHT, WORLD_SIZE_Z / 2.0);
    game.player.rot = Vec3::new(0.0, 0.0, 0.0);
    game.player.vel = Vec3::new(0.0, 0.0, 0.1);
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


    // pan orbit camera around player
    commands.spawn((
        Transform::from_translation(game.player.loc),
        PanOrbitCamera::default(),
    ));
}

fn move_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if game.player.move_cooldown.tick(time.delta()).finished() {
        let mut turned = false;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            game.player.vel = game.player.vel.mul(1.1);
            println!("Speed Up: {:?}", game.player.vel.x);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            game.player.vel = game.player.vel.mul(0.9);
            println!("Slow Down: {:?}", game.player.vel.x);
        };
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            game.player.rot = Vec3::new(0.0, 0.0, game.player.rot.z + 2.0 * PI / 16.0);
            println!("Turn Right: {:?}", game.player.rot);
            turned = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            game.player.rot = Vec3::new(0.0, 0.0, game.player.rot.z  - 2.0 * PI / 16.0);
            println!("Turn Left: {:?}", game.player.rot);
            turned = true;
        }


        // Apply rotation to vel vector
        if (turned) {
            game.player.vel = game.player.vel.mul(game.player.rot); // this is wrong
        }

        // Tick velocity up by vel vector
        game.player.loc = game.player.loc + game.player.vel;

        // move on the board
        game.player.move_cooldown.reset();
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: game.player.loc,
            // rotation: Quat::from_rotation_y(rotation),
            ..default()
        };

    }
}
