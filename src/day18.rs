use crate::{Fail, Input, Solution};

pub static DATA_PATH: &str = "data/day18.txt";
pub static TEST_PATH: &str = "data/test/day18.txt";
pub static TEST_VALUES: (&str, &str) = ("122", "282");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Char(char),
    Int(i64),
}

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<Vec<Token>> = self
            .data
            .lines()
            .map(|l| {
                l.chars()
                    .filter(|c| *c != ' ')
                    .map(|c| match c.to_string().parse::<i64>() {
                        Ok(n) => Token::Int(n),
                        _ => Token::Char(c),
                    })
                    .collect()
            })
            .collect();
        Ok(input
            .iter()
            .map(|v| {
                let (r, first) = parse_term_lr(&v);
                parse_ops_lr(first.unwrap(), r).unwrap()
            })
            .sum::<i64>()
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<Vec<Token>> = self
            .data
            .lines()
            .map(|l| {
                l.chars()
                    .filter(|c| *c != ' ')
                    .map(|c| match c.to_string().parse::<i64>() {
                        Ok(n) => Token::Int(n),
                        _ => Token::Char(c),
                    })
                    .collect()
            })
            .collect();
        Ok(input
            .iter()
            .map(|v| {
                let (r, first) = parse_term_prec(&v);
                parse_ops_prec(first.unwrap(), r).unwrap()
            })
            .sum::<i64>()
            .to_string())
    }
}

fn parse_ops_lr(acc: i64, s: &[Token]) -> Option<i64> {
    if s.len() <= 1 {
        Some(acc)
    } else {
        let (r, t) = parse_term_lr(&s[1..]);
        match s[0] {
            Token::Char('+') => parse_ops_lr(acc + t?, r),
            Token::Char('*') => parse_ops_lr(acc * t?, r),
            _ => None,
        }
    }
}

fn parse_term_lr(s: &[Token]) -> (&[Token], Option<i64>) {
    if s.len() <= 1 {
        match s[0] {
            Token::Int(i) => return (&[], Some(i)),
            _ => return (&[], None),
        }
    }
    match s[0] {
        Token::Int(n) => (&s[1..], Some(n)),
        Token::Char('(') => {
            let i: usize = s
                .iter()
                .fold((0, 0), match_brackets)
                .1;
            let (r, first) = parse_term_lr(&s[1..i - 1]);
            (&s[i..], parse_ops_lr(first.unwrap(), r))
        }
        _ => (s, None),
    }
}

fn parse_ops_prec(acc: i64, s: &[Token]) -> Option<i64> {
    if s.len() <= 1 {
        Some(acc)
    } else {
        let (r, t) = parse_term_prec(&s[1..]);
        match s[0] {
            Token::Char('+') => parse_ops_prec(acc + t?, r),
            Token::Char('*') => {
                let (r, t) = parse_add_expr(t?, r);
                parse_ops_prec(acc * t?, r)
            }
            _ => None,
        }
    }
}

fn parse_add_expr(acc: i64, s: &[Token]) -> (&[Token], Option<i64>) {
    if s.len() <= 1 || s[0] == Token::Char('*') {
        (s, Some(acc))
    } else {
        let (r, t) = parse_term_prec(&s[1..]);
        match s[0] {
            Token::Char('+') => parse_add_expr(acc + t.unwrap(), r),
            _ => (s, None),
        }
    }
}

fn parse_term_prec(s: &[Token]) -> (&[Token], Option<i64>) {
    if s.len() <= 1 {
        match s[0] {
            Token::Int(i) => return (&[], Some(i)),
            _ => return (&[], None),
        }
    }
    match s[0] {
        Token::Int(n) => (&s[1..], Some(n)),
        Token::Char('(') => {
            let i: usize = s
                .iter()
                .fold((0, 0), match_brackets)
                .1;
            let (r, first) = parse_term_prec(&s[1..i - 1]);
            (&s[i..], parse_ops_prec(first.unwrap(), r))
        }
        _ => (s, None),
    }
}

fn match_brackets((depth, index): (i64, usize), c: &Token) -> (i64, usize) {
    if depth == 0 && index != 0 {
        (depth, index)
    } else {
        match c {
            Token::Char('(') => (depth + 1, index + 1),
            Token::Char(')') => (depth - 1, index + 1),
            _ => (depth, index + 1),
        }
    }
}