use std::{
    collections::HashMap,
    io::{self, Read},
};

struct Node {
    childen: HashMap<char, Node>,
    count: u32,
}
struct Trie {
    root: Node,
}

impl Node {
    fn insert(&mut self, char_iter: &mut impl Iterator<Item = char>) -> u32 {
        match char_iter.next() {
            None => {
                let prev_count = self.count;
                self.count += 1;
                return prev_count;
            }
            Some(current_char) => {
                self.count += 1;
                match self.childen.get_mut(&current_char) {
                    Some(node) => node.insert(char_iter),
                    None => {
                        self.childen.insert(
                            current_char,
                            Node {
                                childen: HashMap::new(),
                                count: 0,
                            },
                        );

                        self.childen
                            .get_mut(&current_char)
                            .unwrap()
                            .insert(char_iter)
                    }
                }
            }
        }
    }
}

impl Trie {
    fn insert(&mut self, string: &str) -> u32 {
        let mut char_iter = string.chars();
        self.root.insert(&mut char_iter)
    }
    fn new(string: &str) -> Trie {
        let root = Node {
            childen: HashMap::new(),
            count: 0,
        };
        let mut trie = Trie { root };
        trie.insert(string);
        return trie;
    }
}
pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();

    let mut trie = Trie::new("");
    for _ in 0..n {
        let word = input.next().unwrap();
        println!("{}", trie.insert(word));
    }

    // let stdin = io::stdin();
    //
    // for line in stdin.lock().lines().map(|l| l.unwrap()) {
    //     let words: Vec<&str> = line.split_whitespace().collect();
    //     for word in words {
    //         println!("{}", trie.insert(word));
    //     }
    // }
}
