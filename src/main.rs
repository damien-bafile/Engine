use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use engine::plugins::camera::CameraPlugin;
use engine::plugins::world::car::CarPlugin;
use engine::plugins::world::floor::FloorPlugin;
use engine::plugins::world::lighting::LightingPlugin;
use iyes_perf_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            CarPlugin,
            FloorPlugin,
            LightingPlugin,
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            RenderDiagnosticsPlugin,
            PerfUiPlugin,
        ))
        .run();
}
