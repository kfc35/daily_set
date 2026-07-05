use bevy::{
    asset::{ AssetServer, Assets},
    ecs::prelude::*,
    image::{
        TextureAtlas, TextureAtlasLayout,
    },
    math::UVec2,
    scene::prelude::*,
    ui::prelude::*,
};

/// Marker component for the loading screen
#[derive(Component, Clone, Default)]
pub struct LoadingScreen;

pub fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Node {
            display: Display::Flex,
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            width: percent(90),
            height: percent(90),
            left: percent(5),
            top: percent(5),
        }
        LoadingScreen
        Children [
            loading_image(),
        ]
    });
}

fn loading_image() -> impl Scene {
    bsn! {
        Node {
            width: percent(80),
        }
        // Unsure how to do this by just having to modify the texture_atlas of the ImageNode
        template(move |context| {
            let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 16), 1, 6, None, None);
            let layout_handle = context.resource_mut::<Assets<TextureAtlasLayout>>().add(layout);
            let texture_atlas = TextureAtlas {
                layout: layout_handle,
                index: 6,
            };
            Ok(ImageNode {
                image: context.resource::<AssetServer>().load("loading.png"),
                texture_atlas: Some(texture_atlas),
                ..Default::default()
            })
        })
    }
}