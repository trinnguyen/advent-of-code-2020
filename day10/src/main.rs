use core::panic;
use std::{collections::{HashMap}, io::Read};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
    println!("result part-2: {:?}", part_2(&str));
}

fn part_1(str: &str) -> u32 {
    let (j1, j3) = cal_jolt_differences(str);
    println!("{}, {}", j1, j3);
    j1 * j3
}

fn part_2(str: &str) -> u64 {
    let mut vec: Vec<u64> = Vec::new();
    vec.push(0);
    str.lines().map(|l| l.trim().parse::<u64>().unwrap()).for_each(|i| vec.push(i));
    vec.sort();

    // println!("{:?}", vec);
    let mut map: HashMap<usize, u64> = HashMap::new();
    travel(&vec, 0, &mut map)
}

/// travel and count all paths
/// 
/// use map as cache
fn travel(vec: &Vec<u64>, index: usize, map: &mut HashMap<usize, u64>) -> u64 {

    // return if having cache
    if let Some(cache) = map.get(&index) {
        return *cache;
    }

    // finish
    if index == vec.len() - 1 {
        map.insert(index, 1);
        return 1
    }
    
    let successor = find_successor(vec, &index, vec.get(index).unwrap());
    let sum = (1..=successor).into_iter().map(|i| travel(vec, index + i, map)).sum();
    map.insert(index, sum);
    sum
}

fn find_successor(vec: &Vec<u64>, next: &usize, val: &u64) -> usize {
    let i = *next;
    match (vec.get(i + 1), vec.get(i + 2), vec.get(i + 3)) {
        (Some(x), Some(y), Some(z)) if x - val <= 3 && y - val <= 3 && z - val <= 3 => 3,
        (Some(x), Some(y), _) if x - val <= 3 && y - val <= 3 => 2,
        (Some(x), _, _) if x - val <= 3 => 1,
        _ => 0
    }
}


/// find the chain of 1-jolt and 3-jolt adapters
/// 
/// result for data set
///   sample1 -> 7,5
///   sample2 -> 22,10
///   input -> 64, 31
fn cal_jolt_differences(str: &str) -> (u32, u32) {
    
    let mut vec: Vec<i32> = str.lines().map(|l| l.trim().parse::<i32>().unwrap()).collect();
    vec.sort();

    let mut iter = vec.iter();
    let mut cur = 0; // charging outlet
    let mut count_1 = 0;
    let mut count_3 = 0;
    loop {
        match iter.next() {
            Some(next) => {
                match next - cur {
                    1 => count_1 = count_1 + 1,
                    2 => (),
                    3 => count_3 = count_3 + 1,
                    _ => panic!("unexpected error, differences is greater than 3")
                };

                cur = *next;
            },
            _ => break
        }
    }

    // the last one to connect to the device
    count_3 = count_3 + 1; // 

    (count_1, count_3)
}

#[cfg(test)]
mod tests {
    use crate::{cal_jolt_differences, part_2};

    #[test]
    fn test_part_1() {
        let src = "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";
        assert_eq!((7, 5), cal_jolt_differences(src));
    }

    #[test]
    fn test_part_2() {
        let src = "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";
        assert_eq!(8, part_2(src));
    }
}