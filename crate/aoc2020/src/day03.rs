#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<Vec<bool>> {
    input.lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<bool>]) -> u32 {
    let mut count = 0;
    let mut x_pos = 0;
    for line in input.iter().skip(1) {
        x_pos = (x_pos + 3) % line.len();
        if line[x_pos] { count += 1 }
    }
    count
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<bool>]) -> u32 {
    let mut product = 1;
    for (right, down) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut count = 0;
        let mut x_pos = 0;
        for line in input.iter().step_by(down).skip(1) {
            x_pos = (x_pos + right) % line.len();
            if line[x_pos] { count += 1 }
        }
        product *= count;
    }
    product
}
