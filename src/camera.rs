use ::bevy::prelude::*;
use iyes_perf_ui::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    );
    commands.spawn(camera);
    commands.spawn(PerfUiAllEntries::default());
}
