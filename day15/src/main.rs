use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
}

fn part_1(str: &str) -> u32 {
    let vec: Vec<u32> = str.split(',').map(|c| c.parse::<u32>().unwrap()).collect();
    calc_part_1(&vec)
}

fn calc_part_1(vec: &Vec<u32>) -> u32 {
    let mut last_num = 0;
    let mut turn: usize = 0;
    let mut map: HashMap<u32, (usize, usize)> = HashMap::new();

    while turn < 2020 {
        turn = turn + 1;
        
        last_num = match vec.get(turn - 1) {
            Some(v) => *v,
            None => {
                // check last number
                match map.get(&last_num) {
                    Some((idx_fst, idx_snd)) => {
                        // it was spoken only once (last turn was its first time)
                        if *idx_snd == 0 {
                            0
                        } else {
                            // subtract 
                            (idx_snd - idx_fst) as u32
                        }
                    },
                    None => panic!("unexpected error") // never catch
                }
            }
        };

        // update map
        let new_pair: (usize, usize) = match map.get(&last_num) {
            Some((idx_fst, 0)) => (*idx_fst, turn),
            Some((_, idx_snd)) => (*idx_snd, turn),
            None => (turn, 0)
        };
        map.insert(last_num, new_pair);
    }

    last_num
}

#[cfg(test)]
mod tests {
    use crate::calc_part_1;

    #[test]
    fn test_part_1() {
        assert_eq!(436, calc_part_1(&vec![0, 3, 6]));
        assert_eq!(1, calc_part_1(&vec![1,3,2]));
        assert_eq!(10, calc_part_1(&vec![2,1,3]));
        assert_eq!(27, calc_part_1(&vec![1,2,3]));
    }
}
