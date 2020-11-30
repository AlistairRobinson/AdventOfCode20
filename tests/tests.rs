use aoc20::day0::Solution;
use aoc20::Fail;

#[test]
fn test_part1() -> Result<(), Fail> {
    let solution = Solution::from("data/test/day0.txt");
    assert!(solution.part1()? == "33583", solution.part1()?);
    Ok(())
}

#[test]
fn test_part2() -> Result<(), Fail> {
    let solution = Solution::from("data/test/day0.txt");
    assert!(solution.part2()? == "50346", solution.part2()?);
    Ok(())
}
