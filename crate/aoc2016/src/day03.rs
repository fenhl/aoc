#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<[u32; 3]> {
    input.lines()
        .map(|line| line.split_whitespace().map(|side| side.parse().unwrap()).collect_vec().try_into().unwrap())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[[u32; 3]]) -> usize {
    input.iter()
        .filter(|&&[a, b, c]| a + b > c && a + c > b && b + c > a)
        .count()
}

#[aoc(day3, part2)]
fn part2(input: &[[u32; 3]]) -> usize {
    input.iter()
        .array_chunked()
        .flat_map(|[&[a1, a2, a3], &[b1, b2, b3], &[c1, c2, c3]]| [[a1, b1, c1], [a2, b2, c2], [a3, b3, c3]])
        .filter(|&[a, b, c]| a + b > c && a + c > b && b + c > a)
        .count()
}
