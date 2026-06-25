use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    camera::Camera2d,
    ecs::prelude::*,
    picking::prelude::*,
    scene::prelude::*,
    ui::prelude::*,
    ui_widgets::Button,
};

mod state;
use state::{Card, Color, Fill, GameState, Quantity, Shape};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (state::initialize_game_state, initialize_ui).chain(),
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

fn initialize_ui(mut commands: Commands, state: Res<GameState>) {
    commands.spawn(Camera2d);
    commands.queue_spawn_scene(bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::SpaceAround
        }
        Children [ card_buttons(&state), score() ]
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
                Text::new("0/6")
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
            padding: UiRect::axes(px(0), px(20))
        }
        ImageNode {
            image: card_to_asset_path(&card)
        }
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
