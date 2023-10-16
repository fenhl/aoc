#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<Vec<u32>> {
    input.split("\n\n")
        .map(|section| section.lines().map(|line| line.parse().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|section| section.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|section| section.iter().sum::<u32>()).sorted().rev().take(3).sum()
}
