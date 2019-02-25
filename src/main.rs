use std::io;

mod game;

fn main() {
    println!("Start game!");
    let mut game = game::Game::new();

    println!("Scheme of board");
    game.board_scheme();
    game.print_board();
    let x = game.first_move();
    game.current_turn = x.0;
    game.value = x.1;

    loop {
        if game.game_turn() { break; }
    }
    let mut player_input = String::new();
    io::stdin().read_line(&mut player_input).unwrap();
}
