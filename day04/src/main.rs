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
            if pass.is_valid() {
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
                if let Some(Token::Value{val: _}) = next_tok(chars) {
                    match key.as_str() {
                        "byr" => pass.byr = true,
                        "iyr" => pass.iyr = true,
                        "eyr" => pass.eyr = true,
                        "hgt" => pass.hgt = true,
                        "hcl" => pass.hcl = true,
                        "ecl" => pass.ecl = true,
                        "pid" => pass.pid = true,
                        "cid" => pass.cid = true,
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
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool
}

impl PassKey {
    fn is_valid(&self) -> bool {
        return self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid;
    }
}