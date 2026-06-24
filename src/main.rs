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

fn initialize_ui(mut commands: Commands, state: Res<GameState>) {
    commands.spawn(Camera2d);
    commands.queue_spawn_scene(bsn! {
        Node {
            width: percent(100),
            height: percent(100),
        }
        Children [ card_buttons(&state) ]
    });
}

fn card_buttons(state: &Res<GameState>) -> impl Scene {
    bsn! {
        Node {
            display: Display::Grid,
            min_width: percent(60),
            max_width: percent(100),

            top: px(50),
            grid_template_rows: vec![RepeatedGridTrack::fr(4, 1.)],
        }
        Children [
            card_row(&state.cards[0..=2], 50),
            card_row(&state.cards[3..=5], 150),
            card_row(&state.cards[6..=8], 250),
            card_row(&state.cards[9..=11], 350),
        ]
    }
}

fn card_row(cards: &[Card], top_px: u32) -> impl Scene {
    bsn! {
        Node {
            display: Display::Grid,
            width: percent(100),
            height: percent(15),
            grid_template_columns: vec![RepeatedGridTrack::fr(3, 1.)],
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
            max_width: px(150),
            max_height: px(100),
            border: px(5),
            border_radius: px(3),
        }
        ImageNode {
            image: card_to_asset_path(&card)
        }
        BackgroundColor(bevy::color::Color::WHITE)
        on(|event: On<Pointer<Click>>, mut commands: Commands, mut state: ResMut<GameState>| {
            if let Ok(idx) = state.current_guess.binary_search(&event.entity) {
                state.current_guess.remove(idx);
                commands.entity(event.entity).remove::<BorderColor>();
            } else {
                state.current_guess.push(event.entity);
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

fn check_current_guess(mut commands: Commands, mut state: ResMut<GameState>) {
    for entity in state.current_guess.iter() {
        commands.entity(*entity).remove::<BorderColor>();
    }
    state.current_guess.clear();
}
