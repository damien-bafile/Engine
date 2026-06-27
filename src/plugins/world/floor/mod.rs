pub mod perlin;
pub mod terrain;

use bevy::prelude::*;
use bevy::ecs::observer::On;
use self::terrain::{spawn_terrain_mesh, TerrainSettings};

pub struct FloorPlugin;

#[derive(Event)]
pub struct RegenerateTerrain;

#[derive(Resource)]
pub struct FloorEntity(pub Option<Entity>);

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainSettings::default())
            .insert_resource(FloorEntity(None))
            .add_systems(Startup, spawn_floor)
            .add_observer(regenerate_terrain);
    }
}

fn spawn_floor(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<TerrainSettings>,
) {
    let entity = spawn_terrain_mesh(&mut commands, meshes, materials, &settings);
    commands.insert_resource(FloorEntity(Some(entity)));
}

fn regenerate_terrain(
    _trigger: On<RegenerateTerrain>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<TerrainSettings>,
    floor_entity: Res<FloorEntity>,
) {
    if let Some(entity) = floor_entity.0 {
        commands.entity(entity).despawn();
    }

    let entity = spawn_terrain_mesh(&mut commands, meshes, materials, &settings);
    commands.insert_resource(FloorEntity(Some(entity)));
}
