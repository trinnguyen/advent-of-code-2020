use std::{io::Read};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
}

fn part_1(str: &str) -> i32 {
    
}