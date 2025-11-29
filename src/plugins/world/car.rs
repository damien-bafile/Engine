use ::bevy::prelude::*;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_car);
    }
}
fn spawn_car(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/temp/temp.gltf"))),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
}

#[derive(Component)]
struct Player;

// fn player_movement(
//     keyboar_input: Res<Input<KeyCode>>,
//     time: Res<Time>,
//     mut player_q: Query<&mut Transform, With<Player>>,
//     cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
// ) {
// }
