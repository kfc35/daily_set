use bevy::ecs::prelude::*;
use chrono::{Datelike, Utc};
use rand::{Rng, RngExt, SeedableRng, distr::StandardUniform, prelude::SliceRandom};

extern crate alloc;
use alloc::vec::Vec;

use crate::state::{Card, Color, Fill, Quantity, Shape};

/// Contains the game board - the cards for the game, the sets it contains, and the date.
#[derive(Resource)]
pub struct GameBoard {
    /// The cards the user tries to make Sets out of.
    pub cards: [Card; 12],
    /// The six sets that exist among the cards, i.e. the answer key.
    ///
    /// Each individual set is sorted.
    sets: [[Card; 3]; 6],
    /// The date of the game for this game board, formatted as "%Y/%m/%d" i.e. 2026/06/30.
    /// This uniquely identifies the game board.
    pub date: String,
}

impl GameBoard {
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

/// Initializes the game board.
pub fn init_game_board(mut commands: Commands) {
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

    // Randomly generate the first six cards without any care about the distribution
    // of the type of sets. The cards are randomly chosen so it should be fine.
    let first_card = rng.sample(StandardUniform);
    cards.push(first_card);

    let mut almost_complete_sets: Vec<([Card; 2], Card)> = vec![];
    for _ in 1..=5 {
        let mut new_card = first_card;
        while cards.contains(&new_card) {
            new_card = rng.sample(StandardUniform);
        }

        let new_sets = get_new_sets(&new_card, &sets, &almost_complete_sets);
        remove_completed_sets_with_new_card(&mut almost_complete_sets, &new_card);
        add_new_almost_complete_sets_with_new_card(
            &mut almost_complete_sets,
            &new_card,
            &cards,
            &new_sets,
        );

        // Update cards and sets with the new additions.
        cards.push(new_card);
        for new_set in new_sets.into_iter() {
            sets.push(new_set);
        }
    }

    while sets.len() < 6 {
        let must_create_set = 12 - cards.len() == 6 - sets.len();
        let (new_card, new_sets) = if must_create_set || rng.random() {
            // The number of cards left to add is equal to the number of sets we have to create.
            // Or, we randomly decided to add a card that completes a set.

            // The first new set we complete must conform to the distribution in `generate_new_set`.
            // Of 2/3's being 2-3 aspects different, 1/3 being 1 or 4 aspects different.
            // This is to ensure some degree of difficulty.
            let num_aspects_different_present: [bool; 4] = [false, false, false, false]
                .into_iter()
                .enumerate()
                .map(|(index, _)| {
                    almost_complete_sets.iter().any(|(first_two, third)| {
                        num_all_different_aspects(*first_two, *third) == index + 1
                    })
                })
                .collect::<Vec<bool>>()
                .try_into()
                .unwrap();
            let mut num_aspects_different = if rng.random_ratio(2, 3) {
                if rng.random() { 3 } else { 4 }
            } else if rng.random() {
                1
            } else {
                4
            };
            while !num_aspects_different_present[num_aspects_different - 1] {
                num_aspects_different = if rng.random_ratio(2, 3) {
                    if rng.random() { 3 } else { 4 }
                } else if rng.random() {
                    1
                } else {
                    4
                };
            }
            let sets_to_pick_from = almost_complete_sets
                .iter()
                .filter(|(first_two, third)| {
                    num_all_different_aspects(*first_two, *third) == num_aspects_different
                })
                .collect::<Vec<&([Card; 2], Card)>>();
            let sets_index = rng.random_range(0..sets_to_pick_from.len());

            let new_card = sets_to_pick_from[sets_index].1;
            let new_sets = get_new_sets(&new_card, &sets, &almost_complete_sets);

            if sets.len() + new_sets.len() > 6 || (must_create_set && new_sets.is_empty()) {
                // re-roll. This card either completes more sets than we need or
                // does not complete a set we need.
                continue;
            }

            remove_completed_sets_with_new_card(&mut almost_complete_sets, &new_card);
            (new_card, new_sets)
        } else {
            // Add a random card that will NOT complete a set.
            let mut card = cards[0];
            while cards.contains(&card)
                || almost_complete_sets
                    .iter()
                    .any(|(_, completing_card)| *completing_card == card)
            {
                card = rng.sample(StandardUniform);
            }
            (card, vec![])
        };

        add_new_almost_complete_sets_with_new_card(
            &mut almost_complete_sets,
            &new_card,
            &cards,
            &new_sets,
        );
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

/// Returns the new sets that would be completed if `new_card` were to be added to the game board's cards.
/// These new sets should not be duplicates of ones we've already completed.
fn get_new_sets(
    new_card: &Card,
    sets: &Vec<[Card; 3]>,
    almost_complete_sets: &Vec<([Card; 2], Card)>,
) -> Vec<[Card; 3]> {
    almost_complete_sets
        .iter()
        .filter(|(_, needed)| *needed == *new_card)
        .map(|(pair, _)| {
            let mut set = [pair[0], pair[1], *new_card];
            set.sort();
            set
        })
        // This check is redundant with how we wrote earlier code but it's better to be safe than sorry.
        .filter(|set| !sets.contains(set))
        .collect()
}

/// Removes any sets that would be completed `almost_complete_sets` with `new_card`.
fn remove_completed_sets_with_new_card(
    almost_complete_sets: &mut Vec<([Card; 2], Card)>,
    new_card: &Card,
) {
    // Gather ALL the new sets' indices that this new_card will complete.
    let indices_to_remove: Vec<usize> = almost_complete_sets
        .iter()
        .enumerate()
        .filter(|(_, (_, card))| *new_card == *card)
        .map(|(index, _)| index)
        .collect();
    // Remove the sets at those indices.
    for index in indices_to_remove.into_iter() {
        almost_complete_sets.swap_remove(index);
    }
}

/// Creates any potential sets that could be completed with `new_card` and adds them
/// to `almost_complete_sets`.
///
/// `new_sets` are the sets that this `new_card` just completed, so the cards in those sets should not be considered.
fn add_new_almost_complete_sets_with_new_card(
    almost_complete_sets: &mut Vec<([Card; 2], Card)>,
    new_card: &Card,
    cards: &Vec<Card>,
    new_sets: &Vec<[Card; 3]>,
) {
    let other_cards: Vec<Card> = cards
        .iter()
        .filter(|card| new_sets.iter().all(|set| !set.contains(card)))
        .copied()
        .collect();
    for other in other_cards {
        almost_complete_sets.push((
            [*new_card, other],
            find_card_completing_set(*new_card, other),
        ));
    }
}

#[expect(dead_code)]
/// Randomly generates a Set of cards.
// Currently unused but we keep it here because it may be useful in the future.
fn generate_set<R: Rng + ?Sized>(mut rng: &mut R) -> [Card; 3] {
    // The first card is randomly generated.
    let card: Card = rng.sample(StandardUniform);

    generate_set_with_card(&mut rng, card)
}

/// Randomly generates a Set of cards containing the provided `card`.
fn generate_set_with_card<R: Rng + ?Sized>(mut rng: &mut R, card: Card) -> [Card; 3] {
    // Decide how the next two cards in the same set should be chosen
    let two_or_three_aspects_different = rng.random_ratio(2, 3);
    let [same_shape, same_quantity, same_fill, same_color] = if two_or_three_aspects_different {
        let mut bools = [true, true, true, true];
        if rng.random() {
            // 3 aspects will be different.
            bools = [false, false, false, false];
            bools[rng.random_range(0..=3)] = true;
        } else {
            // 2 aspects will be different.
            let first_index = rng.random_range(0..=3);
            bools[first_index] = false;
            let mut second_index = first_index;
            while second_index == first_index {
                second_index = rng.random_range(0..=3);
            }
            bools[second_index] = false;
        }
        bools
    } else {
        // one or four aspects must be different
        let mut bools = [true, true, true, true];
        if rng.random() {
            // 1 aspect will be different.
            bools[rng.random_range(0..=3)] = false;
        } else {
            bools = [false, false, false, false];
        }
        bools
    };

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

/// Given a set of three cards, it tells you how many of the aspects are all different.
/// Typically, the more aspects that are all different, the more difficult the set may be
/// to spot.
fn num_all_different_aspects(first_two: [Card; 2], third: Card) -> usize {
    let mut count = 0;

    if first_two[0].shape != first_two[1].shape && first_two[1].shape != third.shape {
        count += 1;
    }
    if first_two[0].quantity != first_two[1].quantity && first_two[1].quantity != third.quantity {
        count += 1;
    }
    if first_two[0].fill != first_two[1].fill && first_two[1].fill != third.fill {
        count += 1;
    }
    if first_two[0].color != first_two[1].color && first_two[1].color != third.color {
        count += 1;
    }
    count
}
