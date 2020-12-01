use std::env;
use std::fs::File;
use std::io::{self, BufRead};

const YEAR: i32 = 2020;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("missing input file")
    }

    let filename = &args[1];

    // read vector of number
    let numbers = read_input(filename);
    let result = calc_result_three(numbers);
    if let Some(res) = result {
        print_part2(res);
    } else {
        println!("no result found");
    }
}

fn print_part1(res: (i32, i32, i32)) {
    println!("result: {} * {} = {}", res.0, res.1, res.2);
}

fn print_part2(res: (i32, i32, i32, i32)) {
    println!("result: {} * {} * {} = {}", res.0, res.1, res.2, res.3);
}

/**
 * Part 1: x + y = 2020 => x * y = ?
 */
fn calc_result_two(numbers: Vec<i32>) -> Option<(i32, i32, i32)> {
    let max_x = numbers.len() - 2;
    let max_y = max_x + 1;
    for x in 0..=max_x {
        let val_x = numbers[x];
        if val_x > YEAR {
            continue;
        }

        for y in (x + 1)..=max_y {
            let val_y = numbers[y];
            if val_y > YEAR {
                continue;
            }

            if val_x + val_y == YEAR {
                return Some((val_x, val_y, val_x * val_y));
            }
        }
    }

    return None;
}

/**
 * Part 3: x + y + z = 2020 => x * y * z = ?
 */
fn calc_result_three(numbers: Vec<i32>) -> Option<(i32, i32, i32, i32)> {
    let max_x = numbers.len() - 3;
    let max_y = max_x + 1;
    let max_z = max_y + 1;
    for x in 0..=max_x {
        let val_x = numbers[x];
        if val_x > YEAR {
            continue;
        }

        for y in (x + 1)..=max_y {
            let val_y = numbers[y];
            if val_y > YEAR {
                continue;
            }

            for z in (y + 1)..=max_z {
                let val_z = numbers[z];
                if val_z > YEAR {
                    continue;
                }
    
                if val_x + val_y + val_z == YEAR {
                    return Some((val_x, val_y, val_z, val_x * val_y * val_z));
                }
            }
        }
    }

    return None;
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
