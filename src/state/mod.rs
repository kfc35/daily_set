use bevy::ecs::prelude::*;
use bevy::reflect::{Reflect, std_traits::ReflectDefault};
use core::time::Duration;
use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};

extern crate alloc;
use alloc::vec::Vec;

pub mod init;

/// Contains the current game state.
#[derive(Resource)]
pub struct GameState {
    /// The cards the user tries to make Sets out of.
    pub cards: [Card; 12],
    /// The six sets that exist among the cards, i.e. the answer key.
    ///
    /// Each individual set is sorted.
    sets: [[Card; 3]; 6],
    /// The current guess that the user is in the process of selecting.
    ///
    /// This vec must have max size 3. Once it has size 3, it should be checked
    /// whether it is a valid set and cleared.
    pub current_guess: Vec<Entity>,
    /// The sets which the user has found so far.
    pub found_sets: Vec<[Card; 3]>,
    /// The date of the game in this game state, formatted as "%Y/%m/%d" i.e. 2026/06/30.
    /// This is used for display and for figuring out whether this game state is stale.
    pub date: String,
    /// The duration of active gameplay.
    pub elapsed: Duration,
    /// Whether this game is active (i.e. playing on the game screen)
    pub is_active: bool,
}

impl GameState {
    /// Check whether sets contains the guess.
    ///
    /// ## Panics
    /// `guess` must be sorted, or else this function will panic.
    pub fn contains_guess(&self, guess: &[Card; 3]) -> bool {
        if guess.is_sorted() {
            self.sets.contains(guess)
        } else {
            panic!("Must sort the set before checking that it is in GameState.sets.")
        }
    }
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
