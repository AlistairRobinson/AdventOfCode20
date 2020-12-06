use crate::{Fail, Input, Solution};
use std::collections::HashSet;
use std::iter::FromIterator;

pub static DATA_PATH: &str = "data/day6.txt";
pub static TEST_PATH: &str = "data/test/day6.txt";
pub static TEST_VALUES: (&str, &str) = ("11", "6");

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<&str> = self.data.split("\r\n\r\n").collect();
        let responses: Vec<HashSet<char>> = input
            .iter()
            .map(|s| HashSet::from_iter(s.replace("\r\n", "").chars()))
            .collect();
        Ok(responses.iter().map(|r| r.len()).sum::<usize>().to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<&str> = self.data.split("\r\n\r\n").collect();
        let responses: Vec<HashSet<char>> = input
            .iter()
            .map(|s| HashSet::from_iter(s.chars().filter(|c| s.lines().all(|t| t.contains(*c)))))
            .collect();
        Ok(responses.iter().map(|r| r.len()).sum::<usize>().to_string())
    }
}
