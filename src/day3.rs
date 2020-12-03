use crate::{Fail, Input, Solution};

pub static DATA_PATH: &str = "data/day3.txt";
pub static TEST_PATH: &str = "data/test/day3.txt";
pub static TEST_VALUES: (&str, &str) = ("7", "336");

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<Vec<char>> = self.data.lines().map(|x| x.chars().collect()).collect();
        Ok((0..input.len())
            .filter(|n| input[*n][(3 * n) % input[*n].len()] == '#')
            .collect::<Vec<_>>()
            .len()
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<Vec<char>> = self.data.lines().map(|x| x.chars().collect()).collect();
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
                    .collect::<Vec<_>>()
                    .len()
            })
            .product::<usize>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() -> Result<(), Fail> {
        let input = Input::from(TEST_PATH);
        assert!(input.part1()? == TEST_VALUES.0, input.part1()?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Fail> {
        let input = Input::from(TEST_PATH);
        assert!(input.part2()? == TEST_VALUES.1, input.part2()?);
        Ok(())
    }
}
