use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, input::mouse::AccumulatedMouseMotion, prelude::*};
use std::{f32::consts::FRAC_PI_2, ops::Range};
use crate::plugins::terrain_ui::{terrain_ui_panel, EguiActive};

pub struct CameraPlugin;

#[derive(Debug, Resource)]
struct CameraSettings {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    pub pitch_range: Range<f32>,
    pub roll_speed: f32,
    pub yaw_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            orbit_distance: 20.0,
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            roll_speed: 1.0,
            yaw_speed: 0.004,
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>();
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, orbit.after(terrain_ui_panel));
        app.add_plugins(FpsOverlayPlugin::default());
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    );
    commands.spawn(camera);

}

fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    egui_active: Res<EguiActive>,
) {
    if egui_active.0 {
        return;
    }

    let delta = mouse_motion.delta;
    let mut delta_roll = 0.0;

    if mouse_buttons.pressed(MouseButton::Left) {
        delta_roll -= 1.0;
    }
    if mouse_buttons.pressed(MouseButton::Right) {
        delta_roll += 1.0;
    }

    let delta_pitch = delta.y * camera_settings.pitch_speed;
    let delta_yaw = delta.x * camera_settings.yaw_speed;

    delta_roll *= camera_settings.roll_speed * time.delta_secs();

    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

    let pitch = (pitch + delta_pitch).clamp(
        camera_settings.pitch_range.start,
        camera_settings.pitch_range.end,
    );
    let roll = roll + delta_roll;
    let yaw = yaw + delta_yaw;
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    let target = Vec3::new(0.0, 1.0, 0.0);
    camera.translation = target - camera.forward() * camera_settings.orbit_distance;
}
