use bevy::{
    DefaultPlugins,
    app::{App, FixedUpdate, Startup, Update},
    asset::{AssetMetaCheck, AssetPlugin, AssetServer, Assets, RenderAssetUsages},
    camera::{Camera2d, visibility::Visibility},
    ecs::prelude::*,
    image::{
        ImageLoaderSettings, ImagePlugin, ImageSamplerDescriptor, TextureAtlas, TextureAtlasLayout,
    },
    math::UVec2,
    picking::prelude::*,
    prelude::{Deref, DerefMut, PluginGroup},
    scene::prelude::*,
    settings::SettingsPlugin,
    text::{FontSize, TextColor, TextFont, TextLayout},
    time::{Time, Timer},
    ui::prelude::*,
    ui_widgets::Button,
};

mod state;
use state::{Card, Color, CurrentGame, Fill, Quantity, Shape, game_board::GameBoard};
mod modal;
use modal::results::ResultsModal;
mod start_screen;

pub const SETTINGS_APP_NAME: &'static str = "com.github.kfc35.daily_set";
pub const DEFAULT_BACKGROUND_COLOR: bevy::color::Color =
    bevy::color::Color::srgb(40. / 255., 40. / 255., 40. / 255.);
pub const GREEN_COLOR: bevy::color::Color = bevy::color::Color::srgb(0., 158. / 255., 115. / 255.);
pub const LIGHT_BLUE_COLOR: bevy::color::Color =
    bevy::color::Color::srgb(86. / 255., 180. / 255., 233. / 255.);
pub const TEXT_OVER_COLOR: bevy::color::Color =
    bevy::color::Color::srgb(240. / 255., 228. / 255., 66. / 255.);
pub const TEXT_PRESS_COLOR: bevy::color::Color =
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
                    // Prevent 404's from happening on the web.
                    meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                }),
        )
        .add_plugins(SettingsPlugin::new(SETTINGS_APP_NAME))
        .add_systems(
            Startup,
            (state::game_board::init_game_board, initialize_ui, setup).chain(),
        )
        .add_systems(Startup, modal::how_to_play::spawn)
        .add_systems(Update, animate_images)
        .add_systems(
            FixedUpdate,
            check_current_guess.run_if(|game: Res<CurrentGame>| game.current_guess.len() >= 3),
        )
        .add_systems(
            FixedUpdate,
            increment_elapsed
                .run_if(|game: Res<CurrentGame>| game.started && game.found_sets.len() < 6),
        )
        .add_systems(
            FixedUpdate,
            end_game.run_if(|game: Res<CurrentGame>, has_run: Local<bool>| {
                game.found_sets.len() == 6 && run_once(has_run)
            }),
        )
        .run();
}

/// Marker component for the text node containing the number of sets the user has successfully found.
#[derive(Component, Clone, Default)]
struct Score;

/// Marker component for main game screen
#[derive(Component, Clone, Default)]
struct GameScreen;

/// Marker component for the GameOver Text section
#[derive(Component, Clone, Default)]
struct GameOver;

/// Marker component for an animated image node containing the number of frames
#[derive(Component, Clone, Default, Deref)]
pub struct AnimatedImageNode(usize);

/// Component to used for image animations
#[derive(Component, Clone, Default, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

/// Marker component for a Modal
#[derive(Component, Clone, Default)]
struct Modal;

fn setup(mut commands: Commands, board: Res<GameBoard>) {
    commands.spawn(Camera2d);
    start_screen::start_screen(&mut commands, &board);
}

fn initialize_ui(mut commands: Commands, board: Res<GameBoard>) {
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
        Children [ card_buttons(&board), score() ]
        GameScreen
        Visibility::Hidden
    });
}

