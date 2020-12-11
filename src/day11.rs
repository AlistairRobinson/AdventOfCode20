use crate::{Fail, Input, Solution};
use std::collections::HashMap;

pub static DATA_PATH: &str = "data/day11.txt";
pub static TEST_PATH: &str = "data/test/day11.txt";
pub static TEST_VALUES: (&str, &str) = ("37", "26");

pub type Point = (i32, i32);

#[derive(Clone, Debug)]
pub struct SpaceOne {
    map: HashMap<Point, char>,
    mutable: Vec<Point>,
    taken: i32,
}

#[derive(Clone, Debug)]
pub struct SpaceTwo {
    map: HashMap<Point, char>,
    mutable: Vec<Point>,
    taken: i32,
}

impl Iterator for SpaceOne {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let state: SpaceOne = self.clone();
        let taken: i32 = state.mutable
            .iter()
            .filter(|p| state.map.get(p) == Some(&'L') && state.adj(**p) == 0)
            .map(|e| self.map.insert(*e, '#'))
            .count() as i32;
        let empty: i32 = state.mutable
            .iter()
            .filter(|p| state.map.get(p) == Some(&'#') && state.adj(**p) >= 4)
            .map(|o| self.map.insert(*o, 'L'))
            .count() as i32;
        if taken == 0 && empty == 0 {
            None
        } else {
            self.taken = state.taken + taken - empty;
            Some(self.taken)
        }
    }
}

impl SpaceOne {
    fn new(map: HashMap<Point, char>) -> SpaceOne {
        SpaceOne {
            map: map.clone(),
            mutable: map
                .keys()
                .filter(|p| map.get(p) != Some(&'.'))
                .map(|p| *p)
                .collect(),
            taken: 0,
        }
    }

    fn adj(&self, (x, y): Point) -> i32 {
        (-1..2)
            .map(|i| {
                (-1..2).map(move |j| match self.map.get(&(x + i, y + j)) {
                    Some('#') if (i, j) != (0, 0) => 1,
                    _ => 0,
                }).sum::<i32>()
            })
            .sum()
    }
}

impl Iterator for SpaceTwo {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let state: SpaceTwo = self.clone();
        let taken: i32 = state.mutable
            .iter()
            .filter(|p| state.map.get(p) == Some(&'L') && state.visible(**p) == 0)
            .map(|e| self.map.insert(*e, '#'))
            .count() as i32;
        let empty: i32 = state.mutable
            .iter()
            .filter(|p| state.map.get(p) == Some(&'#') && state.visible(**p) >= 5)
            .map(|o| self.map.insert(*o, 'L'))
            .count() as i32;
        if taken == 0 && empty == 0 {
            None
        } else {
            self.taken = state.taken + taken - empty;
            Some(self.taken)
        }
    }
}

impl SpaceTwo {
    fn new(map: HashMap<Point, char>) -> SpaceTwo {
        SpaceTwo {
            map: map.clone(),
            mutable: map
                .keys()
                .filter(|p| map.get(p) != Some(&'.'))
                .map(|p| *p)
                .collect(),
            taken: 0,
        }
    }

    fn visible(&self, (x, y): Point) -> i32 {
        (-1..2)
            .map(|i| {
                (-1..2).map(move |j| {
                    let n = (1..)
                        .skip_while(|n| self.map.get(&(x + n * i, y + n * j)) == Some(&'.'))
                        .next()
                        .unwrap();
                    ((i, j) != (0, 0) && self.map.get(&(x + n * i, y + n * j)) == Some(&'#')) as i32
                }).sum::<i32>()
            })
            .sum()
    }
}

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: HashMap<Point, char> = self.into();
        Ok(SpaceOne::new(input)
            .into_iter()
            .last()
            .ok_or("No states found")?
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: HashMap<Point, char> = self.into();
        Ok(SpaceTwo::new(input)
            .into_iter()
            .last()
            .ok_or("No states found")?
            .to_string())
    }
}
