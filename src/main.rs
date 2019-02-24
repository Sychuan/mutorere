use std::io;

mod game;

fn main() {
    println!("Start game!");
    let mut game = game::Game::new();

    println!("Scheme of board");
    game.board_scheme();
    game.print_board();

    loop {
        if game.game_turn() { break }
    }
    let mut player_input = String::new();
    io::stdin().read_line(&mut player_input).unwrap();
}
