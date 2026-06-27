use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::plugins::world::floor::RegenerateTerrain;
use crate::plugins::world::floor::terrain::TerrainSettings;
use crate::util::rng::RngKind;

pub struct TerrainUiPlugin;

#[derive(Resource, Default)]
pub struct EguiActive(pub bool);

impl Plugin for TerrainUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EguiActive>()
            .add_plugins(EguiPlugin::default())
            .add_systems(Update, terrain_ui_panel);
    }
}

pub fn terrain_ui_panel(
    mut contexts: EguiContexts,
    mut settings: ResMut<TerrainSettings>,
    mut egui_active: ResMut<EguiActive>,
    mut commands: Commands,
    mut ready: Local<bool>,
) {
    if !*ready {
        *ready = true;
        return;
    }

    let mut regenerate = false;

    egui::Window::new("Terrain Config")
        .default_width(280.0)
        .show(contexts.ctx_mut().unwrap(), |ui| {
            ui.add(
                egui::Slider::new(&mut settings.size, 4..=500)
                    .text("Grid Size"),
            );
            ui.add(
                egui::Slider::new(&mut settings.scale, 0.1..=10.0)
                    .text("Scale"),
            );
            ui.add(
                egui::Slider::new(&mut settings.height_scale, 0.1..=80.0)
                    .text("Height Scale"),
            );
            ui.add(
                egui::Slider::new(&mut settings.noise_scale, 0.001..=1.0)
                    .text("Noise Scale"),
            );
            ui.add(
                egui::Slider::new(&mut settings.octaves, 1..=12)
                    .text("Octaves"),
            );
            ui.add(
                egui::Slider::new(&mut settings.persistence, 0.0..=1.0)
                    .text("Persistence"),
            );
            ui.add(
                egui::Slider::new(&mut settings.lacunarity, 1.0..=5.0)
                    .text("Lacunarity"),
            );
            ui.add(
                egui::DragValue::new(&mut settings.seed)
                    .speed(1)
                    .prefix("Seed: "),
            );

            ui.separator();

            egui::ComboBox::from_label("RNG")
                .selected_text(settings.rng_kind.name())
                .show_ui(ui, |ui| {
                    for kind in RngKind::ALL {
                        ui.selectable_value(
                            &mut settings.rng_kind,
                            kind,
                            kind.name(),
                        );
                    }
                });

            egui_active.0 = ui.ctx().egui_wants_pointer_input();

            ui.separator();

            if ui.button("Regenerate").clicked() {
                regenerate = true;
            }
        });

    if regenerate {
        commands.trigger(RegenerateTerrain);
    }
}
