use std::{
    collections::HashMap,
    io::Read
};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
}

fn part_1(str: &str) -> usize {
    let mut lines = str.lines();

    let mut rules: HashMap<u32, Rule> = HashMap::new();
    lines.try_for_each(|l| {
        match l {
            e if e.is_empty() => None,
            ne => {
                let (index, rule) = parse_rule(ne);
                rules.insert(index, rule);
                Some(())
            }
        }
    });

    // process lines by lines
    lines.into_iter().filter(|l| {
        let r = is_valid_msg(l.as_bytes(), &rules, &rules.get(&0).unwrap());
        r.0 && r.1 == l.len()
    }).count()
}

fn parse_rule(str: &str) -> (u32, Rule) {
    let mut parts = str.split(": ");
    let prefix: u32 = parts.next().unwrap().parse().unwrap();
    
    // parse rule
    let mut content = parts.next().unwrap().split(" | ");
    let lhs: Rule = parse_prime_rule(content.next().unwrap());
    match content.next() {
        Some(ctx) => (prefix, Rule::Or(Box::new(lhs), Box::new(parse_prime_rule(ctx)))),
        _ => (prefix, lhs)
    }
}

/// parse atom or seq
fn parse_prime_rule(str: &str) -> Rule {
    let mut tokens = str.split(' ').peekable();
    match tokens.peek() {
        // parse atom
        Some(s) if *s.as_bytes().first().unwrap() == b'"' => {
            let mut chars = s.chars().skip(1);
            match (chars.next(), chars.next()) {
                (Some(a), Some('"')) => Rule::Atom(a),
                _ => panic!("invalid token")
            }
        },

        // parse sequence
        Some(_) => Rule::Seq(tokens.map(|v| v.parse::<u32>().unwrap()).collect()),
        _ => panic!("invalid token")
    }
}

fn is_valid_msg(bytes: &[u8], rules: &HashMap<u32, Rule>, rule: &Rule) -> (bool, usize) {
    //println!("is_valid_msg: {:?}, {}, for {:?}", bytes, bytes[0] as char, rule);
    match rule {
        Rule::Atom(ch) => if bytes[0] == (*ch as u8) {(true, 1)} else { (false, 0) },
        Rule::Seq(seq) => {
            let size = bytes.len();
            let mut len: usize = 0;
            for item in seq {

                // error: having seq but no more char
                if len >= size {
                    return (false, 0)
                }

                let r = rules.get(item).unwrap();
                match is_valid_msg(&bytes[len..size], rules, r) {
                    (true, l) => len = l as usize + len,
                    _ => return (false, 0)
                }
            }

            (true, len)
        },
        Rule::Or(lhs, rhs) => {
            match is_valid_msg(bytes, rules, lhs) {
                (true, len) => (true, len),
                _ => is_valid_msg(bytes, rules, rhs)
            }
        }
    }   
}

#[derive(Debug, PartialEq, Eq)]
enum Rule {
    Atom(char), // "a" or "b"
    Seq(Vec<u32>),
    Or(Box<Rule>, Box<Rule>),
}

#[cfg(test)]
mod tests {
    use crate::parse_rule;
    use crate::Rule;

    #[test]
    fn test_parse_rule_1() {
        assert_eq!((0, Rule::Seq(vec![4, 1, 5])), parse_rule("0: 4 1 5"))
    }

    #[test]
    fn test_parse_rule_2() {
        assert_eq!((4, Rule::Atom('a')), parse_rule("4: \"a\""))
    }

    #[test]
    fn test_parse_rule_3() {
        assert_eq!((1, Rule::Or(Box::new(Rule::Seq(vec![2, 3])), Box::new(Rule::Seq(vec![3, 2])))), parse_rule("1: 2 3 | 3 2"))
    }
}
