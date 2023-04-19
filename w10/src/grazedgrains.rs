use std::io::{self, Read};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut ufos: Vec<(i8, i8, i8)> = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i8 = input.next().unwrap().parse().unwrap();
        let y: i8 = input.next().unwrap().parse().unwrap();
        let r: i8 = input.next().unwrap().parse().unwrap();
        ufos.push((x, y, r));
    }
    let mut hits = 0;

    let max_x: i64 = 100;
    let max_y: i64 = 100;
    let frac: f64 = 30.0 / max_x as f64;

    for px in 0..max_x {
        let px = (frac * px as f64) - 10.0;
        for py in 0..max_y {
            let py = (frac * py as f64) - 10.0;
            for (x, y, r) in &ufos {
                if (px - (*x as f64)).powi(2) + (py - (*y as f64)).powi(2) <= (r * r) as f64 {
                    hits += 1;
                    break;
                }
            }
        }
    }
    let points_in_frac = hits as f64 / (max_x * max_y) as f64;
    println!("{}", points_in_frac * (30.0 * 30.0));
}
