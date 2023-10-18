#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|line| line.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|line| {
        let (min, max) = line.iter().minmax().into_option().unwrap();
        max - min
    }).sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|line| {
        let [a, b] = line.iter().permutations(2).map(|v| <[_; 2]>::try_from(v).unwrap()).find(|&[&a, &b]| a % b == 0).unwrap();
        a / b
    }).sum()
}
