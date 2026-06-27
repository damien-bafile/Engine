use bevy::{prelude::*, world_serialization::WorldAssetRoot};

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_car);
    }
}

fn spawn_car(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Suzuki Jimny 2018/2018_suzuki_jimny_4all.glb"))),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
}
