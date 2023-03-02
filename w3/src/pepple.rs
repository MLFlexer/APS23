use std::{
    collections::HashMap,
    io::{self, Read},
};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: i32 = input.next().unwrap().parse().unwrap();
    let mut mem: HashMap<[u8; 12], u8> = HashMap::new();
    for _ in 0..n {
        let mut board: [u8; 12] = [0; 12];
        let str_board = input.next().unwrap();
        let mut i = 0;
        let mut num_p: u8 = 0;
        for c in str_board.chars() {
            match c {
                '-' => board[i] = 0,
                'o' => {
                    board[i] = 1;
                    num_p += 1;
                }
                _ => {}
            }
            i += 1;
        }

        println!("{}", search(board, &mut mem, num_p));
    }
}

fn search(board: [u8; 12], mem: &mut HashMap<[u8; 12], u8>, num_p: u8) -> u8 {
    match mem.get(&board) {
        Some(num) => {
            return *num;
        }
        None => {
            let mut rev_board = board;
            rev_board.reverse();

            match mem.get(&board) {
                Some(num) => {
                    return *num;
                }
                None => {
                    let mut score = num_p;
                    for i in 1..11 {
                        match (board[i - 1], board[i], board[i + 1]) {
                            (0, 1, 1) => {
                                let mut new_board = board;
                                new_board[i - 1] = 1;
                                new_board[i] = 0;
                                new_board[i + 1] = 0;

                                let new_score = search(new_board, mem, num_p - 1);

                                if new_score < score {
                                    score = new_score;
                                }
                            }
                            (1, 1, 0) => {
                                let mut new_board = board;
                                new_board[i - 1] = 0;
                                new_board[i] = 0;
                                new_board[i + 1] = 1;

                                let new_score = search(new_board, mem, num_p - 1);

                                if new_score < score {
                                    score = new_score;
                                }
                            }
                            _ => {}
                        }
                    }
                    mem.insert(board, score);
                    return score;
                }
            }
        }
    }
}
