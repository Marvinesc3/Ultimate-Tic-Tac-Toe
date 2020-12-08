#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ggez;
use ggez::event;
use ggez::GameResult;

mod constants;
mod game;
mod drawing;

use constants::{NAME, AUTHOR, WINDOW_SIZE};
use crate::game::{Board, Player};

fn main() -> GameResult {

    // Make a Context and an event loop
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new(NAME, AUTHOR)
        .window_setup(ggez::conf::WindowSetup::default().title(NAME))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .build()?;

    // create the game state with the human player going first with X
    let state = &mut Board::new(Player::X);

    // launch the game by start running the event loop
    // uses the context and event loop we created above and the game state we just created
    event::run(ctx, event_loop, state)
}
