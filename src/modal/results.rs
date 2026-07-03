use bevy::{
    asset::{AssetServer, Assets},
    camera::visibility::Visibility,
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
    AnimatedImageNode, AnimationTimer, DEFAULT_BACKGROUND_COLOR, GREEN_COLOR, GameState, Modal,
    TEXT_OVER_COLOR, TEXT_PRESS_COLOR, on_handler_style_button_text, share_button,
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
pub fn spawn(commands: &mut Commands, state: &Res<GameState>) {
    let mins = state.elapsed.as_secs() / 60;
    let secs = state.elapsed.as_secs() % 60;
    let finish_time = format!(
        "You finished the Daily Set for {} in {}:{:02}!",
        state.date, mins, secs
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
            left: percent(5),
            top: percent(5),
            height: percent(90),
            width: percent(90),
            border: px(5),
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
                get_result_banner(&state)
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
                    border: UiRect::all(px(5))
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                }
                BorderColor::all(GREEN_COLOR)
                on_handler_style_button_text::<Over>(TEXT_OVER_COLOR)
                on_handler_style_button_text::<Press>(TEXT_PRESS_COLOR)
                on_handler_style_button_text::<Release>(TEXT_OVER_COLOR)
                on_handler_style_button_text::<Out>(GREEN_COLOR)
                on(|event: On<Pointer<Click>>,
                    mut commands: Commands,
                    parent_q: Query<&ChildOf>| {
                    commands.entity(parent_q.root_ancestor(event.entity)).insert(Visibility::Hidden);
                })
                Children [
                    Text::new("Close")
                    TextFont {
                        font_size: FontSize::Px(30.0),
                    }
                    TextColor(GREEN_COLOR)
                ]
            )
        ]
    });
}

/// Determines which result banner to give based on the game state.
fn get_result_banner(state: &Res<GameState>) -> Box<dyn Scene> {
    if state.elapsed.as_secs() / 60 >= 5 {
        Box::new(bsn! {
            ImageNode {
                image: "results_banner/nice_try.png"
            }
        })
    } else if state.date == "2026/07/04" {
        Box::new(bsn! {
            ImageNode {
                image: "results_banner/happy_caturday_perched.png"
            }
        })
    } else if state.date == "2026/07/03" {
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
    } else if state.date == "2026/07/02" {
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
