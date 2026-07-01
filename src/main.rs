use bevy::{
    DefaultPlugins,
    app::{App, FixedUpdate, Startup},
    asset::{AssetMetaCheck, AssetPlugin, AssetServer, RenderAssetUsages},
    camera::{Camera2d, visibility::Visibility},
    clipboard::Clipboard,
    ecs::prelude::*,
    image::{ImageLoaderSettings, ImagePlugin, ImageSamplerDescriptor},
    picking::prelude::*,
    prelude::PluginGroup,
    scene::prelude::*,
    text::{FontSize, TextColor, TextFont, TextLayout},
    time::Time,
    ui::prelude::*,
    ui_widgets::Button,
};

mod state;
use state::{Card, Color, Fill, GameState, Quantity, Shape};

const DEFAULT_BACKGROUND_COLOR: bevy::color::Color =
    bevy::color::Color::srgb(40. / 255., 40. / 255., 40. / 255.);
const GREEN_COLOR: bevy::color::Color = bevy::color::Color::srgb(0., 158. / 255., 115. / 255.);
const LIGHT_BLUE_COLOR: bevy::color::Color =
    bevy::color::Color::srgb(86. / 255., 180. / 255., 233. / 255.);
const TEXT_OVER_COLOR: bevy::color::Color =
    bevy::color::Color::srgb(240. / 255., 228. / 255., 66. / 255.);
const TEXT_PRESS_COLOR: bevy::color::Color =
    bevy::color::Color::srgb(230. / 255., 159. / 255., 0. / 255.);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    // All of the assets are pixel art, so pixelated looks best.
                    default_sampler: ImageSamplerDescriptor::nearest(),
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                }),
        )
        .add_systems(
            Startup,
            (state::initialize_game_state, initialize_ui, setup).chain(),
        )
        .add_systems(
            FixedUpdate,
            check_current_guess.run_if(|state: Res<GameState>| state.current_guess.len() >= 3),
        )
        .add_systems(
            FixedUpdate,
            increment_elapsed.run_if(|state: Res<GameState>| state.is_active),
        )
        .add_systems(
            FixedUpdate,
            end_game.run_if(|state: Res<GameState>, has_run: Local<bool>| {
                state.found_sets.len() == 6 && run_once(has_run)
            }),
        )
        .run();
}

/// Marker component for the text node containing the number of sets the user has successfully found.
#[derive(Component, Clone, Default)]
struct Score;

/// Marker component for the start screen
#[derive(Component, Clone, Default)]
struct StartScreen;

/// Marker component for main game screen
#[derive(Component, Clone, Default)]
struct GameScreen;

/// Marker component for the start button image
#[derive(Component, Clone, Default)]
struct StartButtonImage;

/// Marker component for the GameOver Text section
#[derive(Component, Clone, Default)]
struct GameOver;

/// Marker component for the Modal
#[derive(Component, Clone, Default)]
struct Modal;

