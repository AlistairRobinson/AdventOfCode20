use crate::{Fail, Input, Solution};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub static DATA_PATH: &str = "data/day20.txt";
pub static TEST_PATH: &str = "data/test/day20.txt";
pub static TEST_VALUES: (&str, &str) = ("20899048083289", "done");

type Point = (i64, i64);

#[derive(Debug)]
pub struct Tile {
    id: i64,
    top: Vec<char>,
    rgt: Vec<char>,
    btm: Vec<char>,
    lft: Vec<char>,
    map: HashMap<Point, char>,
}

impl From<&str> for Tile {
    fn from(s: &str) -> Tile {
        let mut l = s.lines();
        let id: i64 = l.next().and_then(|i| i[5..9].parse::<i64>().ok()).unwrap();
        let map: HashMap<Point, char> = l
            .enumerate()
            .map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i64, y as i64), c))
            })
            .flatten()
            .collect();
        let mut kvs: Vec<(&Point, &char)> = map.iter().collect();
        kvs.sort();
        Tile {
            id: id,
            top: kvs.iter().filter(|((_, y), _)| *y == 0).map(|(_, v)| **v).collect(),
            btm: kvs.iter().filter(|((_, y), _)| *y == 9).map(|(_, v)| **v).collect(),
            lft: kvs.iter().filter(|((x, _), _)| *x == 0).map(|(_, v)| **v).collect(),
            rgt: kvs.iter().filter(|((x, _), _)| *x == 9).map(|(_, v)| **v).collect(),
            map: map
        }
    }
}

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input = self.data.split("\r\n\r\n");
        let tiles: Vec<Tile> = input.map(|t| Tile::from(t)).collect();
        let edges: HashSet<(i64, Vec<char>)> = HashSet::from_iter(tiles.iter().map(|t| {
            vec![(t.id, t.top.clone()), (t.id, t.lft.clone()), (t.id, t.rgt.clone()), (t.id, t.btm.clone())]
        }).flatten());
        let matching: Vec<&(i64, Vec<char>)> = edges.iter().filter(|(_, v)| {
            edges.iter().filter(|(_, x)| {
                let mut r = x.clone();
                r.reverse();
                v == x || v == &r
            }).count() == 2
        }).collect();
        Ok(tiles.iter().map(|t| t.id).filter(|id| matching.iter().filter(|m| m.0 == *id).count() == 2).product::<i64>().to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        Ok("done".to_string())
    }
}
