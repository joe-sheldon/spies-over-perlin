use bevy::asset::RenderAssetUsages;
use bevy::math::{Vec3};
use bevy::prelude::Mesh;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology::TriangleStrip;
use noise::{NoiseFn, Perlin};
use crate::constants::{TERRAIN_HIGH_FREQ_NOISE_SCALE, WORLD_NOISE_RATIO_HIGH, TERRAIN_LOW_FREQ_NOISE_SCALE, WORLD_NOISE_RATIO_LOW, TERRAIN_MID_FREQ_NOISE_SCALE, WORLD_NOISE_RATIO_MID};

pub fn generate_height(x: f32, y: f32, max_z: f32, seed: u32) -> f32 {
    let perlin = Perlin::new(seed);

    let low_freq_height = WORLD_NOISE_RATIO_LOW * perlin.get([(x / TERRAIN_LOW_FREQ_NOISE_SCALE) as f64, (y / TERRAIN_LOW_FREQ_NOISE_SCALE) as f64]) as f32;
    let mid_freq_height = WORLD_NOISE_RATIO_MID * perlin.get([(x / TERRAIN_MID_FREQ_NOISE_SCALE) as f64, (y / TERRAIN_MID_FREQ_NOISE_SCALE) as f64]) as f32;
    let high_freq_height = WORLD_NOISE_RATIO_HIGH * perlin.get([(x / TERRAIN_HIGH_FREQ_NOISE_SCALE) as f64, (y / TERRAIN_HIGH_FREQ_NOISE_SCALE) as f64]) as f32;

    let height = max_z * (low_freq_height + mid_freq_height + high_freq_height) / (WORLD_NOISE_RATIO_LOW + WORLD_NOISE_RATIO_MID + WORLD_NOISE_RATIO_HIGH);

    height
}

pub fn generate_terrain_vertices(
    size_x: f32,
    size_z: f32,
    max_y: f32,
    divisions_x: u32,
    divisions_z: u32,
    seed: u32,
) -> Result<Vec<Vec3>, String> {
    let mut verts: Vec<Vec3> = Vec::new();

    let perlin = Perlin::new(seed);
    for zp in 0..divisions_z {
        let z =  zp as f32 * size_z / divisions_z as f32;
        for xp in 0..divisions_x {
            let x = size_x * xp as f32 / divisions_x as f32;
            let vert = Vec3::new(x, generate_height(x, z, max_y, seed), z);
            verts.push(vert);
        }
    }

    Ok(verts)
}

pub fn generate_terrain_triangle_strips_from_vertices(
    divisions_x: u32,
    divisions_z: u32,
) -> Result<Vec<Vec<u32>>, String> {
    //
    // Grid is like this for a W4 H6 grid
    // ---------Strip 0--------------
    // 0      1      2      3      4
    // 5      6      7      8      9
    // ---------Strip 1--------------
    // 10     11     12     13     14
    // 15     16     17     18     19
    // ---------Strip 2--------------
    // 20     21     22     23     24
    // 25     26     27     28     29
    //
    // where per strip the indices are numbered:
    // 0      2     4       6      8
    // 1      3     5       7      9
    //
    // eg:
    // strip (0) tristrip:   0  5  1  6  2  7  3  8  4  9
    // strip (0) tris: 0 5 1 , 1 5 6 , 1 6 2 , 2 6 7 , 2 7 3 , 3 7 8 , 3 8 4 , 4 8 9
    //
    // strip (1) tristrip:  10 15 11 16 12 17 13 18 14 19
    // strip (1) tristrip:  20 25 21 26 22 27 23 28 24 29

    let mut strips: Vec<Vec<u32>> = Vec::new();

    if divisions_z % 2 != 0 {
        return Err("Z size of grid must be divisible by 2".to_string());
    }

    // FIXME Last strip would be handled differently. Skip for now
    let n_strips = divisions_z - 1;
    let n_verts_per_strip = 2 * divisions_x;

    for strip_idx in 0..n_strips {
        let mut strip_verts: Vec<u32> = Vec::new();

        let top_left_index = strip_idx * divisions_z;
        let bottom_left_index = top_left_index + divisions_x;

        for xIdx in 0..divisions_x {
            strip_verts.push(top_left_index + xIdx);
            strip_verts.push(bottom_left_index + xIdx);
        }

        strips.push(strip_verts);
    }

    Ok(strips)
}

fn compute_normals(coordinates: Vec<Vec3>, indices: Vec<u32>) -> Vec<Vec3> {
    let mut normals: Vec<Vec3> = Vec::new();
    for coordinates in coordinates {
        normals.push(Vec3::Y)
    }

    normals
}

pub fn generate_terrain_mesh_strips(coordinates: &Vec<Vec3>, strips: &Vec<Vec<u32>>) -> Result<Vec<Mesh>, String> {
    // https://docs.rs/bevy/latest/bevy/render/prelude/struct.Mesh.html
    let mut meshes: Vec<Mesh> = Vec::new();

    for strip in strips {
        // This is probably inefficient -- it's passing all the coordinates in to compute
        // the strip. The triangle-strip's vertices use the index from this list.
        let normals = compute_normals(coordinates.clone(), strip.clone());
        let terrain_mesh = Mesh::new(TriangleStrip, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, coordinates.clone())
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                normals,
            )
            .with_inserted_indices(Indices::U32(strip.clone()));

        meshes.push(terrain_mesh);
    }

    Ok(meshes)
}