fn setup(mut commands: Commands, state: Res<GameState>) {
    commands.spawn(Camera2d);
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

fn initialize_ui(mut commands: Commands, state: Res<GameState>) {
    commands.queue_spawn_scene(bsn! {
        Node {
            display: Display::Grid,
            grid_template_columns: vec![
                GridTrack::flex(2.),
                GridTrack::flex(1.),
            ],
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::SpaceAround
        }
        Children [ card_buttons(&state), score() ]
        GameScreen
        Visibility::Hidden
    });
}

fn card_buttons(state: &Res<GameState>) -> impl Scene {
    bsn! {
        Node {
            display: Display::Grid,
            margin: UiRect::top(vh(1)),
            grid_template_rows: vec![RepeatedGridTrack::flex(4, 1.)],
        }
        Children [
            card_row(&state.cards[0..=2]),
            card_row(&state.cards[3..=5]),
            card_row(&state.cards[6..=8]),
            card_row(&state.cards[9..=11]),
        ]
    }
}

fn card_row(cards: &[Card]) -> impl Scene {
    bsn! {
        Node {
            display: Display::Grid,
            max_height: percent(15),
            grid_template_columns: vec![RepeatedGridTrack::flex(3, 1.)],
        }
        Children [
            card_button(cards[0]),
            card_button(cards[1]),
            card_button(cards[2]),
        ]
    }
}

fn card_button(card: Card) -> impl Scene {
    bsn! {
        Button
        Node {
            border: px(5),
            border_radius: px(3),
        }
        template(move |context|
            Ok(ImageNode::new(
                context.resource_mut::<AssetServer>()
                    .load_builder()
                    .with_settings(|settings: &mut ImageLoaderSettings| {
                        settings.asset_usage = RenderAssetUsages::RENDER_WORLD;
                    })
                    .load(card_to_asset_path(&card))
            ))
        )
        Card {
            shape: {card.shape},
            quantity: {card.quantity},
            fill: {card.fill},
            color: {card.color},
        }
        BackgroundColor(bevy::color::Color::WHITE)
        on(|event: On<Pointer<Click>>, mut commands: Commands, mut state: ResMut<GameState>| {
            if let Ok(idx) = state.current_guess.binary_search(&event.entity) {
                state.current_guess.remove(idx);
                commands.entity(event.entity).remove::<BorderColor>();
            } else {
                state.current_guess.push(event.entity);
                state.current_guess.sort();
                commands.entity(event.entity).insert(BorderColor::all(GREEN_COLOR));
            }
        })
    }
}

fn card_to_asset_path(card: &Card) -> String {
    let shape = match card.shape {
        Shape::Diamond => "diamond",
        Shape::Squiggle => "squiggle",
        Shape::Oval => "oval",
    };
    let quantity = match card.quantity {
        Quantity::One => "1",
        Quantity::Two => "2",
        Quantity::Three => "3",
    };
    let fill = match card.fill {
        Fill::Empty => "E",
        Fill::Dashed => "D",
        Fill::Filled => "F",
    };
    let color = match card.color {
        Color::Blue => "oiblue",
        Color::Gold => "oigold",
        Color::Pink => "oipink",
    };
    format!("card/{shape}/{shape}_{quantity}_{fill}_{color}.png")
}

fn score() -> impl Scene {
    bsn! {
        Score
        Node {
            display: Display::Grid,
            margin: UiRect::top(vh(1)),
            grid_template_rows: vec![
                // The Score
                GridTrack::flex(2.),
                // The Sets found so far
                RepeatedGridTrack::flex(6, 1.),
                // Time Result and Copy Paste
                GridTrack::flex(4.)
            ]
        }
        Children [
            (
                ImageNode {
                    image: "score/0_of_6.png"
                }
            ),
            (), (), (), (), (), (),
            (
                GameOver
                Visibility::Hidden
            )
        ]
    }
}

fn check_current_guess(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
    cards_query: Query<&Card>,
    score: Query<&Children, With<Score>>,
) {
    for entity in state.current_guess.iter() {
        commands.entity(*entity).remove::<BorderColor>();
    }
    let mut guess: [Card; 3] = state
        .current_guess
        .iter()
        .map(|entity| *cards_query.get(*entity).unwrap())
        .collect::<Vec<Card>>()
        .try_into()
        .unwrap();
    guess.sort();
    if state.contains_guess(&guess) && !state.found_sets.contains(&guess) {
        state.found_sets.push(guess);
        let children = score.single().unwrap();
        // The first child is always the score image
        commands
            .entity(*children.first().unwrap())
            .insert(ImageNode::new(
                asset_server.load(format!("score/{}_of_6.png", state.found_sets.len())),
            ));
        // The following children are reserved for the found sets.
        commands
            .entity(*children.get(state.found_sets.len()).unwrap())
            .apply_scene(bsn! {
                Node {
                    display: Display::Grid,
                    grid_template_columns: vec![RepeatedGridTrack::flex(3, 1.)],
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    border: UiRect::all(px(5))
                }
                BackgroundColor(bevy::color::Color::WHITE)
                BorderColor::all(GREEN_COLOR)
                Children [
                    Node {
                        padding: UiRect::right(px(5))
                    }
                    ImageNode {
                        image: card_to_asset_path(&guess[0])
                    },
                    ImageNode {
                        image: card_to_asset_path(&guess[1])
                    },
                    Node {
                        padding: UiRect::left(px(5))
                    }
                    ImageNode {
                        image: card_to_asset_path(&guess[2])
                    },
                ]
            });
        if state.found_sets.len() == 6 {
            state.is_active = false;
        }
    }
    state.current_guess.clear();
}

fn increment_elapsed(mut state: ResMut<GameState>, time: Res<Time>) {
    state.elapsed += time.delta();
}

fn end_game(mut commands: Commands, state: Res<GameState>, query: Query<Entity, With<GameOver>>) {
    let mut ec = commands.entity(query.single().unwrap());
    let mins = state.elapsed.as_secs() / 60;
    let secs = state.elapsed.as_secs() % 60;
    let mins_plural = if mins != 1 { "" } else { "s" };
    let finish_time = format!(
        "You finished the Daily Set for {} in {}:{}!",
        state.date, mins, secs
    );
    let precise_time = format!(
        "{} min{} {}.{} secs",
        mins,
        mins_plural,
        secs,
        state.elapsed.subsec_millis()
    );
    let elapsed = format!("Finished in {precise_time}");

    ec.apply_scene(bsn! {
        Node
        Children [
            Text::new(elapsed)
            TextFont {
                font_size: FontSize::Px(30.0),
            }
            TextColor(GREEN_COLOR)
        ]
        Visibility::Visible
    });

    commands.spawn_scene(bsn! {
        Modal
        ZIndex(1)
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
                ImageNode {
                    image: "congratulations.png"
                }
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
            (
                Button
                Node {
                    border: UiRect::all(px(5))
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                }
                BorderColor::all(LIGHT_BLUE_COLOR)
                on(|event: On<Pointer<Over>>,
                    mut commands: Commands,
                    children_query: Query<&Children>,
                    text: Query<&mut TextColor>| {
                    commands.entity(event.entity).insert(BorderColor::all(TEXT_OVER_COLOR));
                    let Some(text_entity) = children_query
                        .iter_descendants(event.entity)
                        .find(|e| text.contains(*e))
                    else {
                        return;
                    };
                    commands.entity(text_entity).insert(TextColor(TEXT_OVER_COLOR));
                })
                on(|event: On<Pointer<Press>>,
                    mut commands: Commands,
                    children_query: Query<&Children>,
                    text: Query<&mut TextColor>| {
                    commands.entity(event.entity).insert(BorderColor::all(TEXT_PRESS_COLOR));
                    let Some(text_entity) = children_query
                        .iter_descendants(event.entity)
                        .find(|e| text.contains(*e))
                    else {
                        return;
                    };
                    commands.entity(text_entity).insert(TextColor(TEXT_PRESS_COLOR));
                })
                on(|event: On<Pointer<Release>>,
                    mut commands: Commands,
                    children_query: Query<&Children>,
                    text: Query<&mut TextColor>| {
                    commands.entity(event.entity).insert(BorderColor::all(TEXT_OVER_COLOR));
                    let Some(text_entity) = children_query
                        .iter_descendants(event.entity)
                        .find(|e| text.contains(*e))
                    else {
                        return;
                    };
                    commands.entity(text_entity).insert(TextColor(TEXT_OVER_COLOR));
                })
                on(|event: On<Pointer<Out>>,
                    mut commands: Commands,
                    children_query: Query<&Children>,
                    text: Query<&mut TextColor>| {
                    commands.entity(event.entity).insert(BorderColor::all(GREEN_COLOR));
                    let Some(text_entity) = children_query
                        .iter_descendants(event.entity)
                        .find(|e| text.contains(*e))
                    else {
                        return;
                    };
                    commands.entity(text_entity).insert(TextColor(GREEN_COLOR));
                })
                on(|event: On<Pointer<Click>>,
                    mut commands: Commands,
                    mut clipboard: ResMut<Clipboard>,
                    state: Res<GameState>,
                    children_query: Query<&Children>,
                    text: Query<&mut Text>| {
                        let mins = state.elapsed.as_secs() / 60;
                        let secs = state.elapsed.as_secs() % 60;
                        let finish_time = format!("{}:{}", mins, secs);
                        let Some(text_entity) = children_query
                            .iter_descendants(event.entity)
                            .find(|e| text.contains(*e))
                        else {
                            return;
                        };
                        match clipboard.set_text(format!("Daily Set - {} - Finish Time {}",
                            state.date, finish_time)) {
                            Ok(_) => {
                                commands.entity(text_entity).insert(Text::new("Copied!\nShare Results"));
                            }
                            _ => {
                                commands.entity(text_entity).insert(Text::new("Unable to copy results"));
                            }
                        }
                })
                Children [
                    Text::new("Share Results")
                    TextFont {
                        font_size: FontSize::Px(30.0),
                    }
                    TextColor(LIGHT_BLUE_COLOR)
                    TextLayout::justify(bevy::text::Justify::Center)
                ]
            ),
            (
                Button
                Node {
                    border: UiRect::all(px(5))
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                }
                BorderColor::all(GREEN_COLOR)
                on(|_: On<Pointer<Click>>,
                    mut commands: Commands,
                    modal_query: Query<Entity, With<Modal>>| {
                    commands.entity(modal_query.single().unwrap()).despawn();
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
