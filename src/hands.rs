use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use lazy_static::lazy_static;

use self::Hand::*;
use self::HandResult::*;

/// The result of a round of Scissors, Paper, Rock.
#[derive(Debug, Eq, PartialEq, Display)]
pub enum HandResult {
    /// The player won the round.
    Win,
    /// The player lost the round.
    Lose,
    /// The round was a draw.
    Draw,
}

/// A player's hand choice in a round of Scissors, Paper, Rock.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Display)]
pub enum Hand {
    /// Rock.
    Rock,
    /// Paper.
    Paper,
    /// Scissors.
    Scissors,
}

/// A lazy static vector of all the possible hands.
lazy_static! {
    pub static ref HANDS: Vec<Hand> = Hand::iter().collect();
    /// A lazy static vector of the names of all the possible hands.
    pub static ref HANDS_NAMES: Vec<String> = Hand::iter()
                                              .map(|hand| hand.to_string())
                                              .collect();
}

/// A trait for hands that defines the hand that beats them.
pub trait Beats {
    /// Returns the hand that beats `self`.
    fn beats(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        // match is exhaustive, so every enum variant must be covered
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

/// Determines the result of two hands being played against each other.
/// * `own_hand` - The hand being played.  
/// * `other_hand` - The opponent's hand.
/// # Returns the result of the hands being played, as a `HandResult`./// Plays a round of Scissors, Paper, Rock with the given hands and returns the result.
pub fn play_hand(own_hand: Hand, other_hand: Hand) -> HandResult {
    let (own_beats, other_beats) = (own_hand.beats(), other_hand.beats());

    match (own_beats, other_beats) {
        _ if own_beats == other_hand => Win,
        _ if other_beats == own_hand => Lose,
        _                            => Draw,
    }
}
/// Generates a random hand.
/// * `mut rng` - A mutable reference to a random number generator.
/// # Returns A randomly generated `Hand`.
pub fn random_hand(mut rng: &mut ThreadRng) -> Hand {
    *HANDS.choose(&mut rng).unwrap()
}


#[cfg(test)]
mod tests {
    use super::{HANDS, HANDS_NAMES, play_hand};
    use super::{HandResult::*, Hand::*};
    use std::collections::HashSet;

    #[test]
    fn test_unique_names() {
        assert_eq!(HANDS_NAMES.len(), HANDS.len());
        assert_eq!(HANDS_NAMES.iter().collect::<HashSet<_>>().len(),
                   HANDS.len());
    }

    #[test]
    fn test_play_hand() {
        assert_eq!(play_hand(Rock, Scissors), Win);
        assert_eq!(play_hand(Rock, Paper), Lose);
        assert_eq!(play_hand(Rock, Rock), Draw);

        assert_eq!(play_hand(Paper, Rock), Win);
        assert_eq!(play_hand(Paper, Scissors), Lose);
        assert_eq!(play_hand(Paper, Paper), Draw);

        assert_eq!(play_hand(Scissors, Paper), Win);
        assert_eq!(play_hand(Scissors, Rock), Lose);
        assert_eq!(play_hand(Scissors, Scissors), Draw);
    }
}