fn card_buttons(board: &Res<GameBoard>) -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            margin: UiRect::top(vh(1)),
            justify_content: JustifyContent::Center
        }
        Children [
            card_row(&board.cards[0..=2]),
            card_row(&board.cards[3..=5]),
            card_row(&board.cards[6..=8]),
            card_row(&board.cards[9..=11]),
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
                context.resource::<AssetServer>()
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
        on(|event: On<Pointer<Click>>, mut commands: Commands, mut game: ResMut<CurrentGame>| {
            if let Ok(idx) = game.current_guess.binary_search(&event.entity) {
                game.current_guess.remove(idx);
                commands.entity(event.entity).remove::<BorderColor>();
            } else {
                game.current_guess.push(event.entity);
                game.current_guess.sort();
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
    board: ResMut<GameBoard>,
    mut game: ResMut<CurrentGame>,
    asset_server: Res<AssetServer>,
    cards_query: Query<&Card>,
    score: Query<&Children, With<Score>>,
) {
    for entity in game.current_guess.iter() {
        commands.entity(*entity).remove::<BorderColor>();
    }
    let mut guess: [Card; 3] = game
        .current_guess
        .iter()
        .map(|entity| *cards_query.get(*entity).unwrap())
        .collect::<Vec<Card>>()
        .try_into()
        .unwrap();
    guess.sort();
    if board.contains_guess(&guess) && !game.found_sets.contains(&guess) {
        game.found_sets.push(guess);
        let children = score.single().unwrap();
        // The first child is always the score image
        commands
            .entity(*children.first().unwrap())
            .insert(ImageNode::new(
                asset_server.load(format!("score/{}_of_6.png", game.found_sets.len())),
            ));
        // The following children are reserved for the found sets.
        commands
            .entity(*children.get(game.found_sets.len()).unwrap())
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
        if game.found_sets.len() == 6 {
            game.started = false;
        }
    }
    game.current_guess.clear();
}

fn increment_elapsed(mut game: ResMut<CurrentGame>, time: Res<Time>) {
    game.elapsed += time.delta();
}

fn end_game(
    mut commands: Commands,
    board: Res<GameBoard>,
    game: Res<CurrentGame>,
    query: Query<Entity, With<GameOver>>,
) {
    let mins = game.elapsed.as_secs() / 60;
    let secs = game.elapsed.as_secs() % 60;
    let short_time = format!("{}:{:02}", mins, secs);
    let elapsed = format!("Finish Time\n{short_time}");

    modal::results::spawn(&mut commands, &board, &game);

    let mut ec = commands.entity(query.single().unwrap());
    ec.apply_scene(bsn! {
        Node {
            display: Display::Grid,
            grid_template_rows: vec![RepeatedGridTrack::flex(2, 1.)]
        }
        Children [
            // A shortened elapsed time message.
            (
                Text::new(elapsed)
                TextFont {
                    font_size: FontSize::Px(30.0),
                }
                TextColor(GREEN_COLOR)
                TextLayout::justify(bevy::text::Justify::Center)
            ),

            // Reopen Finish Screen button
            (
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
                on(|_: On<Pointer<Click>>, query: Query<&mut Visibility, With<ResultsModal>>| {
                    modal::results::unhide(query);
                })
                // Unsure how to do this by just having to modify the texture_atlas of the ImageNode
                template(move |context| {
                    let layout = TextureAtlasLayout::from_grid(UVec2::new(48, 19), 1, 3, None, None);
                    let layout_handle = context.resource_mut::<Assets<TextureAtlasLayout>>().add(layout);
                    let texture_atlas = TextureAtlas {
                        layout: layout_handle,
                        index: 0,
                    };
                    Ok(ImageNode {
                        image: context.resource::<AssetServer>().load("menu/reopen_finish_screen.png"),
                        texture_atlas: Some(texture_atlas),
                        ..Default::default()
                    })
                })
            )
        ]
        Visibility::Visible
    });
}

/// Helper to attach an observer to an entity for the given Pointer Event `E` that changes:
/// the `BorderColor` of this entity and the `TextColor` this entity and its direct child to
/// the provided color.
pub fn on_handler_style_button_text<E>(text_and_border_color: bevy::color::Color) -> impl Scene
where
    E: core::fmt::Debug + Clone + bevy::reflect::Reflect,
{
    bsn! {
        Node
        on(move |event: On<Pointer<E>>,
            mut commands: Commands,
            children_query: Query<&Children>,
            text: Query<&mut TextColor>| {
            commands.entity(event.entity).insert(BorderColor::all(text_and_border_color));
            // TODO see if we can remove this particular if block if the button
            // is better formatted with the text on the same entity, not the parent.
            if let Some(text_entity) = children_query
                .iter_descendants(event.entity)
                .find(|e| text.contains(*e)) {
                commands.entity(text_entity).insert(TextColor(text_and_border_color));
            }
            if text.contains(event.entity) {
                commands.entity(event.entity).insert(TextColor(text_and_border_color));
            }
        })
    }
}

/// Helper to attach an observer to an entity for the given Pointer Event `E` that changes:
/// the `BorderColor` of this entity to the provided color and the `texture_atlas` of the
/// `ImageNode` on this entity and its direct child to use the provided index.
pub fn on_handler_style_button_image<E>(
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
            if let Some(Ok(mut image_node)) = children_query
                .iter_descendants(event.entity)
                .find(|e| image_q.contains(*e))
                .map(|e| image_q.get_mut(e))
                && let Some(atlas) = &mut image_node.texture_atlas {
              atlas.index = texture_atlas_index;
            }

            if let Some(Ok(mut image_node)) = image_q.get_mut(event.entity).into()
                && let Some(atlas) = &mut image_node.texture_atlas {
              atlas.index = texture_atlas_index;
            }
        })
    }
}

/// System to animate images tagged with an AnimatedImageNode
fn animate_images(
    time: Res<Time>,
    mut query: Query<
        (&AnimatedImageNode, &mut ImageNode, &mut AnimationTimer),
        With<AnimatedImageNode>,
    >,
) {
    for (length, mut image_node, mut timer) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut image_node.texture_atlas
        {
            atlas.index = if atlas.index + 1 == length.0 {
                0
            } else {
                atlas.index + 1
            };
        }
    }
}
