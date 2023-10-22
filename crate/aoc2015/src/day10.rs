#[allow(unused)] use aoc::prelude::*;

fn look_and_say(input: &Vec<char>) -> Vec<char> {
    input.iter().group_by(|&&c| c).into_iter().flat_map(|(digit, digits)| format!("{}{digit}", digits.count()).chars().collect_vec()).collect()
}

#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[aoc(day10, part1)]
fn part1(input: &[char]) -> usize {
    itertools::iterate(input.iter().copied().collect_vec(), look_and_say).nth(40).unwrap().len()
}

#[aoc(day10, part2)]
fn part2(input: &[char]) -> usize {
    itertools::iterate(input.iter().copied().collect_vec(), look_and_say).nth(50).unwrap().len()
}
