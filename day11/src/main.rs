use core::panic;
use std::{fmt::Display, io::Read};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
    // println!("result part-2: {:?}", part_2(&str));
}

fn part_1(str: &str) -> usize {
    let mut vec = parse_vec(str);
    let mut i = 0;
    loop {
        let (tmp1, changed1) = if i % 2 == 0 {
            run_rule_1(&vec)
        } else {
            run_rule_2(&vec)
        };

        if !changed1 {
            println!("no more change, stop at round: {}", i);
            break;
        }

        vec = tmp1;
        i = i + 1;
    }

    count_occupied(&vec)
}

fn part_2(str: &str) -> usize {
    let mut vec = parse_vec(str);
    let (tmp, _) = run_rule_3(&vec);
    print_grid(&tmp);

    count_occupied(&tmp)
}

fn run_rule_1(vec: &Vec<Vec<State>>) -> (Vec<Vec<State>>, bool) {
    run_rule(vec, |i: usize, j: usize, item: &State| {
        if *item == State::Empty && !has_occupied(vec, i, j, 1) {
            (State::Occupied, true)
        } else {
            (*item, false)
        }
    })
}

fn run_rule_2(vec: &Vec<Vec<State>>) -> (Vec<Vec<State>>, bool) {
    run_rule(vec, |i: usize, j: usize, item: &State| {
        if *item == State::Occupied && has_occupied(vec, i, j, 4) {
            (State::Empty, true)
        } else {
            (*item, false)
        }
    })
}

fn run_rule_3(vec: &Vec<Vec<State>>) -> (Vec<Vec<State>>, bool) {
    run_rule(vec, |i: usize, j: usize, item: &State| {
        if *item == State::Empty && !has_occupied(vec, i, j, 1) {
            (State::Occupied, true)
        } else {
            (*item, false)
        }
    })
}

fn run_rule<F>(vec: &Vec<Vec<State>>, mut f: F) -> (Vec<Vec<State>>, bool)
where
    F: FnMut(usize, usize, &State) -> (State, bool),
{
    let mut new_vec: Vec<Vec<State>> = Vec::new();
    let mut changed = false;
    for (i, row_vec) in vec.iter().enumerate() {
        new_vec.push(
            row_vec
                .iter()
                .enumerate()
                .map(|(j, item)| {
                    let (st, ch) = f(i, j, item);
                    if ch {
                        changed = ch;
                    }

                    st
                })
                .collect(),
        );
    }

    (new_vec, changed)
}

fn has_occupied(vec: &Vec<Vec<State>>, i: usize, j: usize, min: u32) -> bool {
    let check: Vec<(usize, usize)> = get_closed_adjacent_list(vec, i, j);
    let mut ctx: u32 = 0;
    check.iter().any(|(x, y)| {
        if let Some(row_vec) = vec.get(*x) {
            match row_vec.get(*y) {
                Some(State::Occupied) => {
                    ctx = ctx + 1;
                    ctx >= min
                }
                _ => false,
            }
        } else {
            false
        }
    })
}

fn get_closed_adjacent_list(vec: &Vec<Vec<State>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut check: Vec<(usize, usize)> = Vec::new();
    if i > 0 {
        if j > 0 {
            check.push((i - 1, j - 1));
        }
        check.push((i - 1, j));
        check.push((i - 1, j + 1));
    }

    if j > 0 {
        check.push((i, j - 1));
    }
    check.push((i, j + 1));

    if i < vec.len() - 1 {
        if j > 0 {
            check.push((i + 1, j - 1));
        }
        check.push((i + 1, j));
        check.push((i + 1, j + 1));
    }

    check
}

fn count_occupied(vec: &Vec<Vec<State>>) -> usize {
    vec.iter()
        .map(|row| row.iter().filter(|i| **i == State::Occupied).count())
        .sum()
}

fn print_grid(vec: &Vec<Vec<State>>) {
    vec.iter().for_each(|row| {
        row.iter().for_each(|i| print!("{}", i));
        println!("")
    })
}

fn parse_vec(str: &str) -> Vec<Vec<State>> {
    // parse input to vector
    str.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    '.' => State::Floor,
                    _ => panic!("invalid input: {}", c),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    Empty,
    Occupied,
    Floor,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                State::Empty => "L",
                State::Occupied => "#",
                _ => ".",
            }
        )
    }
}
