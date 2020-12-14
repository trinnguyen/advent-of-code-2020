use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
}

fn part_1(str: &str) -> u64 {
    let mut mask: Vec<char> = Vec::new();
    let mut map_mem: HashMap<usize, u64> = HashMap::new();

    str.lines().for_each(|line| match parse_line(line) {
        Instr::Mask(t_mask) => mask = t_mask,
        Instr::Mem(addr, val) => {
            let new_val = mask_value(&mask, val);
            map_mem.insert(addr, new_val);
        }
    });

    map_mem.values().sum()
}

fn mask_value(mask: &Vec<char>, val: u64) -> u64 {
    let bin: Vec<char> = dec_to_bin(val);

    // mask
    let new_bin = bin.iter().enumerate().map(|(i,v)| {
        match mask.get(i) {
            Some('1') => '1',
            Some('0') => '0',
            _ => *v
        }
    }).collect();

    bin_to_dec(new_bin)
}

fn dec_to_bin(val: u64) -> Vec<char> {
    let mut tmp = val;
    let mut vec: Vec<char> = Vec::new();
    while tmp != 0 {
        match tmp % 2 {
            1 => vec.push('1'),
            0 => vec.push('0'),
            _ => ()
        }

        tmp = tmp / 2;
    }

    // append if missing
    let mask_size = 36;
    if vec.len() < mask_size {
        for _ in vec.len()..mask_size {
            vec.push('0');
        }
    }

    vec.reverse();

    vec
}

fn bin_to_dec(bin: Vec<char>) -> u64 {
    bin.iter().rev().enumerate().map(|(i,v)| {
        if *v == '1' {
            2u64.pow(i as u32)
        } else {
            0
        }
    }).sum()
}

fn parse_line(line: &str) -> Instr {
    let mut sp = line.split(" = ");
    match (sp.next(), sp.next()) {
        (Some("mask"), Some(s_mask)) => {
            let m: Vec<char> = s_mask.chars().collect();
            Instr::Mask(m)
        }
        (Some(str1), Some(s_val)) => {
            let (add, val) = parse_mem(str1, s_val);
            Instr::Mem(add, val)
        }
        _ => panic!("invalid input"),
    }
}

fn parse_mem(str1: &str, val: &str) -> (usize, u64) {
    let mut sp = str1.split('[');
    match (sp.next(), sp.next()) {
        (Some("mem"), Some(indexer)) => {
            let bytes = indexer.as_bytes();
            match bytes[bytes.len() - 1] {
                b']' => {
                    let s_addr: usize = (&indexer[0..=bytes.len() - 2]).parse().unwrap();
                    return (s_addr, val.parse::<u64>().unwrap());
                }
                _ => panic!("invalid input"),
            }
        }
        _ => panic!("invalid input"),
    }
}

#[derive(Debug)]
enum Instr {
    Mask(Vec<char>),
    Mem(usize, u64),
}

#[cfg(test)]
mod tests {
    use crate::{bin_to_dec, dec_to_bin};
    use crate::mask_value;

    #[test]
    fn test_dec_to_bin() {
        assert_eq!(
            "000000000000000000000000000000001011"
                .chars()
                .collect::<Vec<char>>(),
            dec_to_bin(11)
        );

        assert_eq!(
            "000000000000000000000000000001001001"
                .chars()
                .collect::<Vec<char>>(),
            dec_to_bin(73)
        );
    }

    #[test]
    fn test_bin_to_dec() {
        assert_eq!(
            11,
            bin_to_dec("000000000000000000000000000000001011"
                .chars()
                .collect::<Vec<char>>())
        );

        assert_eq!(
            73,
            bin_to_dec("000000000000000000000000000001001001"
                .chars()
                .collect::<Vec<char>>())
        );
    }

    #[test]
    fn test_mask_value() {
        assert_eq!(
            73,
            mask_value(
                &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                    .chars()
                    .collect::<Vec<char>>(),
                11
            )
        );

        assert_eq!(
            101,
            mask_value(
                &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                    .chars()
                    .collect::<Vec<char>>(),
                101
            )
        );

        assert_eq!(
            64,
            mask_value(
                &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                    .chars()
                    .collect::<Vec<char>>(),
                0
            )
        );
    }
}
