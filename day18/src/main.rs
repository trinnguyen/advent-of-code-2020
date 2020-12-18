use std::{io::Read, str::Chars};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();
    println!("result part-1: {:?}", part_1(&str));
    println!("result part-2: {:?}", part_2(&str));
}

fn part_1(str: &str) -> u64 {
    str.lines().fold(0, |acc, line| acc + execute_line_1(line))
}

fn execute_line_1(str: &str) -> u64 {
    let mut scanner = create_scanner(str);
    if let Some(expr) = parse_expression_1(&mut scanner) {
        compute_expr(&expr)
    } else {
        panic!("no expression found")
    }
}

fn create_scanner(str: &str) -> Scanner {
    let input = Input {
        chars: str.chars(),
        lookahead: Vec::new(),
    };
    Scanner {
        input: input,
        lookahead: Vec::new(),
    }
}

fn parse_expression_1(scanner: &mut Scanner) -> Option<Expr> {
    match parse_prime_expression(scanner, false) {
        Some(lhs) => {
            let mut expr = lhs;
            loop {
                match scanner.peek() {
                    Token::Add | Token::Mul => {
                        let op = match scanner.next_token() {
                            Token::Add => Op::Add,
                            Token::Mul => Op::Mul,
                            _ => break,
                        };

                        // rhs
                        match parse_prime_expression(scanner, false) {
                            Some(rhs) => expr = Expr::Binary(op, Box::new(expr), Box::new(rhs)),
                            _ => panic!("expected expression but EOF"),
                        };
                    }
                    _ => break,
                }
            }
            Some(expr)
        }
        _ => None,
    }
}

fn part_2(str: &str) -> u64 {
    str.lines().fold(0, |acc, line| acc + execute_line_2(line))
}

fn execute_line_2(str: &str) -> u64 {
    let mut scanner = create_scanner(str);
    if let Some(expr) = parse_expression_2(&mut scanner) {
        compute_expr(&expr)
    } else {
        panic!("no expression found")
    }
}

fn parse_expression_2(scanner: &mut Scanner) -> Option<Expr> {
    parse_mul_expr(scanner)
}

// addExpr { * addExpr}
fn parse_mul_expr(scanner: &mut Scanner) -> Option<Expr> {
    match parse_add_expr(scanner) {
        Some(lhs) => {
            let mut expr = lhs;
            loop {
                match scanner.peek() {
                    Token::Mul => {
                        scanner.next_token();
                        match parse_add_expr(scanner) {
                            Some(rhs) => {
                                expr = Expr::Binary(Op::Mul, Box::new(expr), Box::new(rhs))
                            }
                            _ => panic!("expected rhs expr"),
                        }
                    }
                    _ => break,
                }
            }

            Some(expr)
        }
        _ => None,
    }
}

fn parse_add_expr(scanner: &mut Scanner) -> Option<Expr> {
    match parse_prime_expression(scanner, true) {
        Some(lhs) => {
            let mut expr = lhs;
            loop {
                match scanner.peek() {
                    Token::Add => {
                        scanner.next_token();
                        match parse_prime_expression(scanner, true) {
                            Some(rhs) => {
                                expr = Expr::Binary(Op::Add, Box::new(expr), Box::new(rhs))
                            }
                            _ => panic!("expected rhs expr"),
                        }
                    }
                    _ => break,
                }
            }
            
            Some(expr)
        }
        _ => None,
    }
}

fn parse_prime_expression(scanner: &mut Scanner, is_part_2: bool) -> Option<Expr> {
    // expression always start with a number or a parent-open
    match scanner.next_token() {
        Token::Num(val) => Some(Expr::Const(val)),
        Token::ParentOpen => Some(parse_group(scanner, is_part_2)),
        Token::EOF => None,
        tok => panic!("expected number or ( but {:?}", tok),
    }
}

fn parse_group(scanner: &mut Scanner, is_part_2: bool) -> Expr {
    let ex = if is_part_2 {
        parse_expression_2(scanner)
    } else {
        parse_expression_1(scanner)
    };
    match (ex, scanner.next_token()) {
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

#[derive(Debug, Clone, Copy)]
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
        match self.lookahead.pop() {
            Some(c) => Some(c),
            _ => self.chars.next(),
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.lookahead.len() > 0 {
            Some(*self.lookahead.last().unwrap())
        } else {
            match self.chars.next() {
                Some(ch) => {
                    self.lookahead.push(ch);
                    Some(ch)
                }
                _ => None,
            }
        }
    }
}

struct Scanner<'a> {
    input: Input<'a>,
    lookahead: Vec<Token>,
}

impl Scanner<'_> {
    fn next_token(&mut self) -> Token {
        let tok = match self.lookahead.pop() {
            Some(c) => c,
            _ => self.advance_token(),
        };

        //println!("next tok: {:?}", tok);
        tok
    }

    fn peek(&mut self) -> Token {
        if self.lookahead.len() > 0 {
            *self.lookahead.last().unwrap()
        } else {
            let tok = self.advance_token();
            self.lookahead.push(tok);
            tok
        }
    }

    fn advance_token(&mut self) -> Token {
        match self.input.next() {
            Some(c) => {
                let mut ch = c;

                // skip white space
                while ch.is_ascii_whitespace() {
                    if let Some(c) = self.input.next() {
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
                            match self.input.peek() {
                                Some(next_digit) if next_digit.is_ascii_digit() => {
                                    str_num.push(self.input.next().unwrap())
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
}

#[cfg(test)]
mod tests {
    use crate::execute_line_1;
    use crate::execute_line_2;

    #[test]
    fn test_line_0() {
        assert_eq!(71, execute_line_1("1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn test_line_1() {
        assert_eq!(26, execute_line_1("2 * 3 + (4 * 5)"));
    }

    #[test]
    fn test_line_2() {
        assert_eq!(437, execute_line_1("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    }

    #[test]
    fn test_line_3() {
        assert_eq!(
            12240,
            execute_line_1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
    }

    #[test]
    fn test_line_4() {
        assert_eq!(
            13632,
            execute_line_1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }

    #[test]
    fn test_line_2_1() {
        assert_eq!(51, execute_line_2("1 + (2 * 3) + (4 * (5 + 6))"));
    }

    #[test]
    fn test_line_2_2() {
        assert_eq!(46, execute_line_2("2 * 3 + (4 * 5)"));
    }

    #[test]
    fn test_line_2_3() {
        assert_eq!(1445, execute_line_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    }

    #[test]
    fn test_line_2_4() {
        assert_eq!(
            669060,
            execute_line_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
    }
}
