#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<i32> {
    input.lines().map(|change| change.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    let mut freq = 0;
    for change in input.iter().cycle() {
        freq += change;
        if !seen.insert(freq) {
            break
        }
    }
    freq
}
