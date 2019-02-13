use std::io;

#[allow(dead_code)]
#[allow(unused_variables)]
mod game;

fn main() {
    println!("Start game!");
    let mut game = game::Game::new();
    game.game_turn();
    game.game_turn();
    game.game_turn();
    game.game_turn();
}
