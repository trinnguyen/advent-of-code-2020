use core::panic;
use std::io::Read;

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
    println!("result part-2: {:?}", part_2(&str));
}

fn part_1(str: &str) -> u32 {
    let mut lines = str.lines();

    match (lines.next(), lines.next()) {
        (Some(l1), Some(l2)) => {
            let ts: u32 = l1.parse().unwrap();
            let (ts_id, id) = find_bus(ts, l2);
            id * (ts_id - ts)
        }
        _ => panic!("invalid input"),
    }
}

/// hopeless: generate equation for solving in wolframalpha.com
/// use as input on https://www.wolframalpha.com
fn part_2(str: &str) -> String {
    let mut lines = str.lines();
    let line = match (lines.next(), lines.next()) {
        (Some(_), Some(l2)) => l2,
        _ => panic!("invalid input"),
    };
    let vec: Vec<(u32, usize)> = line
        .split(',')
        .enumerate()
        .map(|(i, v)| {
            if let Ok(num) = v.parse::<u32>() {
                Some((num, i))
            } else {
                None
            }
        })
        .filter(|o| if let Some(_) = o { true } else { false })
        .map(|o| o.unwrap())
        .collect();

    let (_, fst_index) = vec.first().unwrap();
    let vec_str: Vec<String> = vec.iter().map(|(num, index)| {
        let distance: usize = index - fst_index;
        if distance == 0 {
            format!("t mod {} = 0", num)
        } else {
            format!("(t + {}) mod {} = 0", distance, num)
        }
    }).collect();

    vec_str.join(" && ")
}

/// find bus
///
/// return timestamp and bus id
fn find_bus(ts: u32, ids: &str) -> (u32, u32) {
    let mut min_ts_id: (u32, u32) = (0, 0);

    for s_id in ids.split(',') {
        match s_id.parse::<u32>() {
            Ok(id) => {
                if ts % id == 0 {
                    return (id, 0);
                }

                // increase
                let id_ts = id * (ts / id + 1);
                if min_ts_id.0 == 0 || id_ts < min_ts_id.0 {
                    min_ts_id = (id_ts, id);
                }
            }
            _ => (),
        }
    }

    // return the min
    min_ts_id
}
