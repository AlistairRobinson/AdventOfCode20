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
    let i: Vec<&str> = input.split(":").collect();
    let (prefix, s) = (i[0], i[1]);
    match prefix {
        "byr" => in_range(s, 1920, 2002),
        "iyr" => in_range(s, 2010, 2020),
        "eyr" => in_range(s, 2020, 2030),
        "hgt" => match &s[s.len() - 2..s.len()] {
            "cm" => in_range(&s[..s.len() - 2], 150, 193),
            "in" => in_range(&s[..s.len() - 2], 59, 76),
            _ => false,
        },
        "hcl" => {
            &s[0..1] == "#" && s.len() == 7 && s[1..].chars().all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s),
        "pid" => s.len() == 9 && in_range(s, 0, 999999999),
        _ => true,
    }
}

fn in_range(n: &str, min: i32, max: i32) -> bool {
    n.parse::<i32>().is_ok() && (min..max + 1).contains(&n.parse::<i32>().unwrap())
}
