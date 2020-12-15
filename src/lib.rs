use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub mod day15;

pub type Fail = Box<dyn std::error::Error>;

pub struct Input {
    data: String,
}

impl From<&str> for Input {
    fn from(file: &str) -> Input {
        Input {
            data: fs::read_to_string(file).expect(&format!("Unable to read file {}", file)),
        }
    }
}

impl From<&Input> for Vec<i32> {
    fn from(input: &Input) -> Vec<i32> {
        input
            .data
            .lines()
            .map(|x| x.parse::<i32>().expect(&format!("Unable to parse {}", x)))
            .collect()
    }
}

impl From<&Input> for Vec<i64> {
    fn from(input: &Input) -> Vec<i64> {
        input
            .data
            .lines()
            .map(|x| x.parse::<i64>().expect(&format!("Unable to parse {}", x)))
            .collect()
    }
}

impl From<&Input> for Vec<Vec<char>> {
    fn from(input: &Input) -> Vec<Vec<char>> {
        input.data.lines().map(|x| x.chars().collect()).collect()
    }
}

impl From<&Input> for HashSet<i32> {
    fn from(input: &Input) -> HashSet<i32> {
        input
            .data
            .lines()
            .map(|x| x.parse::<i32>().expect(&format!("Unable to parse {}", x)))
            .collect()
    }
}

impl From<&Input> for HashMap<(i32, i32), char> {
    fn from(input: &Input) -> HashMap<(i32, i32), char> {
        input
            .data
            .lines()
            .enumerate()
            .map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i32, y as i32), c))
            })
            .flatten()
            .collect()
    }
}

pub trait Solution {
    fn part1(&self) -> Result<String, Fail>;
    fn part2(&self) -> Result<String, Fail>;
}
