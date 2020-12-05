use crate::{Fail, Input, Solution};

pub static DATA_PATH: &str = "data/day5.txt";
pub static TEST_PATH: &str = "data/test/day5.txt";
pub static TEST_VALUES: (&str, &str) = ("820", "-1");

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<Vec<char>> = self.into();
        Ok(input
            .iter()
            .map(|v| row(&v[..7]) * 8 + col(&v[7..]))
            .max()
            .unwrap()
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<Vec<char>> = self.into();
        let mut ids: Vec<i32> = input
            .iter()
            .map(|v| row(&v[..7]) * 8 + col(&v[7..]))
            .collect();
        ids.sort();
        Ok(ids
            .iter()
            .enumerate()
            .fold(None, |b, (i, v)| match b {
                Some(_) => b,
                None if i < ids.len() - 1 && ids[i + 1] == v + 2 => Some(v + 1),
                None => None,
            })
            .unwrap_or(-1)
            .to_string())
    }
}

fn row(r: &[char]) -> i32 {
    r.iter().zip((0..7).rev()).fold(0, |b, c| match c {
        ('B', i) => b + i32::pow(2, i),
        _ => b,
    })
}

fn col(c: &[char]) -> i32 {
    c.iter().zip((0..3).rev()).fold(0, |b, c| match c {
        ('R', i) => b + i32::pow(2, i),
        _ => b,
    })
}
