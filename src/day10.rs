use crate::{Fail, Input, Solution};
use std::cmp::max;
use std::collections::HashMap;

pub static DATA_PATH: &str = "data/day10.txt";
pub static TEST_PATH: &str = "data/test/day10.txt";
pub static TEST_VALUES: (&str, &str) = ("220", "19208");

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let mut input: Vec<i32> = self.into();
        input.sort();
        let (ones, threes, _) = input.iter().fold((0, 0, 0), |(o, t, v), x| match x - v {
            1 => (o + 1, t, *x),
            2 => (o, t, *x),
            3 => (o, t + 1, *x),
            _ => (o, t, v),
        });
        Ok((ones * (threes + 1)).to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let mut input: Vec<i64> = self.into();
        let mut combinations: HashMap<i64, i64> = HashMap::new();
        input.sort();
        Ok(input
            .iter()
            .rev()
            .map(|a| {
                let routes: i64 = (a + 1..a + 4)
                    .map(|i| *combinations.get(&i).unwrap_or(&&0))
                    .sum();
                combinations.insert(*a, max(routes, 1));
                routes
            })
            .last()
            .ok_or("No combinations found")?
            .to_string())
    }
}
