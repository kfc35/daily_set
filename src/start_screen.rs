use bevy::{
    asset::{AssetServer, Assets},
    camera::visibility::Visibility,
    ecs::prelude::*,
    image::{TextureAtlas, TextureAtlasLayout},
    math::UVec2,
    picking::prelude::*,
    scene::prelude::*,
    text::{FontSize, Justify, TextColor, TextFont, TextLayout},
    ui::prelude::*,
    ui_widgets::Button,
};

use crate::{
    GREEN_COLOR, GameScreen, TEXT_OVER_COLOR, TEXT_PRESS_COLOR,
    modal::how_to_play::{self, HowToPlayModal},
    on_handler_style_button_image,
    state::{CurrentGame, game_board::GameBoard},
};

/// Marker component for the start screen
#[derive(Component, Clone, Default)]
pub struct StartScreen;

pub fn start_screen(commands: &mut Commands, board: &Res<GameBoard>) {
    let date = format!("For: {}", &board.date);
    commands.queue_spawn_scene(bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceEvenly,
            align_content: AlignContent::Default,
            align_items: AlignItems::Center,
            width: percent(90),
            height: percent(90),
            left: percent(5),
            top: percent(5),
        }
        StartScreen
        Children [
            logo(),

            // Start Button
            button("menu/start_button.png", UVec2::new(32, 16))
            on(|_: On<Pointer<Click>>, mut game: ResMut<CurrentGame>, mut commands: Commands,
                mut menu_screen: Query<Entity, (With<StartScreen>, Without<GameScreen>)>,
                mut game_screen: Query<&mut Visibility, (With<GameScreen>, Without<StartScreen>)>| {
                commands.entity(menu_screen.single_mut().unwrap()).despawn();
                *game_screen.single_mut().unwrap() = Visibility::Visible;
                game.started = true;
            }),

            // How to Play Button
            button("menu/how_to_play.png", UVec2::new(48, 16))
            on(|_: On<Pointer<Click>>, query: Query<&mut Visibility, With<HowToPlayModal>>| {
                how_to_play::unhide(query);
            }),

            // Date
            Node {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center
            }
            Children [
                Text::new(date)
                TextFont {
                    font_size: FontSize::Px(20.0),
                }
                TextColor(GREEN_COLOR)
                TextLayout::justify(Justify::Center)
            ]
        ]
    });
}

fn logo() -> impl Scene {
    bsn! {
        Node {
            height: percent(50),
            width: percent(80),
        }
        ImageNode {
            image: "logo.png"
        }
    }
}

fn button(path: &'static str, tile_size: UVec2) -> impl Scene {
    bsn! {
        Button
        Node {
            border: UiRect::all(px(5)),
            height: percent(20),
            width: percent(50),
        }
        BorderColor::all(GREEN_COLOR)
        on_handler_style_button_image::<Over>(TEXT_OVER_COLOR, 1)
        on_handler_style_button_image::<Press>(TEXT_PRESS_COLOR, 2)
        on_handler_style_button_image::<Release>(TEXT_OVER_COLOR, 1)
        on_handler_style_button_image::<Out>(GREEN_COLOR, 0)
        // Unsure how to do this by just having to modify the texture_atlas of the ImageNode
        template(move |context| {
            let layout = TextureAtlasLayout::from_grid(tile_size, 1, 3, None, None);
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
    }
}
