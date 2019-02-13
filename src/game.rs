#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_variables)]
use std::io;

pub struct Game {
    current_turn: Turn,
    board: Vec<i32>,

}

enum Turn {
    Player,
    Bot,
}


impl Game {
    pub fn new() -> Game {
        Game {
            current_turn: Turn::Player,
            board: vec![1, 1, 1, 1, 2, 2, 2, 2, 0],
        }
    }

    fn move_validate(&self, pos: u32) -> u32 {
        5
    }
    fn get_player_move(&self) -> u32 {
        loop {
            println!("Which point you'd like to move?");
            let mut player_input = String::new();
            io::stdin().read_line(&mut player_input).unwrap();
            match player_input.trim_right().parse::<u32>() {
                Err(_) => println!("Try again"),
                Ok(i) => match i {
                    0...8 if self.board[i as usize] == 1 => return i,
                    _ => println!("something else!")
                }
            }
        }
    }

    fn swap(&mut self, pos: i32) {
        if pos == 8 {
            for i in 0..=7 {
                if self.board[i as usize] == 0 {
                    //println!("free{}", i);
                    self.board[i as usize] = self.board[8];
                    self.board[8 as usize] = 0;
                }
            }
            //println!("{}", pos);
        } else {
            let choice = vec![modulus(pos - 1, 8), modulus(pos + 1, 8), 8];
            for i in choice {
                if self.board[i as usize] == 0 {
                    //println!("free{}", i);
                    self.board[i as usize] = self.board[pos as usize];
                    self.board[pos as usize] = 0;
                }
            }
            //println!("{} n {} {}", pos, modulus(pos - 1, 8), modulus(pos + 1, 8))
        }
    }

    fn get_bot_move(&self) -> u32 {
        let mut pos = 0;
        for i in &self.board {
            pos += 1;
            if *i == 2 {
                let choice = vec![modulus(pos - 1, 8), modulus(pos + 1, 8), 8];
                println!("{} n {} {}", pos, modulus(pos - 1, 8), modulus(pos + 1, 8));
                for n in &choice {
                    match self.board[*n as usize] {
                        0 => {
                            println!("free {} pos {}", self.board[*n as usize], n);
                            return pos as u32;
                        }
                        1 => println!("not {}", self.board[choice[0] as usize]),
                        2 => println!("not {}", self.board[choice[0] as usize]),
                        _ => println!("not {}", self.board[choice[0] as usize])
                    }
                }
            }
        }
        1
    }

    pub fn game_turn(&mut self) {
        match self.current_turn {
            Turn::Player => {
                let pos = self.get_player_move();
                self.swap(pos as i32);
                self.current_turn = Turn::Bot;
                println!("player move");
                self.print_board();
            }
            Turn::Bot => {
                let pos = self.get_bot_move();
                self.swap(pos as i32);
                self.current_turn = Turn::Player;
                println!("bot move");
                self.print_board();
            }
        }
    }

    fn print_board(&self) {
        self.visualize_board(self.board[0]);
        print!("--");
        self.visualize_board(self.board[1]);
        print!("--");
        self.visualize_board(self.board[2]);
        println!();
        println!(" |     |  / |");
        self.visualize_board(self.board[7]);
        print!("--");
        self.visualize_board(self.board[8]);
        print!("--");
        self.visualize_board(self.board[3]);
        println!();
        println!(" |  /  |    |");
        self.visualize_board(self.board[6]);
        print!("--");
        self.visualize_board(self.board[5]);
        print!("--");
        self.visualize_board(self.board[4]);
        println!();
        println!("{}", "-".repeat(10));
    }

    fn visualize_board(&self, i: i32) {
        if i == 0 {
            print!(" * ")
        } else if i == 1 { print!(" ⚪ ") } else if i == 2 { print!(" ⚫ ") }
    }
}

fn modulus(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

