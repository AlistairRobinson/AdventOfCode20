use crate::Fail;
use std::fs;

pub struct Solution {
    data: String,
}

impl Solution {
    pub fn new() -> Solution {
        let file: String = "data/day1.txt".to_string();
        Solution {
            data: fs::read_to_string(&file).expect(&format!("Unable to read file {}", file)),
        }
    }

    pub fn part1(&self) -> Result<String, Fail> {
        let mut input = self
            .data
            .lines()
            .map(|x| {
                x.parse::<i32>()
                    .expect(&format!("Unable to parse value {}", x))
            })
            .collect::<Vec<i32>>();
        input.sort();
        let (n1, n2) = find_pair(&input, 2020).ok_or("Failed to find a pair")?;
        Ok((n1 * n2).to_string())
    }

    pub fn part2(&self) -> Result<String, Fail> {
        let mut input = self
            .data
            .lines()
            .map(|x| {
                x.parse::<i32>()
                    .expect(&format!("Unable to parse value {}", x))
            })
            .collect::<Vec<i32>>();
        input.sort();
        Ok((input
            .iter()
            .fold(0, |b, i| match (b, find_pair(&input, 2020 - i)) {
                (0, Some((j, k))) => i * j * k,
                (0, None) => 0,
                (b, _) => b,
            }))
        .to_string())
    }
}

impl From<&str> for Solution {
    fn from(file: &str) -> Solution {
        Solution {
            data: fs::read_to_string(file).expect(&format!("Unable to read file {}", file)),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() -> Result<(), Fail> {
        let solution = Solution::from("data/test/day1.txt");
        assert!(solution.part1()? == "514579", solution.part1()?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Fail> {
        let solution = Solution::from("data/test/day1.txt");
        assert!(solution.part2()? == "241861950", solution.part2()?);
        Ok(())
    }
}