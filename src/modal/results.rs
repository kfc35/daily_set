use bevy::{
    asset::{AssetServer, Assets},
    camera::visibility::Visibility,
    clipboard::Clipboard,
    ecs::prelude::*,
    image::{TextureAtlas, TextureAtlasLayout},
    math::UVec2,
    picking::prelude::*,
    scene::prelude::*,
    text::{FontSize, Justify, TextColor, TextFont, TextLayout},
    ui::prelude::*,
    ui_widgets::Button,
};
use chrono::{Datelike, Utc};
use rand::{RngExt, SeedableRng};

use crate::{
    CurrentGame, DEFAULT_BACKGROUND_COLOR, GREEN_COLOR, GameBoard, Modal, TEXT_OVER_COLOR,
    TEXT_PRESS_COLOR, modal::results_banner::ResultsBanner, on_handler_style_button_image,
};

/// Marker component for the Results Modal
#[derive(Component, Clone, Default)]
pub struct ResultsModal;

/// Unhides the Results Modal
pub fn unhide(mut query: Query<&mut Visibility, With<ResultsModal>>) {
    if let Ok(mut visibility) = query.single_mut() {
        *visibility = Visibility::Visible
    }
}

/// Spawns the Results Modal
pub fn spawn(commands: &mut Commands, board: &Res<GameBoard>, game: &CurrentGame) {
    let mins = game.elapsed.as_secs() / 60;
    let secs = game.elapsed.as_secs() % 60;
    let precise_time = format!("{mins:02}:{secs:02}.{:03}", game.elapsed.subsec_millis(),);
    let finish_time = format!(
        "You finished the Daily Set for {}: {precise_time}",
        board.date,
    );

    commands.spawn_scene(bsn! {
        Modal
        ResultsModal
        ZIndex(2) // Higher than the How To Play Modal.
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            left: percent(2.5),
            top: percent(2.5),
            height: percent(95),
            width: percent(95),
            padding: UiRect::horizontal(percent(2)),
            border: px(5),
            align_content: AlignContent::SpaceAround,
            justify_content: JustifyContent::SpaceEvenly,
        }
        BorderColor::all(GREEN_COLOR)
        BackgroundColor(DEFAULT_BACKGROUND_COLOR)
        Children [
            (
                Node {
                    width: percent(100),
                    max_height: percent(50),
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                }
                get_result_banner(&board, &game)
            ),
            (
                Node {
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::axes(percent(10), percent(0)),
                    max_height: percent(20),
                    overflow: Overflow::hidden(),
                }
                Children [
                    Text::new(finish_time)
                    TextFont {
                        font_size: FontSize::Rem(1.125),
                    }
                    TextColor(GREEN_COLOR)
                    TextLayout::justify(Justify::Center)
                ]
            ),
            share_button(),
            (
                Button
                Node {
                    border: UiRect::all(px(5)),
                    width: percent(70),
                    left: percent(15),
                    max_height: percent(10),
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                }
                BorderColor::all(GREEN_COLOR)
                on(|event: On<Pointer<Click>>,
                    mut commands: Commands,
                    parent_q: Query<&ChildOf>| {
                    commands.entity(parent_q.root_ancestor(event.entity)).insert(Visibility::Hidden);
                })
                on_handler_style_button_image::<Over>(TEXT_OVER_COLOR, 1)
                on_handler_style_button_image::<Press>(TEXT_PRESS_COLOR, 2)
                on_handler_style_button_image::<Release>(TEXT_OVER_COLOR, 1)
                on_handler_style_button_image::<Out>(GREEN_COLOR, 0)
                // Unsure how to do this by just having to modify the texture_atlas of the ImageNode
                template(move |context| {
                    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 16), 1, 3, None, None);
                    let layout_handle = context.resource_mut::<Assets<TextureAtlasLayout>>().add(layout);
                    let texture_atlas = TextureAtlas {
                        layout: layout_handle,
                        index: 0,
                    };
                    Ok(ImageNode {
                        image: context.resource::<AssetServer>().load("menu/close.png"),
                        texture_atlas: Some(texture_atlas),
                        ..Default::default()
                    })
                })
            )
        ]
    });
}

