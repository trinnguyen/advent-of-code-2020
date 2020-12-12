use core::panic;
use std::{io::Read};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
}

fn part_1(str: &str) -> u32 {
    let (j1, j3) = cal_jolt_differences(str);
    println!("{}, {}", j1, j3);
    j1 * j3
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
    use crate::cal_jolt_differences;

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
}