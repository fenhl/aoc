#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<[u32; 3]> {
    input.lines()
        .map(|line| line.split('x').map(|dim| dim.parse().unwrap()).collect_vec().try_into().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[[u32; 3]]) -> u32 {
    input.iter()
        .map(|dims| {
            let sides = dims.iter().circular_tuple_windows().map(|(a, b)| a * b).collect_vec();
            2 * sides.iter().sum::<u32>() + sides.iter().min().unwrap()
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[[u32; 3]]) -> u32 {
    input.iter()
        .map(|dims| {
            let perimeters = dims.iter().circular_tuple_windows().map(|(a, b)| 2 * (a + b)).collect_vec();
            perimeters.iter().min().unwrap() + dims.iter().product::<u32>()
        })
        .sum()
}
