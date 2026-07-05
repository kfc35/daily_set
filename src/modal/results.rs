use bevy::{
    asset::{AssetServer, Assets},
    camera::visibility::Visibility,
    clipboard::Clipboard,
    ecs::prelude::*,
    image::{TextureAtlas, TextureAtlasLayout},
    math::UVec2,
    picking::prelude::*,
    scene::prelude::*,
    text::{FontSize, TextColor, TextFont},
    time::{Timer, TimerMode},
    ui::prelude::*,
    ui_widgets::Button,
};

use crate::{
    AnimatedImageNode, AnimationTimer, CurrentGame, DEFAULT_BACKGROUND_COLOR, GREEN_COLOR,
    GameBoard, Modal, TEXT_OVER_COLOR, TEXT_PRESS_COLOR, on_handler_style_button_image,
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
    let mins_plural = if mins != 1 { "s" } else { "" };
    let secs_plural = if secs != 1 { "s" } else { "" };
    let precise_time = format!(
        "{mins} min{mins_plural} and {secs:02}.{:03} sec{secs_plural}",
        game.elapsed.subsec_millis(),
    );
    let finish_time = format!(
        "You finished the Daily Set for {}!\n Finish Time: {precise_time}",
        board.date,
    );

    commands.spawn_scene(bsn! {
        Modal
        ResultsModal
        ZIndex(2) // Higher than the How To Play Modal.
        Node {
            display: Display::Grid,
            grid_template_rows: vec![
                GridTrack::flex(3.),
                GridTrack::flex(1.),
                GridTrack::flex(1.),
                GridTrack::flex(1.),
            ]
            top: percent(5),
            height: percent(90),
            width: percent(100),
            border: px(5),
            padding: UiRect::axes(percent(5), percent(0)),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
        }
        BorderColor::all(GREEN_COLOR)
        BackgroundColor(DEFAULT_BACKGROUND_COLOR)
        Children [
            (
                Node {
                    width: vw(70)
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                }
                get_result_banner(&board, &game)
            ),
            (
                Node {
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                }
                Children [
                    Text::new(finish_time)
                    TextFont {
                        font_size: FontSize::Px(30.0),
                    }
                    TextColor(GREEN_COLOR)
                ]
            ),
            share_button(),
            (
                Button
                Node {
                    border: UiRect::all(px(5)),
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
            border: UiRect::all(px(5))
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
    if game.elapsed.as_secs() / 60 >= 5 {
        Box::new(bsn! {
            ImageNode {
                image: "results_banner/nice_try.png"
            }
        })
    } else if board.date == "2026/07/05" {
        Box::new(bsn! {
            ImageNode {
                image: "results_banner/youre_a_diamond.png"
            }
        })
    } else if board.date == "2026/07/04" {
        Box::new(bsn! {
            ImageNode {
                image: "results_banner/happy_caturday_perched.png"
            }
        })
    } else if board.date == "2026/07/03" {
        Box::new(bsn! {
            template(|context| {
                let layout = TextureAtlasLayout::from_grid(UVec2::new(128, 32), 1, 4, None, None);
                let layout_handle = context.resource_mut::<Assets<TextureAtlasLayout>>().add(layout);
                let texture_atlas = TextureAtlas {
                    layout: layout_handle,
                    index: 0,
                };
                Ok(ImageNode {
                    image: context.resource::<AssetServer>().load("results_banner/goal.png"),
                    texture_atlas: Some(texture_atlas),
                    ..Default::default()
                })
            })
            AnimatedImageNode(4)
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
        })
    } else if board.date == "2026/07/02" {
        Box::new(bsn! {
            ImageNode {
                image: "results_banner/well_done.png"
            }
        })
    } else {
        Box::new(bsn! {
            ImageNode {
                image: "results_banner/congratulations.png"
            }
        })
    }
}
