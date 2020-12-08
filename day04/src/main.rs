use std::{*, io::Read, str};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    
    println!("result part-1: {:?}", part_1(str.as_str()));
}

fn part_1(str: &str) -> u32 {
    let mut cnt = 0;
    let mut chars = str.chars();
    loop {
        if let Some(pass) = next_passport(&mut chars) {
            println!("next: {:?}", pass);
            if pass.is_valid_presence() {
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
}