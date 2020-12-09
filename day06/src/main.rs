use std::io::Read;
use std::str::Lines;
use std::collections::HashSet;

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();

    println!("result part-1: {:?}", part_1(str.as_str()));
    // println!("result part-2: {:?}", part_2(str.as_str()));
}

fn part_1(str: &str) -> usize {
    let mut sum = 0;
    let mut lines = str.lines();
    loop {
        if let Some(gr) = next_group(&mut lines) {
            let len = gr.len();
            println!("new group: {}", len);
            sum += len;
        } else {
            break;
        }
    }

    return sum;
}

fn next_group(line: &mut Lines) -> Option<HashSet<char>> {
    return match line.next() {
        Some(str) => {
            let mut answers = HashSet::new();
            let mut val = str;
            loop {

                // empty line -> end of group
                if val.is_empty() {
                    println!("{:?}", answers);
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