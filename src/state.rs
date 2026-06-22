use bevy::ecs::prelude::*;
use rand::{
    Rng, RngExt, SeedableRng,
    distr::{Distribution, StandardUniform},
};

extern crate alloc;
use alloc::vec::Vec;

/// Contains the current game state.
#[derive(Resource)]
pub struct GameState {
    /// The cards the user tries to make Sets out of.
    pub cards: [Card; 12],
    /// The current guess that the user is in the process of selecting.
    ///
    /// This vec must have max size 3. Once it has size 3, it is checked
    /// whether it is a valid set.
    pub current_guess: Vec<Entity>,
}

/// A card in a game of Set. Its contents can vary in four dimensions: [`Shape`],
/// [`Quantity`], [`Fill`], and [`Color`]. In a standard Set deck,
/// there is one of each unique card, for a total of 3^4 = 81 cards.
#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    shape: Shape,
    quantity: Quantity,
    fill: Fill,
    color: Color,
}

impl Card {
    pub fn new(shape: Shape, quantity: Quantity, fill: Fill, color: Color) -> Card {
        Card {
            shape,
            quantity,
            fill,
            color,
        }
    }
}

impl Distribution<Card> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card {
            shape: self.sample(rng),
            quantity: self.sample(rng),
            fill: self.sample(rng),
            color: self.sample(rng),
        }
    }
}

/// One of the four dimensions that a Set card can vary in.
///
/// Describes the shape that is on the card.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shape {
    Diamond,
    Oval,
    Squiggle,
}

impl Distribution<Shape> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.random_range(0..=2) {
            0 => Shape::Diamond,
            1 => Shape::Oval,
            _ => Shape::Squiggle,
        }
    }
}

/// One of the four dimensions that a Set card can vary in.
///
/// Describes the number of shapes that are on the card.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quantity {
    One,
    Two,
    Three,
}

impl Distribution<Quantity> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quantity {
        match rng.random_range(0..=2) {
            0 => Quantity::One,
            1 => Quantity::Two,
            _ => Quantity::Three,
        }
    }
}

/// One of the four dimensions that a Set card can vary in.
///
/// Describes the inside of the shape(s) that are on the card.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fill {
    Empty,
    Dashed,
    Filled,
}

impl Distribution<Fill> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fill {
        match rng.random_range(0..=2) {
            0 => Fill::Empty,
            1 => Fill::Dashed,
            _ => Fill::Filled,
        }
    }
}

/// One of the four dimensions that a Set card can vary in.
///
/// Describes the color of the shape(s) that are on the card.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Blue,
    Gold,
    Pink,
}

impl Distribution<Color> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.random_range(0..=2) {
            0 => Color::Blue,
            1 => Color::Gold,
            _ => Color::Pink,
        }
    }
}

/// Initializes the game state.
pub fn initialize_game_state(mut commands: Commands) {
    let state = GameState {
        cards: randomize_initial_cards(),
        current_guess: vec![],
    };
    commands.insert_resource(state);
}

fn randomize_initial_cards() -> [Card; 12] {
    let mut rng = rand_pcg::Pcg32::from_seed([
        22, 6, 255, 8, 110, 150, 35, 88, 140, 203, 92, 174, 244, 39, 4, 5,
    ]);

    // first select a random card
    let initial_card = rng.sample(StandardUniform);
    // pick how the next two cards in the set should be chosen
    // pick whether a set should be made with an existing card or not.

    // If it should, choose a card, and then pick how the next two cards should be chosen.
    // If it should not, generate another random set
    [
        initial_card,
        initial_card,
        initial_card,
        initial_card,
        initial_card,
        initial_card,
        initial_card,
        initial_card,
        initial_card,
        initial_card,
        initial_card,
        initial_card,
    ]
}
