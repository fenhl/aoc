#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    input.iter().circular_array_windows().filter(|[a, b]| a == b).map(|[a, _]| a).sum()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    let (first, second) = input.split_at(input.len() / 2);
    2 * first.iter().zip_eq(second).filter(|(a, b)| a == b).map(|(a, _)| a).sum::<u32>()
}
