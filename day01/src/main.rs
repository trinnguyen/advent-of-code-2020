use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("missing input file")
    }

    let filename = &args[1];

    // read vector of number
    let numbers = read_input(filename);
    let max_x = numbers.len() - 2;
    let max_y = max_x + 1;
    for x in 0..=max_x {
        for y in (x + 1)..=max_y {
            let val_x = numbers[x];
            let val_y = numbers[y];
            if val_x + val_y == 2020 {
                let result = val_x * val_y;
                println!("result: {} * {} = {}", val_x, val_y, result);
            }
        }
    }
}

fn read_input(filename: &str) -> Vec<i32> {
    let f = File::open(filename).expect("Cannot read the input file");
    let lines = io::BufReader::new(f).lines();

    let mut result = Vec::new();
    for line in lines {
        if let Ok(l) = line {
            let num: i32 = l.parse().unwrap();
            result.push(num);
        }
    }

    return result;
}
