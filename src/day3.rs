use crate::{Fail, Input, Solution};

pub static DATA_PATH: &str = "data/day3.txt";
pub static TEST_PATH: &str = "data/test/day3.txt";
pub static TEST_VALUES: (&str, &str) = ("7", "336");

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<Vec<char>> = self.into();
        Ok((0..input.len())
            .filter(|n| input[*n][(3 * n) % input[*n].len()] == '#')
            .count()
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<Vec<char>> = self.into();
        let slopes: Vec<fn(usize) -> (usize, usize)> = vec![
            { |n| (n, n) },
            { |n| (n, 3 * n) },
            { |n| (n, 5 * n) },
            { |n| (n, 7 * n) },
            { |n| (2 * n, n) },
        ];
        Ok(slopes
            .iter()
            .map(|f| {
                (0..input.len())
                    .filter(|n| {
                        let (x, y) = f(*n);
                        x < input.len() && input[x][y % input[x].len()] == '#'
                    })
                    .count()
            })
            .product::<usize>()
            .to_string())
    }
}
