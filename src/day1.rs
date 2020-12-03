use crate::{Fail, Input, Solution};

pub static DATA_PATH: &str = "data/day1.txt";
pub static TEST_PATH: &str = "data/test/day1.txt";
pub static TEST_VALUES: (&str, &str) = ("514579", "241861950");

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let mut input: Vec<i32> = self.into();
        input.sort();
        match find_pair(&input, 2020) {
            Some((x, y)) => Ok((x * y).to_string()),
            None => Err("Failed to find a pair".into())
        }
    }

    fn part2(&self) -> Result<String, Fail> {
        let mut input: Vec<i32> = self.into();
        input.sort();
        Ok(input
            .iter()
            .fold(Err("Failed to find a triple"), |b, i| {
                match (b, find_pair(&input, 2020 - i)) {
                    (Err(_), Some((j, k))) => Ok(i * j * k),
                    (b, _) => b,
                }
            })?
            .to_string())
    }
}

fn find_pair(list: &[i32], sum: i32) -> Option<(i32, i32)> {
    if list.len() <= 1 {
        return None;
    }
    match list[0] + list[list.len() - 1] - sum {
        0 => Some((list[0], list[list.len() - 1])),
        p if p > 0 => find_pair(&list[0..list.len() - 1], sum),
        _ => find_pair(&list[1..list.len()], sum),
    }
}
