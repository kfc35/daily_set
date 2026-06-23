use bevy::ecs::prelude::*;
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
    /// The current guess that the user is in the process of selecting.
    ///
    /// This vec must have max size 3. Once it has size 3, it is checked
    /// whether it is a valid set.
    pub current_guess: Vec<Entity>,
}

/// A card in a game of Set. Its contents can vary in four dimensions: [`Shape`],
/// [`Quantity`], [`Fill`], and [`Color`]. In a standard Set deck,
/// there is one of each unique card, for a total of 3^4 = 81 cards.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub shape: Shape,
    pub quantity: Quantity,
    pub fill: Fill,
    pub color: Color,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    let (cards, _sets) = initialize_cards();
    let state = GameState {
        cards: cards,
        current_guess: vec![],
    };
    commands.insert_resource(state);
}

fn initialize_cards() -> ([Card; 12], [[Card; 3]; 6]) {
    let mut rng = rand_pcg::Pcg32::from_seed([
        22, 6, 255, 8, 110, 150, 35, 88, 140, 203, 92, 174, 244, 39, 4, 5,
    ]);
    let mut cards: Vec<Card> = vec![];
    let mut sets = vec![];

    let mut first_set = generate_set(&mut rng);
    first_set.sort();
    sets.push(first_set);
    for card in first_set {
        cards.push(card);
    }

    let mut second_set = first_set;
    while second_set == first_set {
        if rng.random() {
            // generate the second set with one of the cards from the first set.
            let index = rng.random_range(0..first_set.len());
            second_set = generate_set_with_card(&mut rng, first_set[index]);
            second_set.sort();
        } else {
            // generate the second set without considering any of the cards from the first set.
            second_set = generate_set(&mut rng);
            second_set.sort();
        }
    }
    sets.push(second_set);
    for card in second_set {
        if !cards.contains(&card) {
            cards.push(card);
        }
    }

    let mut almost_complete_sets: Vec<([Card; 2], Card)> = first_set
        .iter()
        .flat_map(|&first| {
            second_set.iter().map(move |&second| {
                if first < second {
                    [first, second]
                } else {
                    [second, first]
                }
            })
        })
        .map(|[first, second]| ([first, second], find_card_completing_set(first, second)))
        .collect();

    while 12 - cards.len() > 0 && sets.len() < 6 {
        // TODO end index needs to be updated depending on how many sets can be made
        // and how many cards are left.
        let choice = rng.random_range(0..3);
        match choice {
            0 => {
                // Make set(s) by accepting one of the almost complete sets
                let index = rng.random_range(0..almost_complete_sets.len());
                let (_, new_card) = almost_complete_sets[index];

                // Gather the set(s) that this new_card completes.
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
                    .filter(|set| sets.contains(set))
                    .collect();

                if sets.len() + new_sets.len() > 6 {
                    // re-roll everything
                    continue;
                }

                // Update almost_complete_sets.
                // Remove the sets that we just completed.
                // Add the new combinations of cards that can be made with the new card.
                for index in indices_and_pairs.into_iter().map(|(i, _)| i).rev() {
                    almost_complete_sets.swap_remove(index);
                }
                let filtered_cards: Vec<Card> = cards
                    .iter()
                    .filter(|card| sets.iter().all(|set| !set.contains(card)))
                    .map(|&card| card)
                    .collect();
                for other in filtered_cards {
                    let pair = if new_card < other {
                        [new_card, other]
                    } else {
                        [other, new_card]
                    };
                    almost_complete_sets.push((pair, find_card_completing_set(new_card, other)));
                }

                // Update cards and sets with the new additions.
                cards.push(new_card);
                for new_set in new_sets.into_iter() {
                    sets.push(new_set);
                }
            }
            1 => {
                // Make a random set from choosing one of the existing cards.
            }
            _ => {
                // Make a random set from the complete set of
            }
        }
    }

    if sets.len() == 6 && 12 - cards.len() > 0 {
        // Have to pad with new cards that are:
        // - Not duplicates
        // - Won't complete any set inadvertently
        let mut cannot_add: Vec<Card> = almost_complete_sets
            .into_iter()
            .map(|(_, card)| card)
            .collect();
        cannot_add.sort();
        while sets.len() == 6 && 12 - cards.len() > 0 {
            let card: Card = rng.sample(StandardUniform);
            if cannot_add.binary_search(&card).is_err() && !cards.contains(&card) {
                cards.push(card);
            }
        }
    }

    // If the number of sets left to create is every <= the amount of cards available, we have to
    // Smartly choose sets such that all 12 cards make up 6 sets.

    // Note: If there have been three sets that are made with completely disparate sets of cards,
    // It is possible that sets were made in between the cards. Youd have to check for them.

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
    let (same_shape, same_quantity, same_fill, same_color): (bool, bool, bool, bool) =
        (rng.random(), rng.random(), rng.random(), rng.random());
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
