use std::fs;

pub mod day5;

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
            .collect::<Vec<i32>>()
    }
}

impl From<&Input> for Vec<Vec<char>> {
    fn from(input: &Input) -> Vec<Vec<char>> {
        input.data.lines().map(|x| x.chars().collect()).collect()
    }
}

pub trait Solution {
    fn part1(&self) -> Result<String, Fail>;
    fn part2(&self) -> Result<String, Fail>;
}
