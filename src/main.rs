use aoc20::day1;
use aoc20::{Fail, Input, Solution};

fn main() -> Result<(), Fail> {
    let input = Input::from(day1::DATA_PATH);
    println!("{}", input.part1()?);
    println!("{}", input.part2()?);
    Ok(())
}
