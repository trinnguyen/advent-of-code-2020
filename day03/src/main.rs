
use std::io::{self, Read, Lines};

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let str = buffer.as_str();
    println!("result part-1: {:?}", part_1(str));
    println!("result part-2: {:?}", part_2(str));
}

fn part_1(buf: &str) -> u32 {
    let mut idx: usize = 0;
    let mut count: u32 = 0;

    let lines = buf.lines();
    for str in lines {
        // check the current mark
        let bytes = str.as_bytes();
        let len = bytes.len();


        // first line is skipped
        if idx > 0 {
            // out of range -> duplicate the line
            if idx > len - 1 {
                idx = idx % len
            }

            if bytes[idx] == b'#' {
                count += 1;
            }
        }

        // go to next round

        idx = idx + 3;
    }

    return count;
}

fn part_2(buf: &str) -> u32 {
    let mut idx_11: usize = 0;
    let mut count_11: u32 = 0;

    let mut idx_31: usize = 0;
    let mut count_31: u32 = 0;

    let mut idx_51: usize = 0;
    let mut count_51: u32 = 0;

    let mut idx_71: usize = 0;
    let mut count_71: u32 = 0;

    let mut idx_12: usize = 0;
    let mut count_12: u32 = 0;
    
    let lines = buf.lines();
    let mut line_idx: u32 = 0;
    for str in lines {
        // check the current mark
        let bytes = str.as_bytes();
        let len = bytes.len();

        // first line is skipped
        update(&mut idx_11, &mut count_11, &len, bytes, 1);
        update(&mut idx_31, &mut count_31, &len, bytes, 3);
        update(&mut idx_51, &mut count_51, &len, bytes, 5);
        update(&mut idx_71, &mut count_71, &len, bytes, 7);

        // down 2
        if line_idx % 2 == 0 {
            update( &mut idx_12, &mut count_12, &len, bytes, 1);
        }
        line_idx += 1;
    }

    return count_11 * count_31 * count_51 * count_71 * count_12;
}

fn update(idx: &mut usize, count: &mut u32, len: &usize, bytes: &[u8], right: usize) {

    // skip the first one
    if *idx > 0 {
        // out of range -> duplicate the line
        if *idx > *len - 1 {
            *idx = *idx % *len
        }

        if bytes[*idx] == b'#' {
            *count += 1;
        }
    }

    // go to next round
    *idx = *idx + right;
}