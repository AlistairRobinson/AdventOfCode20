use crate::{Fail, Input, Solution};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub static DATA_PATH: &str = "data/day7.txt";
pub static TEST_PATH: &str = "data/test/day7.txt";
pub static TEST_VALUES: (&str, &str) = ("4", "32");

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<((String, String), i32)> =
            self.data.lines().map(get_edges).flatten().collect();
        let graph: HashMap<(String, String), i32> = HashMap::from_iter(input);
        Ok(find("shiny gold bag".to_string(), graph).len().to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<((String, String), i32)> =
            self.data.lines().map(get_edges).flatten().collect();
        let graph: HashMap<(String, String), i32> = HashMap::from_iter(input);
        Ok((total("shiny gold bag".to_string(), 1, graph) - 1).to_string())
    }
}

fn get_edges(line: &str) -> Vec<((String, String), i32)> {
    let clean = line.replace("bags", "bag").replace(".", "");
    let parts: Vec<&str> = clean.split(" contain ").collect();
    parts[1]
        .split(", ")
        .map(|s| ((&parts[0], &s[2..]), s[..1].parse().unwrap_or(0)))
        .map(|((x, y), n)| ((x.to_string(), y.to_string()), n))
        .collect()
}

fn find(target: String, map: HashMap<(String, String), i32>) -> HashSet<String> {
    let nodes: Vec<String> = map
        .keys()
        .filter(|(_, y)| y == &target)
        .map(|(x, _)| x.clone())
        .collect();
    nodes
        .iter()
        .map(|n| find(n.to_string(), map.clone()))
        .fold(HashSet::from_iter(nodes.clone()), |b, i| &b | &i)
}

fn total(bag: String, quantity: i32, map: HashMap<(String, String), i32>) -> i32 {
    let within: i32 = map
        .keys()
        .filter(|(x, _)| x == &bag)
        .map(|(_, y)| y.clone().to_string())
        .map(|n| total(n.clone(), *map.get(&(bag.clone(), n)).unwrap(), map.clone()))
        .sum::<i32>();
    quantity + quantity * within
}