/// Spawns a clickable share button that copies the result of the
/// user's finished game into the clipboard.
fn share_button() -> impl Scene {
    bsn! {
        Button
        Node {
            border: UiRect::all(px(5)),
            width: percent(70),
            left: percent(15),
            max_height: percent(15),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
        }
        BorderColor::all(GREEN_COLOR)
        on_handler_style_button_image::<Over>(TEXT_OVER_COLOR, 1)
        on_handler_style_button_image::<Press>(TEXT_PRESS_COLOR, 2)
        on_handler_style_button_image::<Release>(TEXT_OVER_COLOR, 1)
        on_handler_style_button_image::<Out>(GREEN_COLOR, 0)
        on(move |event: On<Pointer<Out>>,
            mut commands: Commands,
            asset_server: Res<AssetServer>,
            mut layouts: ResMut<Assets<TextureAtlasLayout>>| {
                let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 16), 1, 3, None, None);
                let layout_handle = layouts.add(layout);
                let texture_atlas = TextureAtlas {
                    layout: layout_handle,
                    index: 0,
                };
                commands.entity(event.entity).insert(ImageNode {
                    image: asset_server.load("menu/share_results.png"),
                    texture_atlas: Some(texture_atlas),
                    ..Default::default()
                });
        })
        on(|event: On<Pointer<Click>>,
            mut commands: Commands,
            mut clipboard: ResMut<Clipboard>,
            board: Res<GameBoard>,
            state: Res<CurrentGame>,
            asset_server: Res<AssetServer>,
            mut layouts: ResMut<Assets<TextureAtlasLayout>>,| {
                let mins = state.elapsed.as_secs() / 60;
                let secs = state.elapsed.as_secs() % 60;
                let finish_time = format!("{}:{:02}", mins, secs);
                match clipboard.set_text(format!("#DailySet - {}\n{}\nhttps://kfc35.github.io/daily_set/",
                    board.date, finish_time)) {
                    Ok(_) => {
                        let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 16), 1, 3, None, None);
                        let layout_handle = layouts.add(layout);
                        let texture_atlas = TextureAtlas {
                            layout: layout_handle,
                            index: 1,
                        };
                        commands.entity(event.entity).insert(ImageNode {
                            image: asset_server.load("menu/copied.png"),
                            texture_atlas: Some(texture_atlas),
                            ..Default::default()
                        });
                    }
                    _ => {
                        commands.entity(event.entity).remove::<ImageNode>();
                        commands.entity(event.entity).insert(Text::new("Unable to Copy Results =/"));
                    }
                }
        })
        // Unsure how to do this by just having to modify the texture_atlas of the ImageNode
        template(move |context| {
            let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 16), 1, 3, None, None);
            let layout_handle = context.resource_mut::<Assets<TextureAtlasLayout>>().add(layout);
            let texture_atlas = TextureAtlas {
                layout: layout_handle,
                index: 0,
            };
            Ok(ImageNode {
                image: context.resource::<AssetServer>().load("menu/share_results.png"),
                texture_atlas: Some(texture_atlas),
                ..Default::default()
            })
        })
    }
}

/// Determines which result banner to give based on the game state.
fn get_result_banner(board: &Res<GameBoard>, game: &CurrentGame) -> Box<dyn Scene> {
    if board.date == "2026/07/10" {
        ResultsBanner::STATIC_RESULTS_BANNER[3].scene()
    } 
    else if board.date == "2026/07/09" {
        ResultsBanner::ANIMATIONS[4].scene()
    } else {
        let day_of_year = Utc::now().with_timezone(&chrono_tz::US::Eastern).ordinal() as u64;
        let results_banner_seed =
            bytemuck::cast::<[u64; 2], [u8; 16]>([game.elapsed.as_secs(), day_of_year << 1]);
        let mut rng = rand_pcg::Pcg32::from_seed(results_banner_seed);

        // A time of less than 3 minutes deserves an animation
        if game.elapsed.as_secs() / 60 < 3 {
            ResultsBanner::ANIMATIONS
                [rng.random_range(0..ResultsBanner::ANIMATIONS.len())]
            .scene()
        } else {
            ResultsBanner::STATIC_RESULTS_BANNER
                [rng.random_range(0..ResultsBanner::STATIC_RESULTS_BANNER.len())]
            .scene()
        }
    }
}
