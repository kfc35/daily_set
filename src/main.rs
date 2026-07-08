use bevy::{
    DefaultPlugins,
    app::{App, FixedUpdate, Startup, Update},
    asset::{AssetMetaCheck, AssetPlugin, AssetServer, Assets, RenderAssetUsages},
    camera::{
        Camera2d,
        visibility::{InheritedVisibility, Visibility},
    },
    ecs::prelude::*,
    image::{
        ImageLoaderSettings, ImagePlugin, ImageSamplerDescriptor, TextureAtlas, TextureAtlasLayout,
    },
    input::mouse::{AccumulatedMouseScroll, MouseScrollUnit},
    math::UVec2,
    picking::prelude::*,
    prelude::{Deref, DerefMut, PluginGroup},
    scene::prelude::*,
    settings::{SaveSettingsSync, SettingsPlugin},
    text::{FontSize, TextColor, TextFont, TextLayout},
    time::{Time, Timer},
    ui::prelude::*,
    ui_widgets::Button,
};

mod state;
use state::{
    Card, Color, CurrentGame, Fill, FoundSet, GameStats, GameSummary, Quantity, Shape,
    game_board::GameBoard,
};
mod modal;
use modal::results::ResultsModal;
mod loading_screen;
use loading_screen::LoadingScreen;
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
pub const SAVE_SETTINGS_INTERVAL_SECS: i64 = 3;
/// To support viewing the game on a website, small phones have a min width of 320px.
/// We use 310px here to be safe.
pub const MIN_WIDTH_PX_MOBILE: i32 = 310;
/// To support landscape mode.
pub const MIN_HEIGHT_PX_MOBILE: i32 = MIN_WIDTH_PX_MOBILE;

/// Marker component for the image node containing the number of sets the user has successfully found.
#[derive(Component, Clone, Default)]
struct Score;

/// Marker component for the node containing the found sets.
#[derive(Component, Clone, Default)]
struct FoundSets;

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

/// Systems that initialize state in the `Startup` schedule
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct StateInitSystems;

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
        .init_resource::<CurrentGame>()
        .add_plugins(SettingsPlugin::new(SETTINGS_APP_NAME))
        .add_systems(
            Startup,
            (
                loading_screen::spawn_loading_screen,
                state::game_board::init_game_board,
                update_current_game_if_already_solved,
            )
                .chain()
                .in_set(StateInitSystems),
        )
        .add_systems(
            Startup,
            (prep_game_screen, spawn_start_screen)
                .chain()
                .after(StateInitSystems),
        )
        .add_systems(Startup, modal::how_to_play::spawn)
        .add_systems(Update, (animate_images, update_scrollbar_with_scroll))
        .add_systems(
            FixedUpdate,
            check_current_guess.run_if(|game: Res<CurrentGame>| game.current_guess.len() >= 3),
        )
        // TODO maybe more clearly separate the pop modal portion out of end_game so that it can depend on game.active instead.
        .add_systems(
            FixedUpdate,
            increment_elapsed.run_if(|game: Res<CurrentGame>| {
                game.started && game.active && game.found_sets.len() < 6
            }),
        )
        .add_systems(
            FixedUpdate,
            end_game.run_if(|game: Res<CurrentGame>, has_run: Local<bool>| {
                game.found_sets.len() == 6 && run_once(has_run)
            }),
        )
        .run();
}

fn spawn_start_screen(
    mut commands: Commands,
    board: Res<GameBoard>,
    loading_screen: Query<Entity, With<LoadingScreen>>,
) {
    commands.spawn(Camera2d);
    start_screen::start_screen(&mut commands, &board);
    commands.entity(loading_screen.single().unwrap()).despawn();
}

fn prep_game_screen(mut commands: Commands, board: Res<GameBoard>, game: Res<CurrentGame>) {
    commands.queue_spawn_scene(bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::SpaceAround
        }
        Children [ card_buttons(&board), score_pane(&game) ]
        GameScreen
        Visibility::Hidden
    });
}

