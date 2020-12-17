use std::{
    collections::{HashMap, HashSet},
    io::Read,
    ops::RangeInclusive,
    str::Lines,
};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
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
                        if !map_classes.contains_key(&n) {
                            sum = sum + n
                        }
                    });
            }
            _ => break, // EOF
        }
    }

    sum
}

fn part_2(str: &str) -> u64 {
    let mut lines = str.lines();

    // parse classes
    let map_classes = parse_classes(&mut lines);

    // skip your ticket
    ensure_your_ticket_header(&mut lines);
    let your_ticket: Vec<u32> = get_ticket(&mut lines).unwrap();
    let your_ticket_locs: Vec<HashSet<usize>> = get_ticket_pos(&your_ticket, &map_classes).unwrap();

    // model
    let mut model: Vec<ModelEntry> = Vec::new();
    for (i, v) in your_ticket_locs.iter().enumerate() {
        model.push(ModelEntry { pos: i, set: v.clone() });
    }

    // nearby tickets
    lines.next();
    ensure_near_by_header(&mut lines);
    while let Some(t) = get_ticket(&mut lines) {
        if let Some(locs) = get_ticket_pos(&t, &map_classes) {
            // find intersection
            for (i, v) in locs.iter().enumerate() {
                let model_entry = model.get_mut(i).unwrap();
                let inter: HashSet<usize> =
                    v.intersection(&model_entry.set).map(|ele| *ele).collect();
                model_entry.set = inter;
            }
        }
    }

    // find ideal model by eliminating
    eliminate(&mut model);

    // map model
    let mut total: u64 = 1;
    let field_indices = get_departure(str);
    for entry in model {
        let ticket_col_index = entry.pos;
        let f_index = entry.set.iter().next().unwrap();
        if field_indices.contains(f_index) {
            let value = your_ticket.get(ticket_col_index).unwrap();
            total = total * (*value as u64);
        }
    }

    total
}

/// elemiate the set
///
///   ModelEntry { set: {12} }
///   ModelEntry { set: {12, 15} }
///   ModelEntry { set: {12, 15, 10} }
///   ModelEntry { set: {15, 12, 17, 10} }
///   ModelEntry { set: {18, 15, 12, 10, 17} }
///   ModelEntry { set: {18, 12, 15, 17, 10, 11} }
///   ModelEntry { set: {2, 10, 18, 12, 11, 17, 15} }
///   ModelEntry { set: {17, 2, 18, 0, 15, 12, 11, 10} }
///   ModelEntry { set: {0, 12, 10, 18, 2, 5, 11, 15, 17} }
///   ModelEntry { set: {12, 17, 10, 15, 2, 11, 5, 0, 18, 1} }
///   ModelEntry { set: {1, 11, 10, 2, 18, 17, 15, 3, 0, 12, 5} }
///   ModelEntry { set: {2, 4, 11, 17, 5, 12, 3, 10, 15, 1, 18, 0} }
///   ModelEntry { set: {18, 7, 5, 17, 0, 1, 12, 3, 10, 2, 15, 4, 11} }
///   ModelEntry { set: {2, 5, 12, 1, 4, 11, 3, 15, 0, 7, 9, 10, 17, 18} }
///   ModelEntry { set: {15, 7, 1, 9, 0, 4, 10, 17, 11, 5, 16, 18, 3, 12, 2} }
///   ModelEntry { set: {10, 8, 3, 7, 1, 12, 11, 2, 15, 4, 9, 5, 16, 17, 18, 0} }
///   ModelEntry { set: {17, 18, 10, 4, 5, 1, 12, 7, 8, 0, 19, 2, 9, 3, 16, 11, 15} }
///   ModelEntry { set: {10, 2, 19, 11, 1, 4, 17, 5, 0, 9, 16, 8, 15, 18, 13, 12, 7, 3} }
///   ModelEntry { set: {17, 15, 4, 2, 7, 8, 19, 11, 16, 9, 3, 13, 5, 6, 12, 1, 18, 10, 0} }
///   ModelEntry { set: {17, 18, 5, 11, 2, 6, 15, 4, 8, 14, 0, 19, 7, 12, 9, 3, 13, 1, 16, 10} }
///
/// result single element in each set
fn eliminate(model: &mut Vec<ModelEntry>) {
    // get the first one
    loop {
        // sort first
        model.sort_by(|a, b| a.set.len().partial_cmp(&b.set.len()).unwrap());

        // there should be a single solution, the first one must has len 1
        if model.first().unwrap().set.len() != 1 {
            panic!("expected single solution for the set")
        }

        let mut reserved: Vec<usize> = Vec::new();
        let mut has_updated = false;
        model.iter_mut().for_each(|entry| {
            if entry.set.len() == 1 {
                reserved.push(*entry.set.iter().next().unwrap());
            } else {
                reserved.iter().for_each(|r| {
                    entry.set.remove(r);
                });
                has_updated = true;
            }
        });

        if !has_updated {
            break;
        }
    }
}

fn get_ticket(lines: &mut Lines) -> Option<Vec<u32>> {
    match lines.next() {
        Some(l) => {
            let t = l.split(',').map(|i| i.parse::<u32>().unwrap()).collect();
            Some(t)
        }
        None => None,
    }
}

fn get_ticket_pos<'a>(
    ticket: &Vec<u32>,
    map_classes: &HashMap<u32, Indices>,
) -> Option<Vec<HashSet<usize>>> {
    let mut result: Vec<HashSet<usize>> = Vec::new();

    for n in ticket {
        match map_classes.get(n) {
            Some(v) => result.push(v.to_set()),
            _ => return None,
        }
    }

    Some(result)
}

#[derive(Debug, PartialEq)]
struct ModelEntry {
    /// the column position on the ticket
    pos: usize,
    set: HashSet<usize>,
}

#[derive(Debug)]
struct Indices {
    items: Vec<usize>,
}

impl Indices {
    fn to_set(&self) -> HashSet<usize> {
        let mut set: HashSet<usize> = HashSet::new();
        self.items.iter().for_each(|i| {
            let _ = set.insert(*i);
        });
        set
    }
}

impl Clone for Indices {
    fn clone(&self) -> Self {
        let vec = self.items.iter().copied().collect();
        Indices { items: vec }
    }
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
                    }
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
        t => panic!("expected 'your ticket:' but '{:?}'", t),
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
fn parse_classes(lines: &mut std::str::Lines) -> HashMap<u32, Indices> {
    let mut map: HashMap<u32, Indices> = HashMap::new();
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
                                    .and_modify(|v| v.items.push(class_index))
                                    .or_insert(Indices {
                                        items: vec![class_index],
                                    });
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
        let mut vec: Vec<Vec<usize>> = Vec::new();

        let v0 = vec![0, 1];
        let v1 = vec![1, 0];
        let v2 = vec![2];
        vec.push(v0);
        vec.push(v1);
        vec.push(v2);

        assert_eq!(vec![vec![0, 1, 2], vec![1, 0, 2]], find_ticket_model(&vec));
    }
}
