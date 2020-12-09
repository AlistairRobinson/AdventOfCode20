use crate::{Fail, Input, Solution};

pub static DATA_PATH: &str = "data/day9.txt";
pub static TEST_PATH: &str = "data/test/day9.txt";
pub static TEST_VALUES: (&str, &str) = ("127", "62");

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<i64> = self.into();
        Ok(input
            .iter()
            .enumerate()
            .skip_while(|(i, x)| i < &25 || valid(input[i - 25..*i].to_vec(), **x).is_some())
            .next()
            .ok_or("No invalid entry found")?
            .1
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<i64> = self.into();
        let target: i64 = self.part1()?.parse::<i64>()?;
        Ok(find_subseq(&input, 0, 0, target)
            .ok_or("No subsequence found")?
            .to_string())
    }
}

fn valid(mut list: Vec<i64>, sum: i64) -> Option<(i64, i64)> {
    list.sort();
    find_pair(&list, sum)
}

fn find_pair(list: &[i64], sum: i64) -> Option<(i64, i64)> {
    if list.len() <= 1 {
        return None;
    }
    match list[0] + list[list.len() - 1] - sum {
        0 => Some((list[0], list[list.len() - 1])),
        p if p > 0 => find_pair(&list[0..list.len() - 1], sum),
        _ => find_pair(&list[1..list.len()], sum),
    }
}

fn find_subseq(list: &[i64], min: usize, max: usize, sum: i64) -> Option<i64> {
    if max > list.len() || min > max {
        return None;
    }
    let subseq: &[i64] = &list[min..max + 1];
    match subseq.iter().sum::<i64>() - sum {
        0 => Some(subseq.iter().max()? + subseq.iter().min()?),
        p if p > 0 => find_subseq(list, min + 1, max, sum),
        _ => find_subseq(list, min, max + 1, sum),
    }
}
