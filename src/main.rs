use bevy::prelude::*;
use engine::plugins::camera::CameraPlugin;
use engine::plugins::terrain_ui::TerrainUiPlugin;
use engine::plugins::world::car::CarPlugin;
use engine::plugins::world::floor::FloorPlugin;
use engine::plugins::world::lighting::LightingPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            CarPlugin,
            FloorPlugin,
            LightingPlugin,
            TerrainUiPlugin,
        ))
        .run();
}
