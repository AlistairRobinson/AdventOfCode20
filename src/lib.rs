use std::fs;

pub mod day1;

pub type Fail = Box<dyn std::error::Error>;

pub struct Solution {
    data: String,
}

impl Solution {
    pub fn new() -> Solution {
        let file: String = "data/default.txt".to_string();
        Solution {
            data: fs::read_to_string(&file).expect(&format!("Unable to read file {}", file)),
        }
    }

    pub fn part1(&self) -> Result<String, Fail> {
        Ok(String::from(&self.data))
    }

    pub fn part2(&self) -> Result<String, Fail> {
        Ok(String::from(&self.data))
    }
}

impl From<&str> for Solution {
    fn from(file: &str) -> Solution {
        Solution {
            data: fs::read_to_string(file).expect(&format!("Unable to read file {}", file)),
        }
    }
}
