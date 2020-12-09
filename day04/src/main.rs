use std::{*, io::Read, str};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    
    println!("result part-1: {:?}", part_1(str.as_str()));
    println!("result part-2: {:?}", part_2(str.as_str()));
}

fn part_1(str: &str) -> u32 {
    let mut cnt = 0;
    let mut chars = str.chars();
    loop {
        if let Some(pass) = next_passport(&mut chars) {
            if pass.is_valid_presence() {
                cnt = cnt + 1;
            }
        } else {
            break
        }
    }

    return cnt;
}

fn part_2(str: &str) -> u32 {
    let mut cnt = 0;
    let mut chars = str.chars();
    loop {
        if let Some(pass) = next_passport(&mut chars) {
            if pass.is_valid_value() {
                cnt = cnt + 1;
            }
        } else {
            break
        }
    }

    return cnt;
}

fn next_passport(chars: &mut str::Chars) -> Option<PassKey> {
    let mut tok = next_tok(chars);
    if tok == None  {
        return None
    }

    let mut pass : PassKey = Default::default();
    loop {
        // found key pair
        match tok {
            // key followed by value
            Some(Token::Key{key}) => {
                if let Some(Token::Value{val}) = next_tok(chars) {
                    match key.as_str() {
                        "byr" => pass.byr = Some(val),
                        "iyr" => pass.iyr = Some(val),
                        "eyr" => pass.eyr = Some(val),
                        "hgt" => pass.hgt = Some(val),
                        "hcl" => pass.hcl = Some(val),
                        "ecl" => pass.ecl = Some(val),
                        "pid" => pass.pid = Some(val),
                        "cid" => pass.cid = Some(val),
                        _ => panic!("unexpected token key: {}", key)
                    };
                } else {
                    panic!("expected token value for key: {}", key)
                }
            },
            // new line -> break
            _ => break
        }

        tok = next_tok(chars);
    }
    
    return Some(pass);
}

fn next_tok(chars: &mut str::Chars) -> Option<Token> {
    if let Some(ch) = chars.next() {

        if ch == '\n' {
            return Some(Token::NewLine);
        }

        // key or value
        let mut val = String::new();
        val.push(ch);
        loop {
            if let Some(ch) = chars.next() {

                // key
                if ch == ':' {
                    return Some(Token::Key {key: val});
                }

                // value
                if ch.is_ascii_whitespace() || ch == '\n' {
                    return Some(Token::Value {val});
                }

                // consume
                val.push(ch);
            } else {
                return Some(Token::Value {val});
            }
        }

    } else {
        return None;
    }
}

#[derive(PartialEq, Debug)]
enum Token {
    NewLine,
    Key {key: String},
    Value {val: String}
}

#[derive(Default, Debug)]
struct PassKey {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}

impl PassKey {
    fn is_valid_presence(&self) -> bool {
        return self.has_value(&self.byr) 
            && self.has_value(&self.iyr) 
            && self.has_value(&self.eyr) 
            && self.has_value(&self.hgt)
            && self.has_value(&self.hcl)
            && self.has_value(&self.ecl)
            && self.has_value(&self.pid);
    }

    fn has_value(&self, opt: &Option<String>) -> bool {
        return match opt {
            Some(_) => true,
            _ => false
        }
    }

    fn is_valid_value(&self) -> bool {
        return self.is_valid_presence()
            && self.is_in_range(&self.byr, 1920, 2002)
            && self.is_in_range(&self.iyr, 2010, 2020)
            && self.is_in_range(&self.eyr, 2020, 2030)
            && self.is_valid_height(&self.hgt)
            && self.is_valid_color(&self.hcl)
            && self.is_valid_ecl(&self.ecl)
            && self.is_valid_pid(&self.pid)
    }

    fn is_valid_ecl(&self, opt: &Option<String>) -> bool {
        // eye color
        if let Some(val) = opt {
            return match val.as_str() {
                "amb" => true,
                "blu" => true, 
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false
            }
        }

        return false
    }

    fn is_valid_pid(&self, opt: &Option<String>) -> bool {
        // Passport ID
        if let Some(val) = opt {
            if val.len() != 9 {
                return false
            }

            let bytes = val.as_bytes();
            for b in bytes {
                if !b.is_ascii_digit() {
                    return false
                }
            }

            return true
        }

        return false
    }

    fn is_in_range(&self, opt: &Option<String>, min: u32, max: u32) -> bool {
        if let Some(str) = opt {
            if let Ok(val) = str.parse::<u32>() {
                return val >= min && val <= max;
            }
        }

        return false
    }

    fn is_valid_height(&self, opt: &Option<String>) -> bool {
        if let Some(str) = opt {
            if str.ends_with("cm") {
                if let Ok(val) = str[0..(str.len() - 2)].parse::<u32>() {
                    return val >= 150 && val <= 193;
                }
            } else if str.ends_with("in") {
                if let Ok(val) = str[0..(str.len() - 2)].parse::<u32>() {
                    return val >= 59 && val <= 76;
                }
            }
        }

        return false;
    }

    fn is_valid_color(&self, opt: &Option<String>) -> bool {
        if let Some(str) = opt {
            let bytes = str.as_bytes();

            if bytes[0] != b'#' {
                return false;
            }

            if bytes.len() != 7 {
                return false;
            }

            for i in 1..=6 {
                let ch = bytes[i];
                if !(matches!(ch, b'0'..=b'9') || matches!(ch, b'a'..=b'f')) {
                    return false;
                }
            }

            return true;
        }

        return false;
    }
}