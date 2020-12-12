use std::str::Lines;
use std::{io::Read};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();

    println!("result part-1: {:?}", part_1(&str));
}

fn part_1(str: &str) -> i32 {

    // brute force
    let preamble: usize = 25;
    let mut vec: Vec<i32> = Vec::new();

    // first pass parse lines
    let mut lines = str.lines();
    loop {
        let num = if vec.is_empty() {preamble + 1} else {1};
        if !parse_vec(&mut lines, &mut vec, num) {
            break;
        }

        // find
        let find = vec.get(vec.len() - 1).unwrap();
        let start: usize = vec.len() - 1 - preamble;
        let end: usize = vec.len() - 1;
        let arr = &vec[start..end];
        if !is_valid(arr, *find) {
            return *find
        }
    }

    panic!("no new line. not found");
}

fn parse_vec(lines: &mut Lines, vec: &mut Vec<i32>, num: usize) -> bool {
    let mut count = 0;
    loop {
        match lines.next() {
            Some(line) => {
                let val: i32 = line.parse().unwrap();
                vec.push(val);
                count = count + 1;
                if count == num {
                    break
                }
            },
            _ => return false
        }
    }

    return true
}

fn is_valid(arr: &[i32], find: i32) -> bool {
    println!("{:?} - {}", arr, find);
    let len = arr.len();
    for j in 0..len - 1 {
        let val_j = arr.get(j).unwrap();
        for t in j + 1..len {
            let val_t = arr.get(t).unwrap();
            if val_j + val_t == find {
                return true;
            }
        }
    }

    false
}