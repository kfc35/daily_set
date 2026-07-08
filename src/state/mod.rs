use bevy::ecs::prelude::*;
use bevy::reflect::{Reflect, std_traits::ReflectDefault};
use bevy::settings::{ReflectSettingsGroup, SettingsGroup};
use core::time::Duration;
use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};

extern crate alloc;
use alloc::vec::Vec;

pub mod game_board;

/// Contains the current game's state - data that is gathered directly from the user's
/// actions.
#[derive(Resource, Default)]
pub struct CurrentGame {
    /// The current guess that the user is in the process of selecting.
    ///
    /// This vec must have max size 3. Once it has size 3, it should be checked
    /// whether it is a valid set and cleared.
    // TODO this can probably be separated from the persisted current game state.
    // This is more ephemeral.
    pub current_guess: Vec<Entity>,
    /// The sets which the user has found so far for the game board,
    /// along with the duration at which it was found and the total number of mistakes
    /// at the time
    pub found_sets: Vec<FoundSet>,
    /// The duration of active gameplay.
    pub elapsed: Duration,
    /// Whether the game has started.
    pub started: bool,
    /// How many mistakes were made.
    /// A mistake is when a NON-set was guessed.
    pub mistake_counter: u16,
    /// How many times already discovered sets were guessed multiple times.
    /// TODO rename to already_found_counter and have people clear their data on the site.
    pub already_guessed_counter: u16,
    /// Whether the game is active.
    /// This is set to false whenever the game is finished (or detected as such upon load).
    pub active: bool,
}

/// Contains stats for the game across multiple sessions.
#[derive(Resource, Default, SettingsGroup, Reflect)]
#[reflect(Resource, Default, SettingsGroup)]
pub struct GameStats {
    /// A list of past game summaries.
    /// The most recent game summary is the last entry in this vec.
    pub summaries: Vec<GameSummary>,
}

#[derive(Reflect)]
pub struct GameSummary {
    /// The date of the finished game.
    pub date_of_board: String,
    /// The sets the user found in the order in which they found them,
    /// the duration into the game at when they were found,
    /// the amount of mistakes that occurred before finding
    /// that set (cumulative), and the amount of times the user
    /// guessed an already guessed set at that moment in time.
    pub sets: [FoundSet; 6],
}

#[derive(Reflect, Clone, Copy, Debug)]
pub struct FoundSet {
    pub cards: [Card; 3],
    pub elapsed: Duration,
    pub mistake_counter: u16,
    pub already_guessed_counter: u16,
}

/// A card in a game of Set. Its contents can vary in four dimensions: [`Shape`],
/// [`Quantity`], [`Fill`], and [`Color`]. In a standard Set deck,
/// there is one of each unique card, for a total of 3^4 = 81 cards.
// The Default derive does not make much sense for a card, but it is so that we can
// take advantage of bsn!
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Reflect)]
#[reflect(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct Card {
    pub shape: Shape,
    pub quantity: Quantity,
    pub fill: Fill,
    pub color: Color,
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
// The Default derive does not make much sense, but it is so that we can
// take advantage of bsn!
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Reflect)]
#[reflect(Clone, Default, Debug, PartialEq, PartialOrd)]
pub enum Shape {
    #[default]
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

impl Shape {
    /// Returns one of the two shapes that is not the provided `exclude` shape
    /// with the standard uniform distribution.
    pub fn sample_std_uniform_excluding<R: Rng + ?Sized>(
        rng: &mut R,
        exclude: Shape,
    ) -> (Shape, Shape) {
        let shapes = match exclude {
            Shape::Diamond => [Shape::Oval, Shape::Squiggle],
            Shape::Oval => [Shape::Diamond, Shape::Squiggle],
            Shape::Squiggle => [Shape::Diamond, Shape::Oval],
        };
        match rng.random_range(0..=1) {
            0 => (shapes[0], shapes[1]),
            _ => (shapes[1], shapes[0]),
        }
    }

    /// Returns the third shape that would complete the set given `self` and the `other` shape.
    /// If self and other are the same shape, the returned shape is the same shape.
    /// If self and other are two different shapes, the returned shape is the other third shape.
    pub fn get_third_to_complete_set(&self, other: Shape) -> Shape {
        match (self, other) {
            (Shape::Diamond, Shape::Oval) | (Shape::Oval, Shape::Diamond) => Shape::Squiggle,
            (Shape::Oval, Shape::Squiggle) | (Shape::Squiggle, Shape::Oval) => Shape::Diamond,
            (Shape::Diamond, Shape::Squiggle) | (Shape::Squiggle, Shape::Diamond) => Shape::Oval,
            // The shapes are the same, so just return the other.
            _ => other,
        }
    }
}

/// One of the four dimensions that a Set card can vary in.
///
/// Describes the number of shapes that are on the card.
// The Default derive does not make much sense, but it is so that we can
// take advantage of bsn!
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Reflect)]
#[reflect(Clone, Default, Debug, PartialEq, PartialOrd)]
pub enum Quantity {
    #[default]
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

impl Quantity {
    /// Returns one of the two quantities that is not the provided `exclude` quantity
    /// with the standard uniform distribution.
    pub fn sample_std_uniform_excluding<R: Rng + ?Sized>(
        rng: &mut R,
        exclude: Quantity,
    ) -> (Quantity, Quantity) {
        let quantities = match exclude {
            Quantity::One => [Quantity::Two, Quantity::Three],
            Quantity::Two => [Quantity::One, Quantity::Three],
            Quantity::Three => [Quantity::One, Quantity::Two],
        };
        match rng.random_range(0..=1) {
            0 => (quantities[0], quantities[1]),
            _ => (quantities[1], quantities[0]),
        }
    }

