use std::{io::Read, str::Chars};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
}

fn part_1(str: &str) -> u64 {
    str.lines().fold(0, |acc, line| acc + execute_line(line))
}

fn execute_line(str: &str) -> u64 {
    let mut input = Input {
        chars: str.chars(),
        lookahead: Vec::new(),
    };
    if let Some(expr) = parse_expression(&mut input) {
        compute_expr(&expr)
    } else {
        panic!("no expression found")
    }
}

fn parse_expression(input: &mut Input) -> Option<Expr> {
    match parse_prime_expression(input) {
        Some(lhs) => {
            let mut expr = lhs;
            loop {
                match input.peek() {
                    Some('+') | Some('*') => {
                        let op = match next_token(input) {
                            Token::Add => Op::Add,
                            Token::Mul => Op::Mul,
                            _ => break,
                        };

                        // rhs
                        match parse_prime_expression(input) {
                            Some(rhs) => expr = Expr::Binary(op, Box::new(expr), Box::new(rhs)),
                            _ => panic!("expected expression but EOF"),
                        };
                    }
                    _ => break
                }
            }
            Some(expr)
        },
        _ => None
    }
}


fn parse_prime_expression(input: &mut Input) -> Option<Expr> {
    // expression always start with a number or a parent-open
    match next_token(input) {
        Token::Num(val) => Some(Expr::Const(val)),
        Token::ParentOpen => Some(parse_group(input)),
        Token::EOF => None,
        tok => panic!("expected number or ( but {:?}", tok),
    }
}

fn parse_group(input: &mut Input) -> Expr {
    match (parse_expression(input), next_token(input)) {
        (None, _) => panic!("expected expr"),
        (Some(expr), Token::ParentClose) => Expr::Group(Box::new(expr)),
        (Some(_), tok) => panic!("expected ) but {:?}", tok),
    }
}

fn compute_expr(expr: &Expr) -> u64 {
    match expr {
        Expr::Const(num) => *num,
        Expr::Binary(op, lhs, rhs) => match *op {
            Op::Add => compute_expr(lhs) + compute_expr(rhs),
            Op::Mul => compute_expr(lhs) * compute_expr(rhs),
        },
        Expr::Group(expr) => compute_expr(expr),
    }
}

fn next_token(input: &mut Input) -> Token {
    match input.next() {
        Some(c) => {
            let mut ch = c;

            // skip white space
            while ch.is_ascii_whitespace() {
                if let Some(c) = input.next() {
                    ch = c
                } else {
                    return Token::EOF;
                }
            }

            // case
            match ch {
                '+' => Token::Add,
                '*' => Token::Mul,
                '(' => Token::ParentOpen,
                ')' => Token::ParentClose,
                digit if digit.is_ascii_digit() => {
                    let mut str_num = String::from("");
                    str_num.push(digit);
                    loop {
                        match input.peek() {
                            Some(next_digit) if next_digit.is_ascii_digit() => {
                                str_num.push(input.next().unwrap())
                            }
                            _ => break,
                        }
                    }

                    // parse to int
                    Token::Num(str_num.parse::<u64>().unwrap())
                }
                _ => panic!("unexpected char: {}", ch),
            }
        }
        _ => Token::EOF,
    }
}

#[derive(Debug)]
enum Expr {
    Const(u64),
    Binary(Op, Box<Expr>, Box<Expr>),
    Group(Box<Expr>),
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
enum Token {
    ParentOpen,
    ParentClose,
    Add,
    Mul,
    Num(u64),
    EOF,
}

struct Input<'a> {
    chars: Chars<'a>,
    lookahead: Vec<char>,
}

impl Input<'_> {

    fn next(&mut self) -> Option<char> {
        let r = match self.lookahead.pop() {
            Some(c) => Some(c),
            _ => self.chars.next(),
        };

        r
    }

    fn peek(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.lookahead.push(c);
                Some(c)
            },
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::execute_line;

    #[test]
    fn test_line_0() {
        assert_eq!(71, execute_line("1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn test_line_1() {
        assert_eq!(26, execute_line("2 * 3 + (4 * 5)"));
    }

    #[test]
    fn test_line_2() {
        assert_eq!(437, execute_line("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    }

    #[test]
    fn test_line_3() {
        assert_eq!(
            12240,
            execute_line("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
    }

    #[test]
    fn test_line_4() {
        assert_eq!(
            13632,
            execute_line("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
