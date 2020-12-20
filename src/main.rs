#![allow(unused_imports)]

use aoc20::day20::{DATA_PATH, TEST_PATH, TEST_VALUES};
use aoc20::{Fail, Input, Solution};

fn main() -> Result<(), Fail> {
    let input = Input::from(DATA_PATH);
    println!("{}", input.part1()?);
    println!("{}", input.part2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() -> Result<(), Fail> {
        let input = Input::from(TEST_PATH);
        assert!(input.part1()? == TEST_VALUES.0, input.part1()?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Fail> {
        let input = Input::from(TEST_PATH);
        assert!(input.part2()? == TEST_VALUES.1, input.part2()?);
        Ok(())
    }
}
