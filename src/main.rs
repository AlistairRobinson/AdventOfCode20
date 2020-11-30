use aoc20::day0::Solution;
use aoc20::Fail;

fn main() -> Result<(), Fail> {
    let solution = Solution::new();
    println!("{}", solution.part1()?);
    println!("{}", solution.part2()?);
    Ok(())
}