    /// Returns the third quantity that would complete the set given `self` and the `other` quantity.
    /// If self and other are the same quantity, the returned quantity is the same quantity.
    /// If self and other are two different quantities, the returned quantity is the other third quantity.
    pub fn get_third_to_complete_set(&self, other: Quantity) -> Quantity {
        match (self, other) {
            (Quantity::One, Quantity::Two) | (Quantity::Two, Quantity::One) => Quantity::Three,
            (Quantity::Two, Quantity::Three) | (Quantity::Three, Quantity::Two) => Quantity::One,
            (Quantity::One, Quantity::Three) | (Quantity::Three, Quantity::One) => Quantity::Two,
            // The quantities are the same, so just return the other.
            _ => other,
        }
    }
}

/// One of the four dimensions that a Set card can vary in.
///
/// Describes the inside of the shape(s) that are on the card.
// The Default derive does not make much sense, but it is so that we can
// take advantage of bsn!
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Reflect)]
#[reflect(Clone, Default, Debug, PartialEq, PartialOrd)]
pub enum Fill {
    #[default]
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

impl Fill {
    /// Returns one of the two fills that is not the provided `exclude` fill
    /// with the standard uniform distribution.
    pub fn sample_std_uniform_excluding<R: Rng + ?Sized>(
        rng: &mut R,
        exclude: Fill,
    ) -> (Fill, Fill) {
        let fills = match exclude {
            Fill::Empty => [Fill::Dashed, Fill::Filled],
            Fill::Dashed => [Fill::Empty, Fill::Filled],
            Fill::Filled => [Fill::Empty, Fill::Dashed],
        };
        match rng.random_range(0..=1) {
            0 => (fills[0], fills[1]),
            _ => (fills[1], fills[0]),
        }
    }

    /// Returns the third fill that would complete the set given `self` and the `other` fill.
    /// If self and other are the same fill, the returned fill is the same fill.
    /// If self and other are two different fills, the returned fill is the other third fill.
    pub fn get_third_to_complete_set(&self, other: Fill) -> Fill {
        match (self, other) {
            (Fill::Empty, Fill::Dashed) | (Fill::Dashed, Fill::Empty) => Fill::Filled,
            (Fill::Dashed, Fill::Filled) | (Fill::Filled, Fill::Dashed) => Fill::Empty,
            (Fill::Empty, Fill::Filled) | (Fill::Filled, Fill::Empty) => Fill::Dashed,
            // The fills are the same, so just return the other.
            _ => other,
        }
    }
}

/// One of the four dimensions that a Set card can vary in.
///
/// Describes the color of the shape(s) that are on the card.
// The Default derive does not make much sense, but it is so that we can
// take advantage of bsn!
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Reflect)]
#[reflect(Clone, Default, Debug, PartialEq, PartialOrd)]
pub enum Color {
    #[default]
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

impl Color {
    /// Returns one of the two colors that is not the provided `exclude` color
    /// with the standard uniform distribution.
    pub fn sample_std_uniform_excluding<R: Rng + ?Sized>(
        rng: &mut R,
        exclude: Color,
    ) -> (Color, Color) {
        let colors = match exclude {
            Color::Blue => [Color::Gold, Color::Pink],
            Color::Gold => [Color::Blue, Color::Pink],
            Color::Pink => [Color::Blue, Color::Gold],
        };
        match rng.random_range(0..=1) {
            0 => (colors[0], colors[1]),
            _ => (colors[1], colors[0]),
        }
    }

    /// Returns the third color that would complete the set given `self` and the `other` color.
    /// If self and other are the same color, the returned color is the same color.
    /// If self and other are two different colors, the returned color is the other third color.
    pub fn get_third_to_complete_set(&self, other: Color) -> Color {
        match (self, other) {
            (Color::Blue, Color::Gold) | (Color::Gold, Color::Blue) => Color::Pink,
            (Color::Gold, Color::Pink) | (Color::Pink, Color::Gold) => Color::Blue,
            (Color::Blue, Color::Pink) | (Color::Pink, Color::Blue) => Color::Gold,
            // The colors are the same, so just return the other.
            _ => other,
        }
    }
}
