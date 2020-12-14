use crate::{Fail, Input, Solution};
use std::collections::HashMap;

pub static DATA_PATH: &str = "data/day14.txt";
pub static TEST_PATH: &str = "data/test/day14.txt";
pub static TEST_VALUES: (&str, &str) = ("51", "208");

type Memory = HashMap<i64, Vec<bool>>;

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<&str> = self.data.lines().collect();
        let mut memory: Memory = Memory::new();
        input.iter().fold(vec![], |m, l| {
            let args: Vec<&str> = l.split(" = ").collect();
            match args[0] {
                "mask" => args[1].chars().collect(),
                _ => {
                    memory.insert(digits(args[0]), mask(bin(digits(args[1])), &m));
                    m
                }
            }
        });
        Ok(memory
            .iter()
            .fold(0, |b, (_, v)| b + dec(v.to_vec()))
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<&str> = self.data.lines().collect();
        let mut memory: Memory = Memory::new();
        input.iter().fold(vec![], |m, l| {
            let args: Vec<&str> = l.split(" = ").collect();
            match args[0] {
                "mask" => args[1].chars().collect(),
                _ => {
                    f_dec(f_mask(bin(digits(args[0])), &m))
                        .iter()
                        .for_each(|r| {
                            memory.insert(*r, bin(digits(args[1])));
                        });
                    m
                }
            }
        });
        Ok(memory
            .iter()
            .fold(0, |b, (_, v)| b + dec(v.to_vec()))
            .to_string())
    }
}

fn bin(n: i64) -> Vec<bool> {
    let mut v: Vec<bool> = vec![];
    (0..36).rev().fold(n, |b, i| {
        v.push(b >= i64::pow(2, i));
        match b >= i64::pow(2, i) {
            true => b - i64::pow(2, i),
            _ => b,
        }
    });
    v
}

fn dec(v: Vec<bool>) -> i64 {
    v.iter().zip((0..36).rev()).fold(0, |b, c| match c {
        (true, i) => b + i64::pow(2, i),
        _ => b,
    })
}

fn mask(v: Vec<bool>, m: &Vec<char>) -> Vec<bool> {
    v.iter()
        .zip(m.iter())
        .map(|(x, y)| match y {
            'X' => *x,
            '0' => false,
            '1' => true,
            _ => panic!("Unknown element in mask"),
        })
        .collect()
}

fn f_mask(v: Vec<bool>, m: &Vec<char>) -> Vec<Option<bool>> {
    v.iter()
        .zip(m.iter())
        .map(|(x, y)| match y {
            'X' => None,
            '0' => Some(*x),
            '1' => Some(true),
            _ => panic!("Unknown element in mask"),
        })
        .collect()
}

fn f_dec(v: Vec<Option<bool>>) -> Vec<i64> {
    v.iter()
        .zip((0..36).rev())
        .fold(vec![0], |mut b, c| match c {
            (None, i) => {
                let mut temp = b.clone();
                b = b.iter().map(|n| n + i64::pow(2, i)).collect();
                b.append(&mut temp);
                b
            }
            (Some(true), i) => b.iter().map(|n| n + i64::pow(2, i)).collect(),
            _ => b,
        })
}

fn digits(s: &str) -> i64 {
    let f: String = s.chars().filter(|c| c.is_digit(10)).collect();
    f.parse::<i64>().expect("Unable to parse")
}
