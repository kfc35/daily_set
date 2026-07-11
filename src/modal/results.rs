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
    ui_widgets::{Button, ControlOrientation, Scrollbar, ScrollbarThumb},
};
use chrono::{Datelike, Utc};
use rand::{RngExt, SeedableRng};

use crate::{
    CurrentGame, DEFAULT_BACKGROUND_COLOR, GREEN_COLOR, GameBoard, LIGHT_BLUE_COLOR, Modal,
    ModalScrollArea, TEXT_OVER_COLOR, TEXT_PRESS_COLOR, WHITE_VERY_TRANSPARENT_COLOR,
    modal::results_banner::ResultsBanner, on_handler_style_button_image,
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
    let flawless = if game.mistake_counter == 0 && game.already_found_but_guessed_counter == 0 {
        " - No Mistakes!"
    } else if game.mistake_counter == 0 {
        " - No Incorrect Guesses!"
    } else {
        ""
    };
    let finish_time = format!(
        "You finished the Daily Set for {}: {precise_time}{flawless}",
        board.date,
    );

    commands.spawn_scene(bsn! {
        Modal
        ResultsModal
        GlobalZIndex(2) // Higher than the How To Play Modal.
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
                    max_height: percent(45),
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                }
                get_result_banner(&board, &game)
            ),
            (
                Node {
                    display: Display::Grid,
                    max_height: percent(24),
                    grid_template_columns: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
                }
                Children [
                    #ResultText
                    ScrollPosition::default()
                    ModalScrollArea
                    Node {
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::axes(percent(10), percent(0)),
                        overflow: Overflow::scroll_y(),
                    }
                    Children [
                        Text::new(finish_time)
                        TextFont {
                            font_size: FontSize::Rem(1.125),
                        }
                        TextColor(GREEN_COLOR)
                        TextLayout::justify(Justify::Center)
                    ],

                    // Scrollbar for the content
                    Node {
                        min_width: px(12),
                    }
                    BackgroundColor(WHITE_VERY_TRANSPARENT_COLOR)
                    Scrollbar {
                        orientation: ControlOrientation::Vertical,
                        target: #ResultText,
                        min_thumb_length: 8.0,
                    }
                    Children [
                        BorderColor::all(LIGHT_BLUE_COLOR)
                        BackgroundColor(LIGHT_BLUE_COLOR)
                        ScrollbarThumb {
                            border_radius: BorderRadius::all(px(4)),
                            border: UiRect::all(px(1)),
                        }
                    ],
                ]
            ),
            share_button(),
            (
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
            game: Res<CurrentGame>,
            asset_server: Res<AssetServer>,
            mut layouts: ResMut<Assets<TextureAtlasLayout>>,| {
                let mins = game.elapsed.as_secs() / 60;
                let secs = game.elapsed.as_secs() % 60;
                let millis = game.elapsed.subsec_millis();
                let finish_time = format!("{}:{:02}", mins, secs);
                let time_emoji = if mins == 0 && secs == 10 && millis == 0 ||
                    mins == 0 && secs < 10 {
                    " 🙇👑🙇"
                } else if mins == 0 && secs == 30 && millis == 0 ||
                    mins == 0 && secs < 30 {
                    " 👑🙇"
                } else if mins == 0 && secs == 30 && millis == 0 ||
                    mins == 0 && secs < 30 {
                    " 👑"
                } else if mins == 1 && secs == 0  || mins < 1 {
                    " 🏅"
                } else if mins == 2 && secs == 0 && millis == 0 || mins < 2{
                    " 🏎️"
                } else if mins == 5 && secs == 0 && millis == 0 || mins < 5{
                    " 💚"
                } else {
                    ""
                };
                let mistake_text = if game.mistake_counter == 0 && game.already_found_but_guessed_counter == 0 {
                    "\n💎 No Mistakes"
                } else if game.mistake_counter == 0 {
                    "\n💯 No Incorrect Guesses"
                } else {
                    ""
                };
                match clipboard.set_text(format!("#DailySet - {}\n{}{}{}\nhttps://kfc35.github.io/daily_set/",
                    board.date, finish_time, time_emoji, mistake_text)) {
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
    if board.date == "2026/07/12" {
        ResultsBanner::STATIC_RESULTS_BANNER[5].scene()
    } else if board.date == "2026/07/11" {
        ResultsBanner::STATIC_RESULTS_BANNER[4].scene()
    } else if board.date == "2026/07/10" {
        ResultsBanner::STATIC_RESULTS_BANNER[3].scene()
    } else {
        let date_time = Utc::now().with_timezone(&chrono_tz::US::Eastern);
        let day_of_year = date_time.ordinal() as u64;
        let results_banner_seed =
            bytemuck::cast::<[u64; 2], [u8; 16]>([game.elapsed.as_secs(), day_of_year << 1]);
        let mut rng = rand_pcg::Pcg32::from_seed(results_banner_seed);

        // A time of less than 3 minutes deserves an animation
        if game.elapsed.as_secs() / 60 < 3 {
            ResultsBanner::ANIMATIONS[rng.random_range(0..ResultsBanner::ANIMATIONS.len())].scene()
        }
        // randomly give caturday image on a saturday
        else if date_time.weekday() == chrono::Weekday::Sat && rng.random() {
            ResultsBanner::HAPPY_CATURDAY.scene()
        } else {
            ResultsBanner::STATIC_RESULTS_BANNER
                [rng.random_range(0..ResultsBanner::STATIC_RESULTS_BANNER.len())]
            .scene()
        }
    }
}
