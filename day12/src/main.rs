use core::panic;
use std::{io::Read, str::Lines};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
}

fn part_1(str: &str) -> i32 {
    let mut lines = str.lines();
    let mut st: State = State {
        x: 0,
        y: 0,
        dir: Dir::East,
    };
    loop {
        let instr = next_instr(&mut lines);
        match instr {
            Some((ch, val)) => st.process(ch, val as i32),
            _ => break, // end of file
        }
    }

    println!("{:?}", st);

    st.calc_distance()
}

fn next_instr(lines: &mut Lines) -> Option<(char, usize)> {
    match lines.next() {
        Some(str) => {
            if str.len() < 2 {
                panic!("invalid input");
            }

            let num = &str[1..str.len()].parse::<usize>().unwrap();
            Some((str.chars().next().unwrap(), *num))
        }
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
struct State {
    x: i32,   // -> east (+), west (-)
    y: i32,   // -> north (+), south (-)
    dir: Dir, // -> current direction
}

impl State {
    fn process(&mut self, ch: char, val: i32) {
        if ch == 'L' || ch == 'R' {
            self.dir = self.new_direction(ch, val);
        }

        match ch {
            'L' | 'R' => (), // ignored
            'N' => self.y = self.y + val,
            'S' => self.y = self.y - val,
            'E' => self.x = self.x + val,
            'W' => self.x = self.x - val,
            'F' => match self.dir {
                Dir::North => self.y = self.y + val,
                Dir::South => self.y = self.y - val,
                Dir::East => self.x = self.x + val,
                Dir::West => self.x = self.x - val,
            },
            _ => panic!("invalid action: {}", ch),
        }
    }

    fn calc_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn new_direction(&self, ch: char, val: i32) -> Dir {
        if ch != 'L' && ch != 'R' {
            panic!("invalid arg: {}", ch);
        }

        let degree: i32 = self.dir.to_degree() + if ch == 'L' { -val } else { val };
        Dir::from_degree(degree)
    }
}

impl Dir {
    /// map direction into degree 0..360
    /// root at east, clockwise direction
    fn to_degree(&self) -> i32 {
        match self {
            Dir::East => 0,
            Dir::South => 90,
            Dir::West => 180,
            Dir::North => 270,
        }
    }

    fn from_degree(degree: i32) -> Self {
        let mut md = degree % 360;
        if md < 0 {
            md = 360 + md;
        }

        match md {
            0 => Dir::East,
            90 => Dir::South,
            180 => Dir::West,
            270 => Dir::North,
            _ => panic!("invalid degree: {}", md),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Dir;

    #[test]
    fn test_from_degree_north() {
        assert_eq!(Dir::North, Dir::from_degree(-90));
        assert_eq!(Dir::North, Dir::from_degree(270));
    }

    #[test]
    fn test_from_degree_south() {
        assert_eq!(Dir::South, Dir::from_degree(90));
        assert_eq!(Dir::South, Dir::from_degree(-270));
    }

    #[test]
    fn test_from_degree_east() {
        assert_eq!(Dir::East, Dir::from_degree(0));
        assert_eq!(Dir::East, Dir::from_degree(360));
        assert_eq!(Dir::East, Dir::from_degree(-360));
        assert_eq!(Dir::East, Dir::from_degree(720));
    }

    #[test]
    fn test_from_degree_west() {
        assert_eq!(Dir::West, Dir::from_degree(180));
        assert_eq!(Dir::West, Dir::from_degree(-180));
    }
}
