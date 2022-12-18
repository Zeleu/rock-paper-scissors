use crate::hands::{Hand, HandResult, HANDS, HANDS_NAMES, play_hand, random_hand};

use rand::prelude::*;
use lazy_static::lazy_static;

use std::collections::HashMap;
use std::io;
use std::io::Write;

/// A struct representing a player's hand choice, along with its label and number.
struct HandInput {
    hand: Hand,
    label: String,
    number: usize,
}

/// A lazy static vector of all the possible hand inputs.
lazy_static! {
    static ref HAND_INPUTS: Vec<HandInput> = {
        HANDS.iter().enumerate().map(
            |(num, &hand)| {
                let number = num + 1;
                let label = format!("[{}] {}", number, hand);

                HandInput { hand, label, number }
            }
        ).collect()
    };

/// A lazy static map of numbers to hand inputs.
    static ref INPUT_MAP: HashMap<usize, &'static HandInput> = {
        let mut input_map = HashMap::new();

        for hand_input in HAND_INPUTS.iter() {
            input_map.insert(hand_input.number, hand_input);
        }

        input_map
    };

/// A lazy static string containing the prompt for hand inputs.
    static ref HAND_INPUT_PROMPT: String = {
        HAND_INPUTS
        .iter()
        .map(|hand_input| hand_input.label.clone())
        .collect::<Vec<_>>()
        .join(" / ")
    };
}

/// A struct representing a game of Scissors, Paper, Rock.
pub struct Game {
    /// A random number generator for generating the computer's hand.
    rng: ThreadRng,

    /// The current score of the game.
    score: isize,
}

impl Game {
    /// Creates a new game with a zero score.
    pub fn new() -> Game {
        Game {
            rng: thread_rng(),
            score: 0,
        }
    }

    /// Runs the game loop, allowing the player to choose a hand and playing a round against the computer. The game ends when the player quits or an error occurs.
    pub fn game_loop(&mut self) {
        println!("=== {} Game ===\n", HANDS_NAMES.join(" "));
        println!("Any non-move input quits.\n");

        while let Some(player_hand) = self.choose_hand() {
            let (cpu_hand, result) = self.play_hand(player_hand);

            println!("You play : {}", player_hand);
            println!("CPU plays: {}\n", cpu_hand);
            println!("Result: you {}!", result);
            println!("Current score: {}\n", self.score);
        }

        println!("Game over.");
    }
/// Prompts the player to choose a hand and returns it, or `None` if an error occurs or the player quits.
    fn choose_hand(&self) -> Option<Hand> {
        print!("Your move ({})? >> ", *HAND_INPUT_PROMPT);

        let _ = io::stdout().flush();
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            None
        } else {
            self.parse_input(input)
        }
    }

    fn parse_input(&self, input: String) -> Option<Hand> {
       /// Parses the given input string and returns the corresponding hand, or `None` if the input is invalid.
        let number: usize = input.trim().parse().ok()?;

        INPUT_MAP.get(&number).map(|hand_input| hand_input.hand)
    }

    fn play_hand(&mut self, hand: Hand) -> (Hand, HandResult) {
         /// Plays a round of Scissors, Paper, Rock with the given hand and returns the computer's hand and the result of the round.
        let cpu_hand = random_hand(&mut self.rng);
        let result = play_hand(hand, cpu_hand);
        let score_delta = match result {
            HandResult::Win => 1,
            HandResult::Lose => -1,
            _ => 0,
        };

        self.score += score_delta;

        (cpu_hand, result)
    }
}
