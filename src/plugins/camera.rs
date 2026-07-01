use bevy::prelude::*;
use super::world::car::CarEntity;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn camera_follow(
    car_entity: Res<CarEntity>,
    transform_query: Query<&Transform, Without<Camera3d>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    let Some(car_entity) = car_entity.0 else { return };
    let Ok(car_transform) = transform_query.get(car_entity) else { return };
    let Ok(mut camera_transform) = camera_query.single_mut() else { return };

    let target = car_transform.translation;
    camera_transform.translation = target + Vec3::new(0.0, 15.0, 15.0);
    camera_transform.look_at(target, Vec3::Y);
}
