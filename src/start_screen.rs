use bevy::{
    asset::AssetServer,
    camera::visibility::Visibility,
    ecs::prelude::*,
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

/// Marker component for the start button image
#[derive(Component, Clone, Default)]
struct StartButtonImage;

pub fn start_screen(mut commands: &mut Commands, state: &Res<GameState>) {
    commands.queue_spawn_scene(bsn! {
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
        Node {
            display: Display::Grid,
            grid_template_rows: vec![
                GridTrack::flex(2.),
                GridTrack::flex(1.),
            ],
            height: percent(100),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
        }
        Children [
            // Start Button
            Button
            Node {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                padding: UiRect::axes(percent(5), percent(4)),
                margin: UiRect::axes(percent(0), percent(5)),
                border: UiRect::all(px(5)),
            }
            BorderColor::all(GREEN_COLOR)
            on(|event: On<Pointer<Press>>, mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut start_button_image: Query<&mut ImageNode, With<StartButtonImage>>| {
                commands.entity(event.entity).insert(BorderColor::all(TEXT_PRESS_COLOR));
                start_button_image.single_mut().unwrap().image = asset_server.load("start/start_button_press.png");
            })
            on(|event: On<Pointer<Over>>,
                mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut start_button_image: Query<&mut ImageNode, With<StartButtonImage>>| {
                commands.entity(event.entity).insert(BorderColor::all(TEXT_OVER_COLOR));
                start_button_image.single_mut().unwrap().image = asset_server.load("start/start_button_over.png");
            })
            on(|event: On<Pointer<Out>>, mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut start_button_image: Query<&mut ImageNode, With<StartButtonImage>>| {
                commands.entity(event.entity).insert(BorderColor::all(GREEN_COLOR));
                start_button_image.single_mut().unwrap().image = asset_server.load("start/start_button.png");
            })
            on(|_: On<Pointer<Click>>, mut state: ResMut<GameState>, mut commands: Commands,
                mut menu_screen: Query<Entity, (With<StartScreen>, Without<GameScreen>)>,
                mut game_screen: Query<&mut Visibility, (With<GameScreen>, Without<StartScreen>)>| {
                commands.entity(menu_screen.single_mut().unwrap()).despawn();
                *game_screen.single_mut().unwrap() = Visibility::Visible;
                state.is_active = true;
            })
            Children [
                Node {
                    // If this uses percent(), it's a little bugged.
                    // Should probably investigate why.
                    min_width: vw(33),
                    height: percent(100),
                    min_height: px(36)
                }
                ImageNode {
                    image: "start/start_button.png"
                }
                StartButtonImage
            ],
            // Date
            Node {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center
            }
            Children [
                Text::new(format!("For: {}", date))
                TextFont {
                    font_size: FontSize::Px(30.0),
                }
                TextColor(GREEN_COLOR)
            ]
        ]
    }
}
