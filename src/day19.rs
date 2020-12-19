use crate::{Fail, Input, Solution};
use std::collections::HashMap;
use std::iter::FromIterator;

pub static DATA_PATH: &str = "data/day19.txt";
pub static TEST_PATH: &str = "data/test/day19.txt";
pub static TEST_VALUES: (&str, &str) = ("3", "12");

#[derive(Debug)]
pub enum Rule {
    Terminal(char),
    NonTerminal(Vec<i32>),
    Branch(Vec<i32>, Vec<i32>),
}

#[derive(Debug)]
pub struct Grammar {
    rules: HashMap<i32, Rule>,
}

impl From<&str> for Grammar {
    fn from(s: &str) -> Grammar {
        Grammar {
            rules: HashMap::from_iter(s.lines().map(|l| {
                let words: Vec<&str> = l.split(" ").collect();
                let rule_number: i32 = words[0][..words[0].len() - 1].parse().unwrap();
                if l.contains("\"") {
                    (
                        rule_number,
                        Rule::Terminal(words[1].chars().nth(1).unwrap()),
                    )
                } else if l.contains("|") {
                    let i: usize = words.iter().position(|w| *w == "|").unwrap();
                    (
                        rule_number,
                        Rule::Branch(
                            words[1..i]
                                .iter()
                                .take_while(|c| **c != "|")
                                .map(|c| c.parse().unwrap())
                                .collect(),
                            words[i + 1..]
                                .iter()
                                .take_while(|c| **c != "|")
                                .map(|c| c.parse().unwrap())
                                .collect(),
                        ),
                    )
                } else {
                    (
                        rule_number,
                        Rule::NonTerminal(words[1..].iter().map(|c| c.parse().unwrap()).collect()),
                    )
                }
            })),
        }
    }
}

impl Grammar {
    fn verify<'a>(&self, chars: &'a [char], rule: i32) -> Vec<&'a [char]> {
        if chars.len() == 0 {
            return vec![];
        }
        match self.rules.get(&rule) {
            Some(Rule::Terminal(t)) if chars[0] == *t => vec![&chars[1..]],
            Some(Rule::NonTerminal(nt)) => nt.iter().fold(vec![&chars], |v, r| {
                v.iter()
                    .map(|chars| self.verify(&chars, *r))
                    .flatten()
                    .collect()
            }),
            Some(Rule::Branch(b1, b2)) => {
                let mut l: Vec<&[char]> = b1.iter().fold(vec![&chars], |v, r| {
                    v.iter()
                        .map(|chars| self.verify(chars, *r))
                        .flatten()
                        .collect()
                });
                let mut r: Vec<&[char]> = b2.iter().fold(vec![&chars], |v, r| {
                    v.iter()
                        .map(|chars| self.verify(chars, *r))
                        .flatten()
                        .collect()
                });
                l.append(&mut r);
                l
            }
            _ => vec![],
        }
    }
}

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let mut input = self.data.split("\r\n\r\n");
        let grammar: Grammar = Grammar::from(input.next().unwrap());
        Ok(input
            .next()
            .unwrap()
            .lines()
            .filter(|l| {
                grammar
                    .verify(&l.chars().collect::<Vec<char>>()[..], 0)
                    .iter()
                    .any(|r| r.len() == 0)
            })
            .count()
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let mut input = self.data.split("\r\n\r\n");
        let mut grammar: Grammar = Grammar::from(input.next().unwrap());
        grammar.rules.insert(8, Rule::Branch(vec![42, 8], vec![42]));
        grammar.rules.insert(11, Rule::Branch(vec![42, 11, 31], vec![42, 31]));
        Ok(input
            .next()
            .unwrap()
            .lines()
            .filter(|l| {
                grammar
                    .verify(&l.chars().collect::<Vec<char>>()[..], 0)
                    .iter()
                    .any(|r| r.len() == 0)
            })
            .count()
            .to_string())
    }
}
