use crate::Fail;
use std::fs;

pub struct Solution {
    data: String,
}

impl Solution {
    pub fn new() -> Solution {
        let file: String = "data/day0.txt".to_string();
        Solution {
            data: fs::read_to_string(&file).expect(&format!("Unable to read file {}", file)),
        }
    }

    pub fn part1(&self) -> Result<String, Fail> {
        Ok(self
            .data
            .lines()
            .map(|x| {
                x.parse::<i32>()
                    .expect(&format!("Unable to parse value {}", x))
            })
            .fold(0, fuel_1)
            .to_string())
    }

    pub fn part2(&self) -> Result<String, Fail> {
        Ok(self
            .data
            .lines()
            .map(|x| {
                x.parse::<i32>()
                    .expect(&format!("Unable to parse value {}", x))
            })
            .fold(0, fuel_2)
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

fn fuel_1(base: i32, weight: i32) -> i32 {
    base + (weight / 3 - 2)
}

fn fuel_2(base: i32, weight: i32) -> i32 {
    let fuel = weight / 3 - 2;
    if fuel > 0 {
        fuel_2(base + fuel, fuel)
    } else {
        base
    }
}
