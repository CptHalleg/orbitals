use bevy::asset::LoadState;
use bevy::light::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};

pub struct SkyboxPlugin;
impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (load_skybox_configs, configure_skybox).chain());
    }
}

#[derive(Component, Clone, Default)]
pub struct SkyboxConfig {
    pub image: &'static str,
    pub brightness: f32,
}

#[derive(Component)]
struct PendingSkybox {
    image: Handle<Image>,
    brightness: f32,
}

fn load_skybox_configs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    configs: Query<(Entity, &SkyboxConfig), Added<SkyboxConfig>>,
) {
    for (entity, config) in &configs {
        commands
            .entity(entity)
            .remove::<SkyboxConfig>()
            .insert(PendingSkybox {
                image: asset_server.load(config.image),
                brightness: config.brightness,
            });
    }
}

fn configure_skybox(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    pending_skyboxes: Query<(Entity, &PendingSkybox)>,
) {
    for (entity, pending) in &pending_skyboxes {
        match asset_server.get_load_state(&pending.image) {
            Some(LoadState::Failed(error)) => {
                error!(
                    "Failed to load skybox image {:?}: {error}",
                    pending.image.path()
                );
                commands.entity(entity).remove::<PendingSkybox>();
                continue;
            }
            Some(LoadState::Loaded) => {}
            _ => continue,
        }

        let Some(mut image) = images.get_mut(&pending.image) else {
            continue;
        };

        match image.texture_descriptor.array_layer_count() {
            1 => {
                if let Err(error) = image.reinterpret_stacked_2d_as_array(6) {
                    error!("Failed to reinterpret skybox image as six layers: {error}");
                    commands.entity(entity).remove::<PendingSkybox>();
                    continue;
                }
            }
            6 => {}
            layers => {
                error!(
                    "Skybox image must have one stacked layer or six cubemap layers, got {layers}"
                );
                commands.entity(entity).remove::<PendingSkybox>();
                continue;
            }
        }

        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });

        commands
            .entity(entity)
            .remove::<PendingSkybox>()
            .insert(Skybox {
                image: Some(pending.image.clone()),
                brightness: pending.brightness,
                rotation: default(),
            });
    }
}
