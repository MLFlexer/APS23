use std::io;

pub fn main() {
    let stdin = io::stdin();
    let mut first_line = String::new();
    stdin.read_line(&mut first_line).unwrap();
    let first_line_vec : Vec<&str> = first_line.split_whitespace().collect();
    //let n = first_line_vec[0].parse::<i32>().unwrap();
    let t = first_line_vec[1].parse::<i32>().unwrap();

    let mut task_str = String::new();
    io::stdin().read_line(&mut task_str).unwrap();

    let mut sum = 0;
    let mut i = 0;
    for s in task_str.split_whitespace() {
        sum += s.parse::<i32>().unwrap();
        if sum > t {
            break;
        } else {
            i += 1;
        }
    }
    println!("{}", i);
}