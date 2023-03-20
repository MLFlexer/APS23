use std::io::{self, Read};

#[derive(Debug)]
struct V {
    x: i32,
    y: i32,
}

impl V {
    fn from_points(p1: (i32, i32), p2: (i32, i32)) -> V {
        return V {
            x: (p1.0 - p2.0),
            y: (p1.1 - p2.1),
        };
    }
    fn cross_prod(&self, v: &V) -> i32 {
        return ((self.x * v.y) - (v.x * self.y));
    }
    fn turns_left(&self, v: &V) -> bool {
        return self.cross_prod(v) > 0;
    }
    fn turns_right(&self, v: &V) -> bool {
        return self.cross_prod(v) < 0;
    }
}

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    loop {
        let n: usize = input.next().unwrap().parse().unwrap();
        if n == 0 {
            break;
        } else if n == 1 {
            let _x: i32 = input.next().unwrap().parse().unwrap();
            let _y: i32 = input.next().unwrap().parse().unwrap();
            println!("0");
            continue;
        }

        let mut points: Vec<(i32, i32)> = Vec::with_capacity(n);
        for _ in 0..n {
            let x: i32 = input.next().unwrap().parse().unwrap();
            let y: i32 = input.next().unwrap().parse().unwrap();
            points.push((x, y));
        }

        points.sort();

        let mut upper_hull: Vec<(i32, i32)> = Vec::with_capacity(n);
        upper_hull.push(points[0]);
        upper_hull.push(points[1]);

        let mut lower_hull: Vec<(i32, i32)> = Vec::with_capacity(n);
        lower_hull.push(points[points.len() - 1]);
        lower_hull.push(points[points.len() - 2]);

        for i in 2..points.len() {
            //Upper hull
            upper_hull.push(points[i]);
            let mut p1 = upper_hull[upper_hull.len() - 3];
            let mut p2 = upper_hull[upper_hull.len() - 2];
            let p3 = upper_hull[upper_hull.len() - 1];
            let mut v1: V = V::from_points(p1, p2);
            let mut v2: V = V::from_points(p2, p3);
            while v1.turns_left(&v2) && upper_hull.len() >= 3 {
                upper_hull.remove(upper_hull.len() - 2);
                if upper_hull.len() == 2 {
                    break;
                }
                p1 = upper_hull[upper_hull.len() - 3];
                p2 = upper_hull[upper_hull.len() - 2];
                v1 = V::from_points(p1, p2);
                v2 = V::from_points(p2, p3);
            }

            //Lower hull
            lower_hull.push(points[points.len() - i - 1]);
            let mut p1 = lower_hull[lower_hull.len() - 3];
            let mut p2 = lower_hull[lower_hull.len() - 2];
            let p3 = lower_hull[lower_hull.len() - 1];
            let mut v1: V = V::from_points(p1, p2);
            let mut v2: V = V::from_points(p2, p3);
            while v1.turns_left(&v2) && lower_hull.len() >= 3 {
                lower_hull.remove(lower_hull.len() - 2);
                if lower_hull.len() == 2 {
                    break;
                }
                p1 = lower_hull[lower_hull.len() - 3];
                p2 = lower_hull[lower_hull.len() - 2];
                v1 = V::from_points(p1, p2);
                v2 = V::from_points(p2, p3);
            }
        }

        let mut convex_points: Vec<(i32, i32)> = upper_hull[..upper_hull.len() - 1].to_vec();
        convex_points.append(&mut lower_hull);

        let mut area: i32 = 0;
        for i in 1..convex_points.len() {
            let p1 = convex_points[i - 1];
            let p2 = convex_points[i];
            area += ((p1.0 * p2.1) - (p1.1 * p2.0));
        }
        println!("{}", area.abs() as f32 / 2.0);
    }
}
