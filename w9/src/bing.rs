use std::io::{self, BufRead};

struct Node {
    childen: Vec<Option<Node>>,
    count: u32,
}
struct Trie {
    root: Node,
}

impl Node {
    fn insert(&mut self, char_iter: impl Iterator<Item = char>) {
        match char_iter.next() {
            None => (),
            Some(current_char) => {
                self.count += 1;
                let index = (current_char.to_digit(32).unwrap() % 32) as usize;
                match self.childen[index] {
                    Some(node) => {
                        node.insert(char_iter);
                    }
                    None => {
                        self.childen[index] = Some(Node {
                            childen: Vec::with_capacity(32),
                            count: 0,
                        });
                        self.childen[index].unwrap().insert(char_iter);
                    }
                }
            }
        }
    }
}

impl Trie {
    fn insert(&mut self, string: &str) {
        let mut char_iter = string.chars();
        self.root.insert(char_iter);
    }
    fn new(string: &str) -> Trie {
        let mut root = Node {
            childen: Vec::with_capacity(32),
            count: 0,
        };
        let mut trie = Trie { root };
        trie.insert(string);
        return trie;
    }
}

pub fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let words: Vec<&str> = line.split_whitespace().collect();
        for word in words {
            if word.len() >= prob_str.len() {
                preprocessing(word, a, b);
            }
        }
    }
}
