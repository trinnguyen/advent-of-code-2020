use std::str::Lines;
use std::{io::Read};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();

    println!("result part-1: {:?}", part_1(&str));
    println!("result part-2: {:?}", part_2(&str));
}

fn part_1(str: &str) -> i32 {
    let mut vec: Vec<i32> = Vec::new();
    find_error(str, &mut vec)
}

fn part_2(str: &str) -> i32 {
    let mut vec: Vec<i32> = Vec::new();
    let needle = find_error(str, &mut vec);
    let (start, end) = find_contiguous_set(&vec, needle);
    calc_weakness(&vec[start..=end])
}

fn find_contiguous_set(vec: &Vec<i32>, needle: i32) -> (usize, usize) {
    // incremental find
    let mut start = 0;
    let mut end = 0;
    let mut sum = 0;
    for (i, v) in vec.iter().enumerate() {

        // value is over, reset the set
        if v >= &needle {
            start = i + 1;
            // println!("increase start: {}", start);
            continue;
        }

        end = i;
        // println!("update end: {}", end);
        sum = sum + v;
        loop {
            // normal exit if sum is smaller
            if sum < needle {
                break;
            }

            // found the set
            if sum == needle {
                return (start, end);
            }
            
            // increase start if greater than the value and try the loop
            if sum > needle {
                sum = sum - vec.get(start).unwrap();
                start = start + 1;
                // println!("increase start: {}", start);
            }
        }
    }

    (start, end)
}

fn calc_weakness(vec: &[i32]) -> i32 {
    // caculate result
    // println!("{:?}", vec);
    let mut min: i32 = *vec.get(0).unwrap();
    let mut max: i32 = min;
    let sub_set = &vec[1..vec.len()];
    for val in sub_set {
        if val < &min {
            min = *val;
        }
        if val > &max {
            max = *val;
        }
    }

    min + max
}

fn find_error(str: &str, vec: &mut Vec<i32>) -> i32 {
    // brute force
    let preamble: usize = 25;

    // first pass parse lines
    let mut lines = str.lines();
    loop {
        let num = if vec.is_empty() {preamble + 1} else {1};
        if !parse_vec(&mut lines, vec, num) {
            break;
        }

        // find
        let find = vec.get(vec.len() - 1).unwrap();
        let start: usize = vec.len() - 1 - preamble;
        let arr = &vec[start..start + preamble];
        if !is_valid(arr, *find) {
            return *find;
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