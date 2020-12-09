use std::io::Read;

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    
    println!("result part-1: {:?}", part_1(str.as_str()));
    println!("result part-2: {:?}", part_2(str.as_str()));
}

fn part_1(str: &str) -> u32 {
    let mut max: u32 = 0;
    for line in str.lines() {
        let id = get_id(get_seat(line));
        if id > max {
            max = id;
        }
    }

    return max
}

fn part_2(str: &str) -> u32 {
    // filter id
    let mut id = Vec::new();
    for line in str.lines() {
        let seat = get_seat(line);
        if seat.0 != 0 && seat.0 != 127 {
            id.push(get_id(seat));
        }
    }

    // sort
    if id.is_empty() {
        panic!("missing data");
    }
    id.sort();

    // find missing piece
    let mut prev = id[0];
    for i in 1..id.len() {
        let val = id[i];
        if val - 1 != prev {
            // invalid. val is ID + 1, prev is ID - 1
            return val - 1;
        }
        
        prev = val;
    }

    panic!("no ID found");
}

fn get_id(seat: (u32, u32)) -> u32 {
    return seat.0 * 8 + seat.1;
}

fn get_seat(str: &str) -> (u32, u32) {
    if str.len() < 10 {
        panic!("invalid line, expect at least 10 chars")
    }

    let bytes = str.as_bytes();

    // read col
    let col = get_value_in_range(&bytes[0..=6], 0, 127, b'F', b'B');
    let row = get_value_in_range(&bytes[7..=9], 0, 7, b'L', b'R');
    return (col, row)
}

fn get_value_in_range(bytes: &[u8], min: u32, max: u32, lower: u8, upper: u8) -> u32 {
    let mut range = (min, max);
    let len = bytes.len();
    for i in 0..len {
        let mid: u32 = range.0 + (range.1 - range.0 + 1) / 2;
        let b = bytes[i];
        range = match b {
            b if b == lower => (range.0, mid - 1),
            b if b == upper => (mid, range.1),
            _ => panic!("unexpected char: {}", b)
        }
    }

    // final
    let col = match bytes[len-1] {
        b if b == lower => range.0,
        b if b == upper => range.1,
        _ => panic!("unexpected char", )
    };

    return col;
}

#[cfg(test)]
mod tests {
    use crate::{get_seat};

    #[test]
    fn test_get_seat() {
        assert_eq!((44, 5), get_seat("FBFBBFFRLR"));
        assert_eq!((70, 7), get_seat("BFFFBBFRRR"));
        assert_eq!((14, 7), get_seat("FFFBBBFRRR"));
        assert_eq!((102, 4), get_seat("BBFFBBFRLL"));
    }
}