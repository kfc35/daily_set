use bevy::{
    asset::{AssetServer, Assets},
    camera::visibility::Visibility,
    ecs::prelude::*,
    image::{TextureAtlas, TextureAtlasLayout},
    math::UVec2,
    picking::prelude::*,
    scene::prelude::*,
    text::{FontSize, TextColor, TextFont},
    ui::prelude::*,
    ui_widgets::Button,
};

use crate::{GREEN_COLOR, GameScreen, TEXT_OVER_COLOR, TEXT_PRESS_COLOR, state::GameState};

/// Marker component for the start screen
#[derive(Component, Clone, Default)]
pub struct StartScreen;

pub fn start_screen(commands: &mut Commands, state: &Res<GameState>) {
    commands.queue_spawn_scene(bsn! {
        // TODO this doesn't need to be a grid... it's just a column.
        Node {
            display: Display::Grid,
            grid_template_rows: vec![RepeatedGridTrack::flex(2, 1.)],
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            width: percent(100),
            height: percent(100),
        }
        StartScreen
        Children [
            logo(),
            menu(state.date.clone())
        ]
    });
}

fn logo() -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
        }
        Children [
            Node {
                // If this uses percent(), it's a little bugged.
                // Should probably investigate why.
                width: vw(90),
            }
            ImageNode {
                image: "logo.png"
            }
        ]
    }
}

fn menu(date: String) -> impl Scene {
    bsn! {
        // TODO have to clean this up. Don't use grid.. it's just a column.
        Node {
            display: Display::Grid,
            grid_template_rows: vec![
                GridTrack::flex(2.),
                GridTrack::flex(2.),
                GridTrack::flex(1.),
            ],
            height: percent(100),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
        }
        Children [
            // Start Button
            button("menu/start_button.png")
            on(|_: On<Pointer<Click>>, mut state: ResMut<GameState>, mut commands: Commands,
                mut menu_screen: Query<Entity, (With<StartScreen>, Without<GameScreen>)>,
                mut game_screen: Query<&mut Visibility, (With<GameScreen>, Without<StartScreen>)>| {
                commands.entity(menu_screen.single_mut().unwrap()).despawn();
                *game_screen.single_mut().unwrap() = Visibility::Visible;
                state.is_active = true;
            }),

            // How to Play Button
            button("menu/how_to_play.png"),

            // Date
            Node {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center
            }
            Children [
                Text::new(format!("For: {}", date))
                TextFont {
                    font_size: FontSize::Px(20.0),
                }
                TextColor(GREEN_COLOR)
            ]
        ]
    }
}

fn button(path: &'static str) -> impl Scene {
    bsn! {
        Button
            Node {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                padding: UiRect::axes(percent(5), percent(4)),
                margin: UiRect::axes(percent(0), percent(5)),
                border: UiRect::all(px(5)),
            }
            BorderColor::all(GREEN_COLOR)
            on_handler_style_border_image::<Over>(TEXT_OVER_COLOR, 1)
            on_handler_style_border_image::<Press>(TEXT_PRESS_COLOR, 2)
            on_handler_style_border_image::<Release>(TEXT_OVER_COLOR, 1)
            on_handler_style_border_image::<Out>(GREEN_COLOR, 0)
            Children[
                Node {
                  // If this uses percent(), it's a little bugged.
                  // Should probably investigate why.
                  min_width: vw(33),
                  height: percent(100),
                  min_height: px(36)
                }
                // Unsure how to do this by just having to modify the texture_atlas of the ImageNode
                template(move |context| {
                    let layout = TextureAtlasLayout::from_grid(UVec2::new(48, 16), 1, 3, None, None);
                    let layout_handle = context.resource_mut::<Assets<TextureAtlasLayout>>().add(layout);
                    let texture_atlas = TextureAtlas {
                        layout: layout_handle,
                        index: 0,
                    };
                    Ok(ImageNode {
                        image: context.resource::<AssetServer>().load(path),
                        texture_atlas: Some(texture_atlas),
                        ..Default::default()
                    })
                })
            ]
    }
}

fn on_handler_style_border_image<E>(
    border_color: bevy::color::Color,
    texture_atlas_index: usize,
) -> impl Scene
where
    E: core::fmt::Debug + Clone + bevy::reflect::Reflect,
{
    bsn! {
        Node
        on(move |event: On<Pointer<E>>,
            mut commands: Commands,
            children_query: Query<&Children>,
            mut image_q: Query<&mut ImageNode>| {
            commands.entity(event.entity).insert(BorderColor::all(border_color));
            let Some(Ok(mut image_node)) = children_query
                .iter_descendants(event.entity)
                .find(|e| image_q.contains(*e))
                .map(|e| image_q.get_mut(e))
            else {
                return;
            };
            if let Some(atlas) = &mut image_node.texture_atlas {
              atlas.index = texture_atlas_index;
            }
        })
    }
}
