use std::collections::{HashMap};
use std::io::Read;

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
    println!("result part-2: {:?}", part_2(&str));
}

fn part_1(str: &str) -> u64 {
    let mut mask: Vec<char> = Vec::new();
    let mut map_mem: HashMap<usize, u64> = HashMap::new();

    str.lines().for_each(|line| match parse_line(line) {
        Instr::Mask(t_mask) => mask = t_mask,
        Instr::Mem(addr, val) => {
            let new_val = mask_value_1(&mask, val);
            map_mem.insert(addr, new_val);
        }
    });

    map_mem.values().sum()
}

fn part_2(str: &str) -> u64 {

    let mut mask: Vec<char> = Vec::new();
    let mut map_mem: HashMap<usize, u64> = HashMap::new();

    str.lines().for_each(|line| match parse_line(line) {
        Instr::Mask(t_mask) => mask = t_mask,
        Instr::Mem(addr, val) => {
            let vec_addr = mask_values_2(&mask, addr);
            vec_addr.iter().for_each(|i| {
                map_mem.insert(*i, val);
            });
        }
    });

    map_mem.values().sum()
}

fn mask_value_1(mask: &Vec<char>, val: u64) -> u64 {
    // mask
    let bin: Vec<char> = dec_to_bin(val);
    let new_bin = bin.iter().enumerate().map(|(i,v)| {
        match mask.get(i) {
            Some('1') => '1',
            Some('0') => '0',
            _ => *v
        }
    }).collect();

    bin_to_dec(new_bin)
}

/// compute set of address
fn mask_values_2(mask: &Vec<char>, addr: usize) -> Vec<usize> {
    // mask
    let bin: Vec<char> = dec_to_bin(addr as u64);
    let mut x_locs: Vec<usize> = Vec::new();
    let masked_bin: Vec<char> = bin.iter().enumerate().map(|(i,v)| {
        match mask.get(i) {
            Some('1') => '1',
            Some('0') => *v,
            Some('X') => {
                x_locs.push(i);
                'X'
            },
            _ => panic!("invalid mask")
        }
    }).collect();

    let mut vec_result: Vec<usize> = Vec::new();
    let paths = combinations(&x_locs);
    for path in paths {
        let tmp_bin: Vec<char> = masked_bin.iter().enumerate().map(|(i,v)|
            if let Some(p_val) = path.get(&i) {
                *p_val
            } else {
                *v
            }
        ).collect();

        vec_result.push(bin_to_dec(tmp_bin) as usize);

    }

    vec_result
}

fn combinations(locs: &Vec<usize>) -> Vec<HashMap<usize, char>> {
    let mut paths: Vec<HashMap<usize, char>> = Vec::new();
    let mut stack: Vec<(usize, char)> = Vec::new();
    travel(&mut paths, &mut stack, &locs[0..locs.len()]);

    paths
}

fn travel(paths: &mut Vec<HashMap<usize, char>>, stack: &mut Vec<(usize, char)>, locs: &[usize]) {
    if locs.len() == 0 {
        // copy stack in reversed order
        let mut path: HashMap<usize, char> = HashMap::new();
        stack.iter().for_each(|(i, v)| {
            path.insert(*i, *v);
        });
        paths.push(path);
    } else {
        // 2 branches
        let i = locs[0];

        stack.push((i, '0'));
        travel(paths, stack, &locs[1..locs.len()]);

        stack.push((i, '1'));
        travel(paths, stack, &locs[1..locs.len()]);
    }

    // pop stack
    stack.pop();
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
    cal_bin_to_dec(bin.iter())
}

fn cal_bin_to_dec(iter: std::slice::Iter<char>) -> u64 {
    iter.rev().enumerate().map(|(i,v)| {
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
    use crate::mask_value_1;

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
            mask_value_1(
                &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                    .chars()
                    .collect::<Vec<char>>(),
                11
            )
        );

        assert_eq!(
            101,
            mask_value_1(
                &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                    .chars()
                    .collect::<Vec<char>>(),
                101
            )
        );

        assert_eq!(
            64,
            mask_value_1(
                &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                    .chars()
                    .collect::<Vec<char>>(),
                0
            )
        );
    }
}
