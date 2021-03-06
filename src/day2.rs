use crate::{Fail, Input, Solution};

pub static DATA_PATH: &str = "data/day2.txt";
pub static TEST_PATH: &str = "data/test/day2.txt";
pub static TEST_VALUES: (&str, &str) = ("2", "1");

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<Vec<&str>> = self.data.lines().map(|x| x.split(" ").collect()).collect();
        Ok(input
            .iter()
            .filter(|y| parse_password_1(y.to_vec()).unwrap())
            .count()
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<Vec<&str>> = self.data.lines().map(|x| x.split(" ").collect()).collect();
        Ok(input
            .iter()
            .filter(|y| parse_password_2(y.to_vec()).unwrap())
            .count()
            .to_string())
    }
}

fn parse_password_1(input: Vec<&str>) -> Option<bool> {
    let range: Vec<i32> = input[0].split("-").map(|x| x.parse().unwrap()).collect();
    valid_password_1(range[0], range[1], input[1].chars().next()?, input[2])
}

fn valid_password_1(min: i32, max: i32, letter: char, pass: &str) -> Option<bool> {
    let len: i32 = pass.chars().filter(|c| c == &letter).count() as i32;
    Some(len >= min && len <= max)
}

fn parse_password_2(input: Vec<&str>) -> Option<bool> {
    let range: Vec<usize> = input[0].split("-").map(|x| x.parse().unwrap()).collect();
    valid_password_2(range[0], range[1], input[1].chars().next()?, input[2])
}

fn valid_password_2(i_1: usize, i_2: usize, letter: char, pass: &str) -> Option<bool> {
    let chars: Vec<char> = pass.chars().collect();
    Some((chars[i_1 - 1] == letter) ^ (chars[i_2 - 1] == letter))
}
