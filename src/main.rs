use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

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
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .run();
}
