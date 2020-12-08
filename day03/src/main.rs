
use std::io::Lines;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(std::io::stdin());
    let lines = reader.lines();
    
    println!("result 1: {:?}", calc_1(lines));
}

fn calc_1(lines: Lines<BufReader<std::io::Stdin>>) -> u32 {
    let mut idx: usize = 0;
    let mut count: u32 = 0;
    let mut items = lines.peekable();
    while let Some(line) = items.next() {
        // check the current mark
        let str = line.unwrap();
        let bytes = str.as_bytes();
        let len = bytes.len();
        if idx > 0 {
            // out of range -> duplicate the line
            if idx > len - 1 {
                idx = idx % len
            }

            if bytes[idx] == b'#' {
                count += 1;
            }
        }

        // go to next round
        idx = idx + 3;
    }

    return count;
}
