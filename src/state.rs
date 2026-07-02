use bevy::ecs::prelude::*;
use chrono::{Datelike, Utc};
use core::time::Duration;
use rand::{
    Rng, RngExt, SeedableRng,
    distr::{Distribution, StandardUniform},
    prelude::SliceRandom,
};

extern crate alloc;
use alloc::vec::Vec;

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
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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

/// Initializes the game state.
pub fn initialize_game_state(mut commands: Commands) {
    // The game will change seeds every day in Eastern time.
    let time = Utc::now().with_timezone(&chrono_tz::US::Eastern);
    let date = format!("{}", time.format("%Y/%m/%d"));
    let year = time.year() as u64;
    let day_of_year = time.ordinal() as u64;
    // the last bit is shifted to the left since Pcg32 drops the last bit from the seed.
    let seed = bytemuck::cast::<[u64; 2], [u8; 16]>([year, day_of_year << 1]);

    let (cards, sets) = initialize_cards(seed);
    let state = GameState {
        cards,
        sets,
        current_guess: vec![],
        found_sets: vec![],
        date,
        is_active: false,
        elapsed: Default::default(),
    };
    println!("{sets:?}");
    commands.insert_resource(state);
}

fn initialize_cards(seed: [u8; 16]) -> ([Card; 12], [[Card; 3]; 6]) {
    let mut rng = rand_pcg::Pcg32::from_seed(seed);
    let mut cards: Vec<Card> = vec![];
    let mut sets = vec![];

    // Randomly generate the first set.
    let mut first_set = generate_set(&mut rng);
    // Sets must be sorted before they are pushed so that sets.contains() works correctly.
    first_set.sort();
    sets.push(first_set);
    for card in first_set {
        cards.push(card);
    }

    // Add a random fourth card and set up potential sets that can be created.
    let mut fourth_card = first_set[0];
    while cards.contains(&fourth_card) {
        fourth_card = rng.sample(StandardUniform);
    }
    cards.push(fourth_card);
    let mut almost_complete_sets: Vec<([Card; 2], Card)> = first_set
        .iter()
        .map(|&card| {
            (
                [card, fourth_card],
                find_card_completing_set(card, fourth_card),
            )
        })
        .collect();

    while sets.len() < 6 {
        let must_create_set = 12 - cards.len() == 6 - sets.len();
        let new_card = if must_create_set || rng.random() {
            // The number of cards left to add is equal to the number of sets we have to create.
            // Or, we randomly decided to add a card that completes a set.
            let index = rng.random_range(0..almost_complete_sets.len());
            almost_complete_sets[index].1
        } else {
            // Add any random card. It most likely will not complete a set.
            // If it does, it is not a problem!
            let mut card = first_set[0];
            while cards.contains(&card) {
                card = rng.sample(StandardUniform);
            }
            card
        };

        // Gather the new set(s) that this new_card completes.
        let indices_and_pairs: Vec<(usize, [Card; 2])> = almost_complete_sets
            .iter()
            .enumerate()
            .filter(|(_, (_, card))| new_card == *card)
            .map(|(index, (pair, _))| (index, *pair))
            .collect();

        let new_sets: Vec<[Card; 3]> = indices_and_pairs
            .iter()
            .map(|(_, pair)| {
                let mut set = [pair[0], pair[1], new_card];
                set.sort();
                set
            })
            .filter(|set| !sets.contains(set))
            .collect();

        if sets.len() + new_sets.len() > 6 || (must_create_set && new_sets.is_empty()) {
            // re-roll. This card either completes more sets than we need or
            // does not complete a set we need.
            continue;
        }

        // Update almost_complete_sets:
        // - Remove the sets that we just completed.
        // - Add the new combinations of cards that can be made with the new card.
        for index in indices_and_pairs.into_iter().map(|(i, _)| i).rev() {
            // .rev() so that we can swap_remove with O(1) perf
            almost_complete_sets.swap_remove(index);
        }
        // Cards that are not in any of the new_sets with the new_card.
        let other_cards: Vec<Card> = cards
            .iter()
            .filter(|card| new_sets.iter().all(|set| !set.contains(card)))
            .copied()
            .collect();
        for other in other_cards {
            almost_complete_sets
                .push(([new_card, other], find_card_completing_set(new_card, other)));
        }

        // Update cards and sets with the new additions.
        cards.push(new_card);
        for new_set in new_sets.into_iter() {
            sets.push(new_set);
        }
    }

    // The while loop ensures that we will have 6 sets by this point, but
    // it will not ensure that we have enough cards.
    if 12 - cards.len() > 0 {
        // Have to pad with new cards that are:
        // - Not duplicates
        // - Won't complete any set inadvertently
        let mut cannot_add: Vec<Card> = almost_complete_sets
            .into_iter()
            .map(|(_, card)| card)
            .collect();
        cannot_add.sort();
        while 12 - cards.len() > 0 {
            let card: Card = rng.sample(StandardUniform);
            if cannot_add.binary_search(&card).is_err() && !cards.contains(&card) {
                cards.push(card);
            }
        }
    }

    cards.shuffle(&mut rng);
    (cards.try_into().unwrap(), sets.try_into().unwrap())
}

/// Randomly generates a Set of cards.
fn generate_set<R: Rng + ?Sized>(mut rng: &mut R) -> [Card; 3] {
    // The first card is randomly generated.
    let card: Card = rng.sample(StandardUniform);

    generate_set_with_card(&mut rng, card)
}

/// Randomly generates a Set of cards containing the provided `card`.
fn generate_set_with_card<R: Rng + ?Sized>(mut rng: &mut R, card: Card) -> [Card; 3] {
    // Decide how the next two cards in the same set should be chosen
    let (mut same_shape, mut same_quantity, mut same_fill, mut same_color) =
        (true, true, true, true);
    // The set cannot be one where all of its qualities are equal.
    while same_shape && same_quantity && same_fill && same_color {
        (same_shape, same_quantity, same_fill, same_color) =
            (rng.random(), rng.random(), rng.random(), rng.random());
    }
    // For each aspect, the cards have to either be all the same or all different in that given aspect.
    let (second_card_shape, third_card_shape) = if same_shape {
        (card.shape, card.shape)
    } else {
        Shape::sample_std_uniform_excluding(&mut rng, card.shape)
    };
    let (second_card_quantity, third_card_quantity) = if same_quantity {
        (card.quantity, card.quantity)
    } else {
        Quantity::sample_std_uniform_excluding(&mut rng, card.quantity)
    };
    let (second_card_fill, third_card_fill) = if same_fill {
        (card.fill, card.fill)
    } else {
        Fill::sample_std_uniform_excluding(&mut rng, card.fill)
    };
    let (second_card_color, third_card_color) = if same_color {
        (card.color, card.color)
    } else {
        Color::sample_std_uniform_excluding(&mut rng, card.color)
    };
    let second_card = Card {
        shape: second_card_shape,
        quantity: second_card_quantity,
        fill: second_card_fill,
        color: second_card_color,
    };
    let third_card = Card {
        shape: third_card_shape,
        quantity: third_card_quantity,
        fill: third_card_fill,
        color: third_card_color,
    };

    [card, second_card, third_card]
}

/// Returns the third card that would complete the set given these two cards.
fn find_card_completing_set(first: Card, second: Card) -> Card {
    let shape = first.shape.get_third_to_complete_set(second.shape);
    let quantity = first.quantity.get_third_to_complete_set(second.quantity);
    let fill = first.fill.get_third_to_complete_set(second.fill);
    let color = first.color.get_third_to_complete_set(second.color);

    Card {
        shape,
        quantity,
        fill,
        color,
    }
}
