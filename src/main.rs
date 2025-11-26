use bevy::prelude::*;

mod camera;
mod player;
mod world;

use camera::CameraPlugin;
use player::PlayerPlugin;
use world::floor::FloorPlugin;
use world::lighting::LightingPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            FloorPlugin,
            LightingPlugin,
            PlayerPlugin,
        ))
        .run();
}
