use crate::{Fail, Input, Solution};

pub static DATA_PATH: &str = "data/day4.txt";
pub static TEST_PATH: &str = "data/test/day4.txt";
pub static TEST_VALUES: (&str, &str) = ("2", "2");

pub static KEYS: [&str; 7] = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<&str> = self.data.split("\r\n\r\n").collect();
        Ok(input
            .iter()
            .filter(|x| KEYS.iter().all(|y| x.contains(y)))
            .count()
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<&str> = self.data.split("\r\n\r\n").collect();
        Ok(input
            .iter()
            .filter(|x| {
                KEYS.iter().all(|y| x.contains(y)) && x.replace("\r\n", " ").split(" ").all(valid)
            })
            .count()
            .to_string())
    }
}

fn valid(input: &str) -> bool {
    let mut i = input.split(":");
    let (prefix, s) = (i.next().unwrap(), i.next().unwrap());
    match prefix {
        "byr" => s.parse::<i32>().is_ok() && (1920..2003).contains(&s.parse::<i32>().unwrap()),
        "iyr" => s.parse::<i32>().is_ok() && (2010..2021).contains(&s.parse::<i32>().unwrap()),
        "eyr" => s.parse::<i32>().is_ok() && (2020..2031).contains(&s.parse::<i32>().unwrap()),
        "hgt" => {
            if s.ends_with("cm") {
                let h = s.strip_suffix("cm").unwrap();
                h.parse::<i32>().is_ok() && (150..194).contains(&h.parse::<i32>().unwrap())
            } else if s.ends_with("in") {
                let h = s.strip_suffix("in").unwrap();
                h.parse::<i32>().is_ok() && (59..77).contains(&h.parse::<i32>().unwrap())
            } else {
                false
            }
        }
        "hcl" => {
            s.starts_with("#")
                && s.len() == 7
                && s.strip_prefix("#")
                    .unwrap()
                    .chars()
                    .all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s),
        "pid" => s.len() == 9 && s.parse::<i32>().is_ok(),
        _ => true,
    }
}
