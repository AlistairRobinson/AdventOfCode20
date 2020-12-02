use aoc20::day2::DATA_PATH;
use aoc20::{Fail, Input, Solution};

fn main() -> Result<(), Fail> {
    let input = Input::from(DATA_PATH);
    println!("{}", input.part1()?);
    println!("{}", input.part2()?);
    Ok(())
}
