#[allow(unused)] use aoc::prelude::*;

fn parse_digit(digit: &str) -> u32 {
    match digit {
        "0" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!(),
    }
}

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> u32 {
    input.par_iter()
        .map(|line| {
            let digits = line.chars().filter_map(|c| c.to_digit(10)).collect_vec();
            let tens = *digits.first().unwrap();
            let ones = *digits.last().unwrap();
            10 * tens + ones
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[String]) -> u32 {
    input.par_iter()
        .map(|line| {
            let first = regex_find!("[0-9]|one|two|three|four|five|six|seven|eight|nine", line).unwrap();
            let (_, last) = regex_captures!(".*([0-9]|one|two|three|four|five|six|seven|eight|nine)", line).unwrap();
            let tens = parse_digit(first);
            let ones = if last.is_empty() { tens } else { parse_digit(last) };
            10 * tens + ones
        }).sum()
}
