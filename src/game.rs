use std::collections::HashMap;
use std::io;

use rand::Rng;

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

    fn get_bot_move(&self, moves: HashMap<i32, i32>) -> (i32, i32) {
        let mut keys_vector: Vec<i32> = vec![];
        for keys in moves.keys() {
            keys_vector.append(&mut vec![keys.clone()]);
        }
        let i = rand::thread_rng().gen_range(0, keys_vector.len());
        let key = keys_vector[i];

        match moves.get(&key) {
            None => {}
            Some(free_pos) => {
                return (key, *free_pos);
            }
        }
        (0, 0)
    }

    fn get_player_move(&self, moves: HashMap<i32, i32>) -> (i32, i32) {
        loop {
            println!("Which point you'd like to move?");
            let mut player_input = String::new();
            io::stdin().read_line(&mut player_input).unwrap();
            match player_input.trim_right().parse::<i32>() {
                Err(_) => println!("Try again"),
                Ok(pos) => {
                    match moves.get(&pos) {
                        None => println!("Move impossible"),
                        Some(free_pos) => return (pos as i32, *free_pos)
                    }
                }
            }
        }
    }

    fn check_game_over(&self) -> HashMap<i32, i32> {//Vec<Vec<i32>> {
        let check_value;
        //let mut availiable=vec![];
        let mut availiable = HashMap::new();

        match &self.current_turn {
            Players::Bot => {
                check_value = 2;
            }
            Players::Human => {
                check_value = 1;
            }
        }

        for (pos, n) in self.board.iter().enumerate() {
            if *n == check_value {
                let r = self.check_possible_move(pos as i32);
                match r {
                    None => {}
                    Some(i) => {
                        availiable.insert(pos as i32, i);
                    }
                }
            }
        }
        availiable
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
                        result = Some(i)
                    } else if (self.board[i as usize] == 0) & (i == 8) {
                        if (self.board[modulus(pos - 1, 8) as usize] == check_value) || (self.board[modulus(pos + 1, 8) as usize] == check_value)
                        {
                            result = Some(i)
                        }
                    }
                }
            }
        }
        result
    }

    fn swap(&mut self, pos: i32, free_pos: i32) {
        println!("{}-->{}", pos, free_pos);
        self.board[free_pos as usize] = self.board[pos as usize];
        self.board[pos as usize] = 0;
    }


    pub fn game_turn(&mut self) -> bool {
        match self.current_turn {
            Players::Human => {
                let res = self.check_game_over();

                if res.len() == 0 {
                    println!("you lost");
                    return true;
                } else {
                    println!("human----");
                    let (pos, free_pos) = self.get_player_move(res);
                    self.swap(pos as i32, free_pos);
                    self.current_turn = Players::Bot;
                    println!("human moved");
                    self.print_board();
                }
            }
            Players::Bot => {
                println!("bot---");
                let res = self.check_game_over();
                if res.len() == 0 {
                    println!("Bot lost");
                    return true;
                } else {
                    let (pos, free_pos) = self.get_bot_move(res);
                    self.swap(pos as i32, free_pos);
                    self.current_turn = Players::Human;
                    println!("bot moved");
                    self.print_board();
                }
            }
        }
        false
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

