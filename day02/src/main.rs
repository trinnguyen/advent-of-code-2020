
use std::io::Read;

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    
    calc_2(str.lines());
    calc_1(str.lines());
}

fn calc_1(lines: std::str::Lines) -> () {
    let mut global_cnt = 0;

    for line in lines {
        // fmt: 1-3 a: abcde
        let (min, max, ch, pwd) = parse_line(line);
        if max < min {
            continue;
        }

        // go through password
        let mut cnt = 0;
        for c in pwd.chars() {
            if c == ch {
                cnt = cnt + 1;

                // ignore immediately
                if cnt > max {
                    break;
                }
                
            }
        }

        // till the end, check with range
        if cnt >= min && cnt <= max {
            global_cnt = global_cnt + 1;
        }
    }

    println!("result: {}", global_cnt);
}

fn calc_2(lines: std::str::Lines) -> () {
    let mut global_cnt = 0;

    for line in lines {
        // fmt: 1-3 a: abcde
        let (fst, snd, ch, pwd) = parse_line(line);
        if fst == 0 || snd == 0 {
            continue;
        }

        let bytes = pwd.as_bytes();
        let is_fst_matched = bytes[fst - 1] == ch as u8;
        let is_snd_matched = bytes[snd - 1] == ch as u8;
        if (is_fst_matched || is_snd_matched) && !(is_fst_matched && is_snd_matched) {
            global_cnt = global_cnt + 1;
        }
    }

    println!("result: {}", global_cnt);
}

fn parse_line(line: &str) -> (usize, usize, char, &str) {
    // 1-3 a: abcde
    let items: Vec<_> = line.split(' ').collect();

    // range
    let range: Vec<_> = items.get(0).unwrap().split('-').collect();
    let min: usize = range.get(0).unwrap().parse().unwrap();
    let max: usize = range.get(1).unwrap().parse().unwrap();

    // char
    let ch = items.get(1).unwrap().chars().next().unwrap();

    // last
    let pwd = items.get(2).unwrap();
    return (min, max, ch, pwd);
}