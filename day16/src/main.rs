use std::{
    collections::{HashMap, HashSet},
    io::Read,
    ops::RangeInclusive,
};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    // println!("result part-1: {:?}", part_1(&str));
    println!("result part-2: {:?}", part_2(&str));
}

fn part_1(str: &str) -> u32 {
    let mut lines = str.lines();

    // parse classes
    let map_classes = parse_classes(&mut lines);

    // skip your ticket
    skip_your_ticket(&mut lines);
    ensure_near_by_header(&mut lines);

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

fn part_2(str: &str) -> u32 {
    let mut lines = str.lines();

    // parse classes
    let map_classes = parse_classes(&mut lines);

    // skip your ticket
    ensure_your_ticket_header(&mut lines);

    // vector of posibilities for each index
    let your_ticket: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|i| i.parse::<u32>().unwrap())
        .collect();

    println!("your_ticket: {:?}", your_ticket);

    let vec_locs: Vec<&Vec<usize>> = your_ticket
        .iter()
        .map(|n| match map_classes.get(&n) {
            Some(v) => v,
            _ => panic!("your ticket is invalid"),
        })
        .collect();

    println!("vec_locs: {}", vec_locs.len());
    vec_locs.iter().for_each(|i| println!("{:?}", i));

    // find models
    let mut models: Vec<Vec<usize>> = find_ticket_model(&vec_locs);
    println!("models: {:?}", models);

    // check if ticket match model
    let model_match_provider = |model: &Vec<usize>, ticket: &Vec<u32>| {
        ticket
            .iter()
            .enumerate()
            .all(|(i, t)| match map_classes.get(t) {
                Some(v) if v.contains(&model.get(i).unwrap()) => true,
                _ => false,
            })
    };

    // check if ticket is valid
    let valid_ticket_provider = |ticket: &Vec<u32>| {
        ticket.iter().all(|t| {
            if let Some(_) = map_classes.get(t) {
                true
            } else {
                false
            }
        })
    };

    // go through nearby
    lines.next(); // empty line
    ensure_near_by_header(&mut lines);

    // line by line
    loop {
        match lines.next() {
            Some(l) => {
                let ticket: Vec<u32> = l.split(',').map(|i| i.parse::<u32>().unwrap()).collect();
                if valid_ticket_provider(&ticket) {
                    println!("- ticket: {:?}", ticket);
                    let invalid: Vec<usize> = models
                        .iter()
                        .enumerate()
                        .filter(|(_, model)| !model_match_provider(*model, &ticket))
                        .map(|(i, _)| i)
                        .collect();

                    invalid.iter().rev().for_each(|i| {
                        let _ = models.swap_remove(*i);
                    });

                    println!("{:?}", models);
                } else {
                    println!("- invalid ticket: {:?}", ticket);
                }
            }
            _ => break, // EOF
        }
    }

    if models.is_empty() {
        panic!("no modal found")
    }

    // index of six departures
    let field_indices: HashSet<usize> = get_departure(str);
    println!("departure: {:?}", field_indices);

    // result
    let model = models.first().unwrap();
    let r = your_ticket.iter().enumerate().filter(|(i, v)| field_indices.contains(&i)).map(|(i,_)| model.get(i).unwrap()).fold(1_u32, |acc, v| acc * (*v as u32));
    r
}

fn get_departure(str: &str) -> HashSet<usize> {
    let mut lines = str.lines();
    let mut vec: HashSet<usize> = HashSet::new();
    let mut i: usize = 0;
    loop {
        match lines.next() {
            Some(l) if l.is_empty() => break,
            Some(l) => {
                let mut iter = l.split(' ');
                match iter.next() {
                    Some("departure") => {
                        let _ = vec.insert(i);
                    },
                    _ => break,
                }
            }
            _ => panic!("unexpected EOF"),
        }
        i = i + 1;
    }

    vec
}

fn ensure_your_ticket_header(lines: &mut std::str::Lines) {
    match lines.next() {
        Some("your ticket:") => (),
        _ => panic!("expected 'your ticket:'"),
    }
}

/// generate vector of completed model of the ticket
///
/// example input:
/// [[0,1], [0,1], [2]]
///
/// output:
///  [[0, 1, 2], [1, 0, 2]]
///
fn find_ticket_model(vec_locs: &Vec<&Vec<usize>>) -> Vec<Vec<usize>> {
    // use a stack for current travel, and a cached set
    let mut result: Vec<Vec<usize>> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut cached: HashSet<usize> = HashSet::new();
    travel(&vec_locs, 0, &mut stack, &mut cached, &mut result);

    result
}

fn travel(
    vec_locs: &Vec<&Vec<usize>>,
    index: usize,
    stack: &mut Vec<usize>,
    cached: &mut HashSet<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    match vec_locs.get(index) {
        Some(v) => {
            // go through value of v
            for elem in *v {
                if !cached.contains(elem) {
                    stack.push(*elem);
                    cached.insert(*elem);
                    travel(vec_locs, index + 1, stack, cached, result);

                    // pop
                    let item = stack.pop().unwrap();
                    cached.remove(&item);
                }
            }
        }
        _ => {
            // end -> copy the stack
            let id = stack.iter().copied().collect();
            println!("id: {:?}", id);
            result.push(id);
        }
    }
}

fn skip_your_ticket(lines: &mut std::str::Lines) -> () {
    loop {
        match lines.next() {
            Some(t) if t.is_empty() => break,
            Some(_) => (),
            _ => panic!("unexpected EOF"),
        }
    }
}

fn ensure_near_by_header(lines: &mut std::str::Lines) {
    match lines.next() {
        Some("nearby tickets:") => (),
        _ => panic!("expected 'nearby tickets:'"),
    }
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

#[cfg(test)]
mod tests {
    use crate::find_ticket_model;

    #[test]
    fn test_find_ticket_model() {
        let mut vec: Vec<&Vec<usize>> = Vec::new();

        let v0 = vec![0, 1];
        let v1 = vec![1, 0];
        let v2 = vec![2];
        vec.push(&v0);
        vec.push(&v1);
        vec.push(&v2);

        assert_eq!(vec![vec![0, 1, 2], vec![1, 0, 2]], find_ticket_model(&vec));
    }
}
