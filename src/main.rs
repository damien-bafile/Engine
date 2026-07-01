use bevy::prelude::*;
use engine::plugins::camera::CameraPlugin;
use engine::plugins::terrain_ui::TerrainUiPlugin;
use engine::plugins::world::car::CarPlugin;
use engine::plugins::world::floor::FloorPlugin;
use engine::plugins::world::lighting::LightingPlugin;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let debug = args.contains(&"debug".to_string());

    let mut app = App::new();
    app.add_plugins((
            DefaultPlugins,
            CameraPlugin,
            CarPlugin,
            FloorPlugin,
            LightingPlugin,
    ));

    if debug {
        app.add_plugins(TerrainUiPlugin);
    }
        
    app.run();
}
