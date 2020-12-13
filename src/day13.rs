use crate::{Fail, Input, Solution};

pub static DATA_PATH: &str = "data/day13.txt";
pub static TEST_PATH: &str = "data/test/day13.txt";
pub static TEST_VALUES: (&str, &str) = ("295", "1068781");

type Equation = (i64, i64);

impl Solution for Input {
    fn part1(&self) -> Result<String, Fail> {
        let input: Vec<&str> = self.data.lines().collect();
        let start: i32 = input[0].parse()?;
        let buses: Vec<i32> = input[1].split(",").filter_map(|n| n.parse().ok()).collect();
        let depart: i32 = (start..)
            .skip_while(|t| buses.iter().all(|b| t % b != 0))
            .next()
            .ok_or("None found")?;
        let bus: i32 = *buses
            .iter()
            .filter(|b| depart % **b == 0)
            .next()
            .ok_or("None found")?;
        Ok(((depart - start) * bus).to_string())
    }

    fn part2(&self) -> Result<String, Fail> {
        let input: Vec<&str> = self.data.lines().collect();
        let mut diff: i64 = -1;
        let eq: Equation = input[1].split(",").fold((1, 0), |b, n| {
            diff += 1;
            match n {
                "x" => b,
                bus => start_find(b, (bus.parse().unwrap(), 0), diff),
            }
        });
        Ok(eq.1.to_string())
    }
}

fn start_find(a: Equation, b: Equation, diff: i64) -> Equation {
    find(a, 1, b, 1, diff)
}

fn find(a: Equation, ax: i64, b: Equation, bx: i64, diff: i64) -> Equation {
    match eval(a, ax) + diff - eval(b, bx) {
        0 => (a.0 * b.0, eval(a, ax)),
        d if d > 0 => find(a, ax, b, div(eval(a, ax) - b.1 + diff, b.0), diff),
        _ => find(a, div(eval(b, bx) - a.1 - diff, a.0), b, bx, diff),
    }
}

fn eval(e: Equation, x: i64) -> i64 {
    e.0 * x + e.1
}

fn div(a: i64, b: i64) -> i64 {
    match a % b {
        0 => a / b,
        _ => a / b + 1,
    }
}