fn card_buttons(board: &Res<GameBoard>) -> impl Scene {
    bsn! {
        Node {
            // To ensure resizing for mobile doesnt look bad.
            min_width: px(MIN_WIDTH_PX_MOBILE),
            // Takes up 2/3 of the width ideally.
            // Subtract -1 percent so that there isn't random snapping
            // between views.
            width: percent(66),
            // If on its own row, take up the max width.
            max_width: percent(100),

            // Take up a quarter of the mobile screen space.
            min_height: percent(33),
            // Take up all of the height when it is in its own column
            max_height: percent(100),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            margin: UiRect::top(vh(1)),
            padding: UiRect::all(px(2)),
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
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
            height: percent(100),
            border: UiRect::all(percent(2)),
        }
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

fn score_pane(game: &Res<CurrentGame>) -> impl Scene {
    let score = game.found_sets.len();
    let image_path = format!("score/{}_of_6.png", score);
    bsn! {
        Node {
            // To ensure resizing for mobile doesnt look bad.
            min_width: px(MIN_WIDTH_PX_MOBILE),
            // Takes up 1/3 of the width ideally.
            width: percent(33),
            // If on its own row, take up the max width.
            max_width: percent(100),
            // Take up nearly 2/3 of the mobile screen space.
            // We subtract 1 or else dynamic resizing makes it snap oddly.
            min_height: percent(66),
            // Take up all of the height when it is in its own column
            max_height: percent(100),

            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            align_content: AlignContent::Start,

            margin: UiRect::top(vh(1)),
        }
        Children [
            (
                Score
                Node {
                    width: percent(50),
                    left: percent(25),
                    max_height: percent(30),
                }
                ImageNode {
                    image: image_path
                }
            ),
            found_sets_rows(&game.found_sets),
            game_over_section(&game)
        ]
    }
}

fn found_sets_rows(found_sets: &Vec<FoundSet>) -> impl Scene {
    let mut sets = found_sets
        .iter()
        .map(|found_set| Some(found_set.cards))
        .collect::<Vec<Option<[Card; 3]>>>();
    sets.resize(6, None);

    bsn! {
        FoundSets
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: percent(100),
            height: percent(48),
        }
        Children [
            found_set_row(sets[0]),
            found_set_row(sets[1]),
            found_set_row(sets[2]),
            found_set_row(sets[3]),
            found_set_row(sets[4]),
            found_set_row(sets[5])
        ]
    }
}

fn found_set_row(set: Option<[Card; 3]>) -> Box<dyn Scene> {
    set.map_or_else::<Box<dyn Scene>, _, _>(
        || {
            Box::new(bsn! {
                Node {
                    display: Display::Block,
                    height: percent(16),
                }
                // Visibility::Hidden
            })
        },
        |set| {
            Box::new(bsn! {
                Node {
                    display: Display::Grid,
                    width: percent(100),
                    height: percent(16),
                    grid_template_columns: vec![RepeatedGridTrack::flex(3, 1.)],
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    border: UiRect::all(px(2)),
                    padding: UiRect::all(px(2)),
                }
                Visibility::Inherited
                BackgroundColor(bevy::color::Color::WHITE)
                BorderColor::all(GREEN_COLOR)
                Children [
                    Node {
                        padding: UiRect::right(px(2))
                    }
                    ImageNode {
                        image: card_to_asset_path(&set[0])
                    },
                    ImageNode {
                        image: card_to_asset_path(&set[1])
                    },
                    Node {
                        padding: UiRect::left(px(2))
                    }
                    ImageNode {
                        image: card_to_asset_path(&set[2])
                    },
                ]
            })
        },
    )
}

fn game_over_section(game: &CurrentGame) -> Box<dyn Scene> {
    if game.found_sets.len() < 6 {
        Box::new(bsn! {
            GameOver
            Node {
                max_height: percent(10),
            }
            Visibility::Hidden
        })
    } else {
        let mins = game.elapsed.as_secs() / 60;
        let secs = game.elapsed.as_secs() % 60;
        let short_time = format!("{:02}:{:02}", mins, secs);
        let elapsed = format!("{short_time}");

        Box::new(bsn! {
            GameOver
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: percent(100),
                max_height: percent(10),
                justify_content: JustifyContent::Start,
                align_content: AlignContent::Start,
            }
            Children [
                // A shortened elapsed time message.
                (
                    Node {
                        width: percent(50),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                    }
                    Children [
                        Text::new(elapsed)
                        TextFont {
                            font_size: FontSize::Rem(1.5),
                        }
                        TextColor(GREEN_COLOR)
                        TextLayout::justify(bevy::text::Justify::Center)
                    ]
                ),

                // Reopen Finish Screen button
                (
                    Button
                    Node {
                        border: UiRect::all(px(5))
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        width: percent(50),
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
            Visibility::Inherited
        })
    }
}

fn check_current_guess(
    mut commands: Commands,
    board: ResMut<GameBoard>,
    mut game: ResMut<CurrentGame>,
    asset_server: Res<AssetServer>,
    cards_query: Query<&Card>,
    score: Query<Entity, With<Score>>,
    found_sets_q: Query<&Children, With<FoundSets>>,
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
    let already_found = !game
        .found_sets
        .iter()
        .any(|found_set| found_set.cards == guess);
    if board.contains_guess(&guess) && already_found {
        let (elapsed, mistake_counter, already_guessed_counter) = (
            game.elapsed,
            game.mistake_counter,
            game.already_guessed_counter,
        );
        game.found_sets.push(FoundSet {
            cards: guess,
            elapsed,
            mistake_counter,
            already_guessed_counter,
        });
        let children = found_sets_q.single().unwrap();
        // The first child is always the score image
        commands
            .entity(score.single().unwrap())
            .insert(ImageNode::new(
                asset_server.load(format!("score/{}_of_6.png", game.found_sets.len())),
            ));
        // The next child is reserved for the found sets.
        commands
            .entity(*children.get(game.found_sets.len() - 1).unwrap())
            .apply_scene(found_set_row(Some(guess)));
    } else if !already_found {
        game.mistake_counter += 1;
    } else {
        game.already_guessed_counter += 1;
    }

    game.current_guess.clear();
}

fn increment_elapsed(mut game: ResMut<CurrentGame>, time: Res<Time>) {
    game.elapsed += time.delta();
}

fn end_game(
    mut commands: Commands,
    board: Res<GameBoard>,
    mut game: ResMut<CurrentGame>,
    mut stats: ResMut<GameStats>,
    query: Query<Entity, With<GameOver>>,
) {
    // The modal spawns even if the game is not active.
    // Thus, it will pop up if the user is opening DailySet anew after having finished.
    modal::results::spawn(&mut commands, &board, game.as_ref());

    if game.active {
        let mut ec = commands.entity(query.single().unwrap());
        ec.apply_scene(game_over_section(game.as_ref()));

        if stats
            .summaries
            .last()
            .is_none_or(|summary| summary.date_of_board != board.date)
        {
            stats.summaries.push(GameSummary {
                date_of_board: board.date.clone(),
                sets: game
                    .found_sets
                    .iter()
                    .copied()
                    .collect::<Vec<FoundSet>>()
                    .try_into()
                    .unwrap(),
            });
        }
        commands.queue(SaveSettingsSync::Always)
    }
    game.active = false;
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

/// If the user has already solved today's game, it should not
/// prompt them to play again.
///
/// This system ensures that.
pub fn update_current_game_if_already_solved(
    mut game: ResMut<CurrentGame>,
    stats: Res<GameStats>,
    board: Res<GameBoard>,
) {
    // If the game has already been solved for the day, people can always revisit (even in multiple browser tabs)
    let Some(summary) = stats.summaries.last() else {
        return;
    };
    if summary.date_of_board == board.date {
        game.found_sets = summary.sets.to_vec();
        game.elapsed = summary.sets[5].elapsed;
        game.started = true;
    }
}

pub fn update_scrollbar_with_scroll(
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    mut query: Query<(
        &mut ScrollPosition,
        &Node,
        &ComputedNode,
        &InheritedVisibility,
    )>,
) {
    let scroll = match accumulated_mouse_scroll.unit {
        MouseScrollUnit::Line => {
            accumulated_mouse_scroll.delta.y * MouseScrollUnit::SCROLL_UNIT_CONVERSION_FACTOR
        }
        MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y,
    };

    for (mut scroll_pos, node, computed_node, visibility) in query.iter_mut() {
        if node.overflow.y == OverflowAxis::Scroll && visibility.get() {
            let max_offset = (computed_node.content_size() - computed_node.size())
                * computed_node.inverse_scale_factor();

            let max = if scroll > 0. {
                scroll_pos.y >= max_offset.y
            } else {
                scroll_pos.y <= 0.
            };

            if !max {
                scroll_pos.y += scroll;
            }

            // There should be only one scrollbar visible at a time.
            break;
        }
    }
}
