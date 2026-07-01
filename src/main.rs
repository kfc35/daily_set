use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    asset::{AssetMetaCheck, AssetPlugin, AssetServer, RenderAssetUsages},
    camera::{Camera2d, visibility::Visibility},
    ecs::prelude::*,
    image::{ImageLoaderSettings, ImagePlugin, ImageSamplerDescriptor},
    picking::prelude::*,
    prelude::PluginGroup,
    scene::prelude::*,
    text::{FontSize, TextColor, TextFont},
    ui::prelude::*,
    ui_widgets::Button,
};

mod state;
use state::{Card, Color, Fill, GameState, Quantity, Shape};

const TEXT_COLOR: bevy::color::Color = bevy::color::Color::srgb(0., 158. / 255., 115. / 255.);
const TEXT_OVER_COLOR: bevy::color::Color = bevy::color::Color::srgb(240. / 255., 228. / 255., 66. / 255.);
const TEXT_PRESS_COLOR: bevy::color::Color = bevy::color::Color::srgb(230. / 255., 159. / 255., 0. / 255.);


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
            Update,
            check_current_guess.run_if(|state: Res<GameState>| state.current_guess.len() >= 3),
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
            BorderColor::all(TEXT_COLOR)
            on(|event: On<Pointer<Over>>, mut commands: Commands,| {
                commands.entity(event.entity).insert(BorderColor::all(TEXT_OVER_COLOR));
            })
            on(|event: On<Pointer<Press>>, mut commands: Commands,| {
                commands.entity(event.entity).insert(BorderColor::all(TEXT_PRESS_COLOR));
            })
            on(|event: On<Pointer<Out>>, mut commands: Commands,| {
                commands.entity(event.entity).insert(BorderColor::all(TEXT_COLOR));
            })
            on(|_: On<Pointer<Click>>,
                mut menu_screen: Query<&mut Visibility, (With<StartScreen>, Without<GameScreen>)>,
                mut game_screen: Query<&mut Visibility, (With<GameScreen>, Without<StartScreen>)>| {
                *menu_screen.single_mut().unwrap() = Visibility::Hidden;
                *game_screen.single_mut().unwrap() = Visibility::Visible;
            })
            Children [
                Node {
                    // If this uses percent(), it's a little bugged.
                    // Should probably investigate why.
                    width: vw(33),
                    height: percent(100),
                    min_height: percent(25)
                }
                ImageNode {
                    image: "start_button.png"
                }
            ],
            // Date
            Node {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center
            }
            Children [
                Text::new(date)
                TextFont {
                    font_size: FontSize::Px(30.0),
                }
                TextColor(TEXT_COLOR)
            ]
        ]
    }
}

fn initialize_ui(mut commands: Commands, state: Res<GameState>) {
    commands.queue_spawn_scene(bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::SpaceAround
        }
        Children [ card_buttons(&state), score() ]
        GameScreen
        Visibility::Hidden
    });
}

fn score() -> impl Scene {
    bsn! {
        Node {
            top: px(50),
            min_width: percent(40)
        }
        Children [
            (
                Score
                ImageNode {
                    image: "score/0_of_6.png"
                }
            )
        ]
    }
}

fn card_buttons(state: &Res<GameState>) -> impl Scene {
    bsn! {
        Node {
            display: Display::Grid,
            min_width: percent(60),
            max_width: percent(100),
            top: px(50),
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
            width: percent(100),
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
            println!("I'm being clicked");
            if let Ok(idx) = state.current_guess.binary_search(&event.entity) {
                state.current_guess.remove(idx);
                commands.entity(event.entity).remove::<BorderColor>();
            } else {
                state.current_guess.push(event.entity);
                state.current_guess.sort();
                commands.entity(event.entity).insert(BorderColor::all(bevy::color::palettes::css::GREEN));
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

fn check_current_guess(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    cards_query: Query<&Card>,
    mut score: Query<&mut Text, With<Score>>,
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
        *score.single_mut().unwrap() = Text::new(format!("{}/6", state.found_sets.len()));
    }
    state.current_guess.clear();
}
