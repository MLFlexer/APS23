use std::{io::{self, Read}, collections::HashMap};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    
    let n : i32 = input.next().unwrap().parse().unwrap();
    for _ in 0..n {
        let board : [u8; 12] = [0; 12];
        let str_board = input.next().unwrap();
        let i = 0;
        for c in str_board.chars() {
            match c {
                '-' => {board[i] = 0},
                'o' => {board[i] = 1},
            }
            i += 1;
        }

        let mut mem : HashMap<[u8; 12], i64> = HashMap::new();


    }
}

fn search(board : [u8; 12], mem : HashMap<[u8; 12], i64>) -> i64 {
    for i in 1..11 {
        match (board[i - 1], board[i], board[i + 1]) {
            (0,1,1) => {},
            _ => {},
        }
    }
    return 0;
}
