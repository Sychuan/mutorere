#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_variables)]
use std::io;

use crate::game::Players::Human;

pub struct Game {
    current_turn: Players,
    board: Vec<i32>,

}

enum Players {
    Human,
    Bot,
}


impl Game {
    pub fn new() -> Game {
        Game {
            current_turn: Players::Human,
            board: vec![1, 1, 1, 1, 2, 2, 2, 2, 0],
        }

    }

    fn get_bot_move(&self) -> i32 {
        //let mut pos = 0;
        for i in &self.board {
            if *i == 2 {
                let r = self.check_possible_move(*i);
                match r {
                    None => {}
                    Some(pos) => return pos
                }
            }
        }
        0
    }





    fn get_player_move(&self) -> i32 {
        loop {
            println!("Which point you'd like to move?");
            let mut player_input = String::new();
            io::stdin().read_line(&mut player_input).unwrap();
            match player_input.trim_right().parse::<u32>() {
                Err(_) => println!("Try again"),
                Ok(i) => match i {
                    0..=8 if self.board[i as usize] == 1 => {
                        let r = self.check_possible_move(i as i32);
                        match r {
                            None => { println!("move impossible") }
                            Some(i) => return i
                        }
                    }
                    _ => println!("something else!")
                }
            }
        }
    }

    fn check_game_over(&self) -> Vec<i32> {
        let mut check_value = 0;
        let mut all_options = vec![];

        match &self.current_turn {
            Players::Bot => {
                check_value = 2;
                println!("Bot")
            }
            Players::Human => {
                check_value = 1;
                println!("Non-Bot")
            }
        }

        for pos in 0..self.board.len() {
            if self.board[i] == check_value {
                let r = self.check_possible_move(pos as i32);
                match r {
                    None => {},
                    Some(i) => {
                        all_options.append(&mut vec![i]);
                        println!("add")
                    }
                }
            }
        }
        all_options
    }

    fn check_possible_move(&self, pos: i32) -> Option<i32> {
        let check_value;
        let mut result: Option<i32> = None;
        match &self.current_turn {
            Players::Bot => {
                check_value = 1;
            }
            Players::Human => {
                check_value = 2;
            }
        }
        dbg!(pos);
        match pos {
            8 => {
                for i in 0..=7 {
                    if self.board[i as usize] == 0 {
                        result = Some(i);
                    }
                }
            }
            _ => {
                let choice = vec![modulus(pos - 1, 8), modulus(pos + 1, 8), 8];
                for i in choice {
                    if (self.board[i as usize] == 0) & (i != 8) {
                        println!("r{}", i);
                        result = Some(i)
                    } else if (self.board[i as usize] == 0) & (i == 8) {
                        if (self.board[modulus(pos - 1, 8) as usize] == check_value) || (self.board[modulus(pos + 1, 8) as usize] == check_value)
                        {
                            println!("r{}", i);
                            result = Some(i)
                        }
                    }
                }
            }
        }
        result
    }

    fn swap(&mut self, pos: i32, free_pos: i32) {
        self.board[free_pos as usize] = self.board[pos as usize];
        self.board[pos as usize] = 0;
    }


    pub fn game_turn(&mut self) {
        match self.current_turn {
            Players::Human => {
                let res = self.check_game_over();
                dbg!(res);

                println!("human----");
                let pos = self.get_player_move();
                self.swap(pos as i32, pos);
                self.current_turn = Players::Bot;
                println!("player moved");
                self.print_board();
            }
            Players::Bot => {
                println!("bot---");
                let pos = self.get_bot_move();
                self.swap(pos as i32, 0);
                self.current_turn = Players::Human;
                println!("bot moved");
                self.print_board();
            }
        }
    }

    pub fn print_board(&self) {
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
            print!("  ◇  ")
        } else if i == 1 { print!(" ⚪ ") } else if i == 2 { print!(" ⚫ ") }
    }
}

fn modulus(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

