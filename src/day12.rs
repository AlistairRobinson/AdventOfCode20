use crate::{Fail, Input, Solution};

pub static DATA_PATH: &str = "data/day12.txt";
pub static TEST_PATH: &str = "data/test/day12.txt";
pub static TEST_VALUES: (&str, &str) = ("25", "286");

#[derive(Clone, Copy)]
pub enum Facing {
    North,
    South,
    East,
    West,
}

type Point = (i32, i32);

#[derive(Clone, Copy)]
pub struct PositionOne {
    point: Point,
    facing: Facing,
}

impl PositionOne {
    fn new() -> PositionOne {
        PositionOne {
            point: (0, 0),
            facing: Facing::East,
        }
    }

    fn advance(mut self, direction: Facing, magnitude: i32) -> PositionOne {
        let (x, y) = self.point;
        match direction {
            Facing::North => self.point = (x, y + magnitude),
            Facing::East => self.point = (x + magnitude, y),
            Facing::South => self.point = (x, y - magnitude),
            Facing::West => self.point = (x - magnitude, y),
        }
        self
    }

    fn rotate(mut self, deg: i32) -> PositionOne {
        (0..deg / 90).for_each(|_| match self.facing {
            Facing::North => self.facing = Facing::East,
            Facing::East => self.facing = Facing::South,
            Facing::South => self.facing = Facing::West,
            Facing::West => self.facing = Facing::North,
        });
        self
    }

    fn dist(self) -> i32 {
        let (x, y) = self.point;
        x.abs() + y.abs()
    }
}

pub struct PositionTwo {
    point: Point,
    waypoint: Point,
}

impl PositionTwo {
    fn new() -> PositionTwo {
        PositionTwo {
            point: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn advance_waypoint(mut self, direction: Facing, magnitude: i32) -> PositionTwo {
        let (x, y) = self.waypoint;
        match direction {
            Facing::North => self.waypoint = (x, y + magnitude),
            Facing::East => self.waypoint = (x + magnitude, y),
            Facing::South => self.waypoint = (x, y - magnitude),
            Facing::West => self.waypoint = (x - magnitude, y),
        }
        self
    }

    fn follow_waypoint(mut self, magnitude: i32) -> PositionTwo {
        let (x, y) = self.point;
        let (wx, wy) = self.waypoint;
        self.point = (x + wx * magnitude, y + wy * magnitude);
        self
    }

    fn left(mut self, deg: i32) -> PositionTwo {
        (0..deg / 90).for_each(|_| {
            let (wx, wy) = self.waypoint;
            self.waypoint = (-wy, wx);
        });
        self
    }

    fn right(mut self, deg: i32) -> PositionTwo {
        (0..deg / 90).for_each(|_| {
            let (wx, wy) = self.waypoint;
            self.waypoint = (wy, -wx);
        });
        self
    }

    fn dist(self) -> i32 {
        let (x, y) = self.point;
        x.abs() + y.abs()
    }
}

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<&str> = self.data.lines().collect();
        Ok(input
            .iter()
            .fold(PositionOne::new(), |p, i| {
                let x: i32 = i[1..].parse().expect("Unable to parse");
                match i.chars().next().expect("No instruction given") {
                    'N' => p.advance(Facing::North, x),
                    'E' => p.advance(Facing::East, x),
                    'S' => p.advance(Facing::South, x),
                    'W' => p.advance(Facing::West, x),
                    'F' => p.advance(p.facing, x),
                    'L' => p.rotate(x * 3),
                    'R' => p.rotate(x),
                    _ => p,
                }
            })
            .dist()
            .to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<&str> = self.data.lines().collect();
        Ok(input
            .iter()
            .fold(PositionTwo::new(), |p, i| {
                let x: i32 = i[1..].parse().expect("Unable to parse");
                match i.chars().next().expect("No instruction given") {
                    'N' => p.advance_waypoint(Facing::North, x),
                    'E' => p.advance_waypoint(Facing::East, x),
                    'S' => p.advance_waypoint(Facing::South, x),
                    'W' => p.advance_waypoint(Facing::West, x),
                    'F' => p.follow_waypoint(x),
                    'L' => p.left(x),
                    'R' => p.right(x),
                    _ => p,
                }
            })
            .dist()
            .to_string())
    }
}
