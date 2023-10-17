#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<(RangeInclusive<usize>, char, String)> {
    input.lines()
        .map(|line| {
            let (_, min, max, letter, password) = regex_captures!("^([0-9]+)-([0-9]+) (.?): (.*)$", line).unwrap();
            (min.parse().unwrap()..=max.parse().unwrap(), letter.parse().unwrap(), password.to_owned())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(RangeInclusive<usize>, char, String)]) -> usize {
    input.iter()
        .filter(|(range, letter, password)| range.contains(&password.matches(*letter).count()))
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &[(RangeInclusive<usize>, char, String)]) -> usize {
    input.iter()
        .filter(|(range, letter, password)| (password.chars().nth(*range.start() - 1).unwrap() == *letter) ^ (password.chars().nth(*range.end() - 1).unwrap() == *letter))
        .count()
}
