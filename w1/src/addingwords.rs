use std::{io::{self, Read}, collections::HashMap};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    
    //let n : u32 = input.next().unwrap().parse().unwrap();
    let mut var_env : HashMap<&str, i16>= HashMap::new();
    let mut val_env : HashMap<i16, &str>= HashMap::new();

    let mut word = input.next();
    while word != None {
        match word.unwrap() {
            "clear" => {
                var_env = HashMap::new();
                val_env = HashMap::new();
            },
            "def" => {
                let var = input.next().unwrap();
                let val : i16 = input.next().unwrap().parse().unwrap();
                var_env.insert(var, val);
                val_env.insert(val, var);
            },
            "calc" => {
                let var = input.next().unwrap();
                print!("{} ", var);
                let result = var_env.get(var).copied();
                
                match cal(&mut input, &var_env, result) {
                    None => println!("unknown"),
                    Some(r) => {
                        let x = val_env.get(&r);
                        match x {
                            None => println!("unknown"),
                            Some(s) => println!("{}", s),
                        }
                    },
                }
            },
            _ => {},
        }
        word = input.next();
    }
}


fn cal(mut input : &mut std::str::SplitAsciiWhitespace, var_env : &HashMap<&str, i16>, result : Option<i16>) -> Option<i16> {
    let op = input.next().unwrap();
    print!("{} ", op);
    match op {
        "=" => {
            return result;
        },
        _ => {
            let x = input.next().unwrap();
            print!("{} ", x);
            let v_x = var_env.get(x);
            match (op, v_x) {
                (_, None) => cal(input, var_env, None),
                ("+", Some(y)) => {
                    match result {
                        None => cal(input, var_env, None),
                        Some(r) => cal(input, var_env, Some(r + y)),
                    }
                },
                ("-", Some(y)) => {
                    match result {
                        None => cal(input, var_env, None),
                        Some(r) => cal(input, var_env, Some(r - y)),
                    }
                },
                (_, _) => None,
            }
            
        },
    }
}