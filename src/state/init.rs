use bevy::ecs::prelude::*;
use chrono::{Datelike, Utc};
use rand::{Rng, RngExt, SeedableRng, distr::StandardUniform, prelude::SliceRandom};

extern crate alloc;
use alloc::vec::Vec;

use crate::state::{Card, Color, Fill, GameBoard, Quantity, Shape};

/// Initializes the game board.
pub fn initialize_game_board(mut commands: Commands) {
    // The game will change seeds every day in Eastern time.
    let time = Utc::now().with_timezone(&chrono_tz::US::Eastern);
    let date = format!("{}", time.format("%Y/%m/%d"));
    let year = time.year() as u64;
    let day_of_year = time.ordinal() as u64;
    // the last bit is shifted to the left since Pcg32 drops the last bit from the seed.
    let seed = bytemuck::cast::<[u64; 2], [u8; 16]>([year, day_of_year << 1]);

    let (cards, sets) = initialize_cards(seed);
    let board = GameBoard { cards, sets, date };
    commands.insert_resource(board);
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
        let mut length = cards.len();
        while 12 - length > 0 {
            let card: Card = rng.sample(StandardUniform);
            if !cards.contains(&card)
                && !almost_complete_sets
                    .iter()
                    .any(|(_, completing_card)| *completing_card == card)
            {
                for other in cards.iter() {
                    almost_complete_sets
                        .push(([card, *other], find_card_completing_set(card, *other)));
                }
                cards.push(card);
                length += 1;
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
