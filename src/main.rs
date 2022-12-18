mod hands;
mod game;

use game::{Game};

/// Plays the game of Scissors, Paper, Rock in the command line.
fn play_commandline() {
    let mut game = Game::new();

    game.game_loop();
}

/// The main entry point for the game.
fn main() {
    play_commandline();
}
