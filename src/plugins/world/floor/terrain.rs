use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;

use super::perlin::Perlin;
use crate::util::rng::RngKind;

#[derive(Resource, Clone)]
pub struct TerrainSettings {
    pub size: u32,
    pub scale: f32,
    pub height_scale: f32,
    pub noise_scale: f32,
    pub octaves: u32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub seed: u64,
    pub rng_kind: RngKind,
}

impl Default for TerrainSettings {
    fn default() -> Self {
        Self {
            size: 120,
            scale: 1.0,
            height_scale: 14.0,
            noise_scale: 0.04,
            octaves: 5,
            persistence: 0.5,
            lacunarity: 2.0,
            seed: 42,
            rng_kind: RngKind::SplitMix64,
        }
    }
}

pub fn build_terrain_mesh(settings: &TerrainSettings) -> Mesh {
    let perlin = Perlin::new(settings.seed, settings.rng_kind);
    let verts_per_side = settings.size + 1;

    let height_at = |x: f32, z: f32| -> f32 {
        perlin.fbm2d(
            x * settings.noise_scale,
            z * settings.noise_scale,
            settings.octaves,
            settings.persistence,
            settings.lacunarity,
        ) * settings.height_scale
    };

    let vertex_count = (verts_per_side * verts_per_side) as usize;
    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut uvs = Vec::with_capacity(vertex_count);

    let eps = settings.scale * 0.5;

    for z in 0..verts_per_side {
        for x in 0..verts_per_side {
            let wx = x as f32 * settings.scale;
            let wz = z as f32 * settings.scale;
            let wy = height_at(wx, wz);

            positions.push([wx, wy, wz]);
            uvs.push([
                x as f32 / settings.size as f32,
                z as f32 / settings.size as f32,
            ]);

            let hl = height_at(wx - eps, wz);
            let hr = height_at(wx + eps, wz);
            let hd = height_at(wx, wz - eps);
            let hu = height_at(wx, wz + eps);
            let normal = Vec3::new(hl - hr, 2.0 * eps, hd - hu).normalize();
            normals.push(normal.to_array());
        }
    }

    let mut indices = Vec::with_capacity((settings.size * settings.size * 6) as usize);
    for z in 0..settings.size {
        for x in 0..settings.size {
            let i = z * verts_per_side + x;
            let i_right = i + 1;
            let i_down = i + verts_per_side;
            let i_down_right = i_down + 1;

            indices.extend_from_slice(&[i, i_down, i_right]);
            indices.extend_from_slice(&[i_right, i_down, i_down_right]);
        }
    }

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices))
}

pub fn spawn_terrain_mesh(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: &TerrainSettings,
) -> Entity {
    let mesh = build_terrain_mesh(settings);
    let world_size = settings.size as f32 * settings.scale;
    let center = world_size * 0.5;

    commands
        .spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.35, 0.55, 0.30),
                perceptual_roughness: 0.9,
                ..default()
            })),
            Transform::from_xyz(-center, 0.0, -center),
        ))
        .id()
}
