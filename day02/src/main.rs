
use std::io::Read;

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    
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
 

fn parse_line(line: &str) -> (u32, u32, char, &str) {
    // 1-3 a: abcde
    let items: Vec<_> = line.split(' ').collect();

    // range
    let range: Vec<_> = items.get(0).unwrap().split('-').collect();
    let min: u32 = range.get(0).unwrap().parse().unwrap();
    let max: u32 = range.get(1).unwrap().parse().unwrap();

    // char
    let ch = items.get(1).unwrap().chars().next().unwrap();

    // last
    let pwd = items.get(2).unwrap();
    return (min, max, ch, pwd);
}