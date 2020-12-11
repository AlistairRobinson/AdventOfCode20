use crate::{Fail, Input, Solution};
use std::collections::HashMap;

pub static DATA_PATH: &str = "data/day11.txt";
pub static TEST_PATH: &str = "data/test/day11.txt";
pub static TEST_VALUES: (&str, &str) = ("37", "26");

pub type Point = (i32, i32);

#[derive(Clone, Debug)]
pub struct SpaceOne {
    map: HashMap<Point, char>,
    taken: usize,
}

#[derive(Clone, Debug)]
pub struct SpaceTwo {
    map: HashMap<Point, char>,
    taken: usize,
}

impl Iterator for SpaceOne {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let state: SpaceOne = self.clone();
        let empty: Vec<Point> = state.empty();
        let taken: Vec<Point> = state.taken();
        if empty.len() == 0 && taken.len() == 0 {
            None
        } else {
            self.taken = state.taken + empty.iter().map(|e| self.map.insert(*e, '#')).count()
                - taken.iter().map(|o| self.map.insert(*o, 'L')).count();
            Some(self.taken)
        }
    }
}

impl SpaceOne {
    fn new(map: HashMap<Point, char>) -> SpaceOne {
        SpaceOne { map: map, taken: 0 }
    }

    fn empty(&self) -> Vec<Point> {
        self.map
            .keys()
            .filter(|p| self.map.get(p) == Some(&'L') && self.adj(**p) == 0)
            .map(|p| *p)
            .collect()
    }

    fn taken(&self) -> Vec<Point> {
        self.map
            .keys()
            .filter(|p| self.map.get(p) == Some(&'#') && self.adj(**p) >= 4)
            .map(|p| *p)
            .collect()
    }

    fn adj(&self, (x, y): Point) -> i32 {
        (-1..2)
            .map(|i| {
                (-1..2).map(move |j| match self.map.get(&(x + i, y + j)) {
                    Some('#') if (i, j) != (0, 0) => 1,
                    _ => 0,
                })
            })
            .flatten()
            .sum()
    }
}

impl Iterator for SpaceTwo {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let state: SpaceTwo = self.clone();
        let empty: Vec<Point> = state.empty();
        let taken: Vec<Point> = state.taken();
        if empty.len() == 0 && taken.len() == 0 {
            None
        } else {
            self.taken = state.taken + empty.iter().map(|e| self.map.insert(*e, '#')).count()
                - taken.iter().map(|o| self.map.insert(*o, 'L')).count();
            Some(self.taken)
        }
    }
}

impl SpaceTwo {
    fn new(map: HashMap<Point, char>) -> SpaceTwo {
        SpaceTwo { map: map, taken: 0 }
    }

    fn empty(&self) -> Vec<Point> {
        self.map
            .keys()
            .filter(|p| self.map.get(p) == Some(&'L') && self.visible(**p) == 0)
            .map(|p| *p)
            .collect()
    }

    fn taken(&self) -> Vec<Point> {
        self.map
            .keys()
            .filter(|p| self.map.get(p) == Some(&'#') && self.visible(**p) >= 5)
            .map(|p| *p)
            .collect()
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
                })
            })
            .flatten()
            .sum()
    }
}

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: HashMap<Point, char> = self.into();
        let space: SpaceOne = SpaceOne::new(input);
        Ok(space
            .into_iter()
            .last()
            .ok_or("No states found")?
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: HashMap<Point, char> = self.into();
        let space: SpaceTwo = SpaceTwo::new(input);
        Ok(space
            .into_iter()
            .last()
            .ok_or("No states found")?
            .to_string())
    }
}
