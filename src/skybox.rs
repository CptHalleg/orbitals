use bevy::light::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};

pub struct SkyboxPlugin;
impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            configure_skybox.run_if(on_message::<AssetEvent<Image>>),
        );
    }
}

fn configure_skybox(
    mut image_events: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
    skyboxes: Query<&Skybox>,
) {
    for event in image_events.read() {
        let id = match event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => *id,
            _ => continue,
        };

        let is_skybox_image = skyboxes.iter().any(|skybox| {
            skybox
                .image
                .as_ref()
                .is_some_and(|handle| handle.id() == id)
        });
        if !is_skybox_image {
            continue;
        }

        let Some(mut image) = images.get_mut(id) else {
            continue;
        };
        if image.texture_descriptor.array_layer_count() != 1 {
            continue;
        }

        if let Err(error) = image.reinterpret_stacked_2d_as_array(6) {
            error!("Failed to reinterpret skybox image as six layers: {error}");
            continue;
        }
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }
}
