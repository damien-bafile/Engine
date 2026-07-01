use bevy::{gltf::*, prelude::*};

pub struct CarPlugin;

#[derive(Resource, Default)]
pub struct CarEntity(pub Option<Entity>);

#[derive(Resource)]
pub struct CarSettings {
    location: Vec3,
}

impl Default for CarSettings {
    fn default() -> Self {
        Self {
            location: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}

#[derive(Resource)]
struct CarAssets {
    gltf_handle: Handle<Gltf>,
    spawned: bool,
}

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CarEntity>()
            .init_resource::<CarSettings>()
            .add_systems(Startup, load_car)
            .add_systems(Update, spawn_car_meshes);
    }
}

fn load_car(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<CarSettings>,
    mut car_entity: ResMut<CarEntity>,
) {
    let gltf_handle = asset_server.load::<Gltf>("models/Suzuki Jimny 2018/2018_suzuki_jimny_4all.glb");

    let entity = commands
        .spawn((
            Transform::from_translation(settings.location),
            Visibility::default(),
        ))
        .id();

    commands.insert_resource(CarAssets {
        gltf_handle,
        spawned: false,
    });
    car_entity.0 = Some(entity);
}

fn spawn_car_meshes(
    mut commands: Commands,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    gltf_materials: Res<Assets<GltfMaterial>>,
    mut car_assets: ResMut<CarAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    car_entity: Res<CarEntity>,
) {
    if car_assets.spawned {
        return;
    }

    let Some(gltf) = gltf_assets.get(&car_assets.gltf_handle) else { return };
    let Some(car_root) = car_entity.0 else { return };

    car_assets.spawned = true;

    for mesh_handle in &gltf.meshes {
        let Some(gltf_mesh) = gltf_meshes.get(mesh_handle) else { continue };

        for primitive in &gltf_mesh.primitives {
            let material = primitive
                .material
                .as_ref()
                .and_then(|h| gltf_materials.get(h))
                .map(|gltf_mat| StandardMaterial {
                    base_color: gltf_mat.base_color,
                    perceptual_roughness: gltf_mat.perceptual_roughness,
                    metallic: gltf_mat.metallic,
                    ..Default::default()
                })
                .unwrap_or(StandardMaterial {
                    base_color: Color::srgb(0.8, 0.8, 0.8),
                    ..Default::default()
                });

            let material_handle = materials.add(material);
            commands.entity(car_root).with_child((
                Mesh3d(primitive.mesh.clone()),
                MeshMaterial3d(material_handle),
                Transform::default(),
            ));
        }
    }
}
