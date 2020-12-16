use crate::{Fail, Input, Solution};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub static DATA_PATH: &str = "data/day16.txt";
pub static TEST_PATH: &str = "data/test/day16.txt";
pub static TEST_VALUES: (&str, &str) = ("250", "1716");

type Range = ((i32, i32), (i32, i32));

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let mut input = self.data.split("\r\n\r\n");
        let spec: Vec<&str> = parse_next(input.next());
        input.next();
        let nearby: Vec<&str> = parse_next(input.next());
        let ranges: Vec<Range> = spec.iter().map(|s| parse_spec(drop_header(*s))).collect();
        Ok(nearby[1..]
            .iter()
            .map(|s| {
                parse_ticket(s)
                    .iter()
                    .filter(|n| !valid(**n, &ranges))
                    .sum::<i32>()
            })
            .sum::<i32>()
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let mut input = self.data.split("\r\n\r\n");
        let spec: Vec<&str> = parse_next(input.next());
        let ticket: Vec<i32> = parse_ticket(parse_next(input.next())[1]);
        let nearby: Vec<&str> = parse_next(input.next());
        let headers: Vec<&str> = spec.iter().map(|s| parse_header(*s)).collect();
        let ranges: Vec<Range> = spec.iter().map(|s| parse_spec(drop_header(*s))).collect();
        let valid_tickets: Vec<Vec<i32>> = nearby[1..]
            .iter()
            .map(|s| parse_ticket(s))
            .filter(|v| v.iter().all(|n| valid(*n, &ranges)))
            .collect();
        let cols: usize = headers.len();
        let mut ordered: Vec<(&str, Vec<usize>)> = vec![];
        let mut attributes: HashMap<&str, usize> = HashMap::new();
        let mut available: HashSet<usize> = HashSet::from_iter(0..cols);
        ranges.iter().zip(headers).for_each(|(r, h)| {
            ordered.push((
                h,
                (0..cols)
                    .filter(|i| valid_tickets.iter().all(|v| in_range(v[*i], *r)))
                    .collect(),
            ));
        });
        ordered.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
        ordered.iter().for_each(|(h, v)| {
            let n: usize = *v.iter().filter(|n| available.contains(n)).next().unwrap();
            attributes.insert(h, n);
            available.remove(&n);
        });
        Ok(attributes
            .iter()
            .filter(|(h, _)| h.contains("departure"))
            .map(|(_, i)| ticket[*i] as i64)
            .product::<i64>()
            .to_string())
    }
}

fn parse_next(o: Option<&str>) -> Vec<&str> {
    o.unwrap().split("\r\n").collect()
}

fn parse_spec(s: &str) -> Range {
    let ranges: Vec<(i32, i32)> = s
        .split("or")
        .map(|r| {
            let mut i = r.split("-");
            (
                i.next().and_then(|i| i.trim().parse().ok()).unwrap(),
                i.next().and_then(|i| i.trim().parse().ok()).unwrap(),
            )
        })
        .collect();
    (ranges[0], ranges[1])
}

fn parse_ticket(s: &str) -> Vec<i32> {
    s.split(",").filter_map(|i| i.trim().parse().ok()).collect()
}

fn parse_header(s: &str) -> &str {
    s.split(":").nth(0).unwrap()
}

fn drop_header(s: &str) -> &str {
    s.split(":").nth(1).unwrap()
}

fn valid(n: i32, ranges: &Vec<Range>) -> bool {
    ranges
        .iter()
        .any(|(r1, r2)| ((n >= r1.0) & (n <= r1.1)) | ((n >= r2.0) & (n <= r2.1)))
}

fn in_range(n: i32, range: Range) -> bool {
    ((n >= range.0 .0) & (n <= range.0 .1)) | ((n >= range.1 .0) & (n <= range.1 .1))
}
