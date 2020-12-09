use std::io::Read;
use std::str::Lines;
use std::collections::HashSet;

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();

    println!("result part-1: {:?}", part_1(str.as_str()));
    println!("result part-2: {:?}", part_2(str.as_str()));
}

fn part_1(str: &str) -> usize {
    let mut sum = 0;
    let mut lines = str.lines();
    loop {
        match next_group_disjoint(&mut lines) {
            Some(gr) => sum += gr.len(),
            _ => break
        }
    }

    return sum;
}

fn part_2(str: &str) -> usize {
    let mut sum = 0;
    let mut lines = str.lines();
    loop {
        match next_group_common(&mut lines) {
            Some(gr) => sum += gr.len(),
            _ => break
        }
    }

    return sum;
}

fn next_group_disjoint(line: &mut Lines) -> Option<HashSet<char>> {
    return match line.next() {
        Some(str) => {
            let mut answers = HashSet::new();
            let mut val = str;
            loop {

                // empty line -> end of group
                if val.is_empty() {
                    return Some(answers)
                }

                // add all char
                val.chars()
                    .filter(|c| matches!(c, 'a'..='z'))
                    .for_each(|c| {
                        let _ = answers.insert(c);
                    });

                // next line
                match line.next() {
                    Some(str2) => val = str2,
                    _ => val = "" // EOF
                }
            }
        },
        _ => None
    }
}

fn next_group_common(line: &mut Lines) -> Option<HashSet<char>> {
    return match line.next() {
        Some(str) => {
            let mut answers = HashSet::new();

            // add first line
            str.chars()
                .filter(|c| matches!(c, 'a'..='z'))
                .for_each(|c| {
                    let _ = answers.insert(c);
                });

            // next line
            loop {
                match line.next() {
                    Some(str2) => {

                        // empty line -> end of group
                        if str2.is_empty() {
                            break
                        }

                        // find common
                        let common: Vec<char> = str2.chars()
                            .filter(|c| {
                                matches!(c, 'a'..='z') && answers.contains(c)
                            }).collect();

                        // update existing
                        answers.clear();
                        common.iter().for_each(|c| {
                            let _ = answers.insert(*c);
                        });
                    },
                    _ => break
                }
            }

            return Some(answers)
        },
        _ => None
    }
}