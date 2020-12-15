use crate::{Fail, Input, Solution};
use std::collections::HashMap;
use std::iter::FromIterator;

pub static DATA_PATH: &str = "data/day15.txt";
pub static TEST_PATH: &str = "data/test/day15.txt";
pub static TEST_VALUES: (&str, &str) = ("436", "436");

#[derive(Debug)]
pub struct Game {
    counter: usize,
    last: usize,
    history: HashMap<usize, usize>,
}

impl Game {
    fn new(start: Vec<usize>) -> Game {
        Game {
            counter: start.len(),
            history: HashMap::from_iter(start.iter().zip(1..start.len()).map(|(x, i)| (*x, i))),
            last: start[start.len() - 1],
        }
    }
}

impl Iterator for Game {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self.history.insert(self.last, self.counter) {
            Some(i) => self.last = self.counter - i,
            None => self.last = 0,
        }
        self.counter += 1;
        Some(self.last)
    }
}

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<usize> = self
            .data
            .split(",")
            .map(|x| x.parse::<usize>().expect(&format!("Unable to parse {}", x)))
            .collect();
        let length = input.len() + 1;
        Ok(Game::new(input).into_iter().nth(2020 - length).ok_or("Not found")?.to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<usize> = self
            .data
            .split(",")
            .map(|x| x.parse::<usize>().expect(&format!("Unable to parse {}", x)))
            .collect();
        let length = input.len() + 1;
        Ok(Game::new(input).into_iter().nth(30000000 - length).ok_or("Not found")?.to_string())
    }
}
