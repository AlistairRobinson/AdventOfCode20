use crate::{Fail, Input, Solution};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub static DATA_PATH: &str = "data/day17.txt";
pub static TEST_PATH: &str = "data/test/day17.txt";
pub static TEST_VALUES: (&str, &str) = ("112", "848");

pub type Point = (i32, i32, i32);
pub type HyperPoint = (i32, i32, i32, i32);

#[derive(Debug)]
pub struct Space {
    map: HashMap<Point, char>,
    cubes: i32,
}

impl Iterator for Space {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let points: HashSet<Point> = self
            .map
            .iter()
            .map(|(k, _)| Space::neighbours(*k))
            .flatten()
            .collect();
        let invalid: HashSet<&Point> = points
            .iter()
            .filter(|p| {
                self.map.get(p) == Some(&'#') && ((self.adj(**p) <= 1) | (self.adj(**p) >= 4))
            })
            .collect();
        let valid: HashSet<&Point> = points
            .iter()
            .filter(|p| self.map.get(p) != Some(&'#') && (self.adj(**p) == 3))
            .collect();
        let removed: i32 = invalid.iter().map(|p| self.map.insert(**p, '.')).count() as i32;
        let filled: i32 = valid.iter().map(|p| self.map.insert(**p, '#')).count() as i32;
        if removed == 0 && filled == 0 {
            None
        } else {
            self.cubes = self.cubes + filled - removed;
            Some(self.cubes)
        }
    }
}

impl Space {
    fn new(map: HashMap<Point, char>) -> Space {
        Space {
            map: map.clone(),
            cubes: map.iter().filter(|(_, v)| **v == '#').count() as i32,
        }
    }

    fn neighbours((x, y, z): Point) -> impl Iterator<Item = Point> {
        (-1..2)
            .map(move |i| {
                (-1..2)
                    .map(move |j| {
                        (-1..2)
                            .filter(move |k| (i, j, *k) != (0, 0, 0))
                            .map(move |k| (x + i, y + j, z + k))
                    })
                    .flatten()
            })
            .flatten()
    }

    fn adj(&self, p: Point) -> i32 {
        Space::neighbours(p)
            .filter(|n| self.map.get(&n) == Some(&'#'))
            .count() as i32
    }
}

#[derive(Debug)]
pub struct HyperSpace {
    map: HashMap<HyperPoint, char>,
    cubes: i32,
}

impl Iterator for HyperSpace {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let points: HashSet<HyperPoint> = self
            .map
            .iter()
            .map(|(k, _)| HyperSpace::neighbours(*k))
            .flatten()
            .collect();
        let invalid: HashSet<&HyperPoint> = points
            .iter()
            .filter(|p| {
                self.map.get(p) == Some(&'#') && ((self.adj(**p) <= 1) | (self.adj(**p) >= 4))
            })
            .collect();
        let valid: HashSet<&HyperPoint> = points
            .iter()
            .filter(|p| self.map.get(p) != Some(&'#') && (self.adj(**p) == 3))
            .collect();
        let removed: i32 = invalid.iter().map(|p| self.map.insert(**p, '.')).count() as i32;
        let filled: i32 = valid.iter().map(|p| self.map.insert(**p, '#')).count() as i32;
        if removed == 0 && filled == 0 {
            None
        } else {
            self.cubes = self.cubes + filled - removed;
            Some(self.cubes)
        }
    }
}

impl HyperSpace {
    fn new(map: HashMap<HyperPoint, char>) -> HyperSpace {
        HyperSpace {
            map: map.clone(),
            cubes: map.iter().filter(|(_, v)| **v == '#').count() as i32,
        }
    }

    fn neighbours((x, y, z, w): HyperPoint) -> impl Iterator<Item = HyperPoint> {
        (-1..2)
            .map(move |i| {
                (-1..2)
                    .map(move |j| {
                        (-1..2)
                            .map(move |k| {
                                (-1..2)
                                    .filter(move |l| (i, j, k, *l) != (0, 0, 0, 0))
                                    .map(move |l| (x + i, y + j, z + k, w + l))
                            })
                            .flatten()
                    })
                    .flatten()
            })
            .flatten()
    }

    fn adj(&self, p: HyperPoint) -> i32 {
        HyperSpace::neighbours(p)
            .filter(|n| self.map.get(&n) == Some(&'#'))
            .count() as i32
    }
}

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: HashMap<Point, char> = HashMap::from_iter(
            HashMap::from(self)
                .iter()
                .map(|((x, y), v)| ((*x, *y, 0), *v)),
        );
        Ok(Space::new(input)
            .into_iter()
            .nth(5)
            .ok_or("No states found")?
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: HashMap<HyperPoint, char> = HashMap::from_iter(
            HashMap::from(self)
                .iter()
                .map(|((x, y), v)| ((*x, *y, 0, 0), *v)),
        );
        Ok(HyperSpace::new(input)
            .into_iter()
            .nth(5)
            .ok_or("No states found")?
            .to_string())
    }
}
