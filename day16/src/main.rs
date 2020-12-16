use core::panic;
use std::{collections::HashMap, io::Read, ops::RangeInclusive};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
}

fn part_1(str: &str) -> u32 {
    let mut lines = str.lines();

    // parse classes
    let map_classes = parse_classes(&mut lines);

    // skip your ticket
    loop {
        match lines.next() {
            Some(t) if t.is_empty() => break,
            Some(t) => (),
            _ => panic!("unexpected EOF"),
        }
    }

    // process nearby ticket line by line
    match lines.next() {
        Some("nearby tickets:") => (),
        _ => panic!("expected 'nearby tickets:'"),
    }

    let mut sum = 0;
    loop {
        match lines.next() {
            Some(l) => {
                l.split(',')
                    .map(|i| i.parse::<u32>().unwrap())
                    .for_each(|n| {
                        // validate ticket
                        if !validate_num(&map_classes, n) {
                            sum = sum + n
                        }
                    });
            }
            _ => break, // EOF
        }
    }

    sum
}

fn validate_num(map_classes: &HashMap<u32, Vec<usize>>, n: u32) -> bool {
    map_classes.contains_key(&n)
}

/// parses lines into a map of number and class index
///
/// example:
///     class: 1-3 or 5-7
///     row: 6-11 or 33-44
///
/// result an array of
///     {1:0}, {2:0}, {3,0}, {4: []}, {5:0}, {6:0}
///     {6:[0,1]}, [7:1], ..., [12: []], ...
///
fn parse_classes(lines: &mut std::str::Lines) -> HashMap<u32, Vec<usize>> {
    let mut map: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut class_index: usize = 0;
    loop {
        match lines.next() {
            Some(l) => {
                if l.is_empty() {
                    break;
                }

                let mut items = l.split(": ");
                match (items.next(), items.next()) {
                    (Some(_), Some(v)) => {
                        let vec_range = parse_range(v);
                        for range in vec_range {
                            for num in range {
                                map.entry(num)
                                    .and_modify(|v| v.push(class_index))
                                    .or_insert(vec![class_index]);
                            }
                        }
                    }
                    _ => panic!("invalid value"),
                }
            }
            _ => (),
        }

        class_index = class_index + 1;
    }

    map
}

fn parse_range(str: &str) -> Vec<RangeInclusive<u32>> {
    let iter = str.split(" or ");

    iter.map(|i| {
        let mut range_iter = i.split('-');
        let fst = range_iter.next().unwrap().parse::<u32>().unwrap();
        let snd = range_iter.next().unwrap().parse::<u32>().unwrap();
        fst..=snd
    })
    .collect()
}
