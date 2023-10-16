#[allow(unused)] use aoc::prelude::*;

enum Direction {
    Up,
    Down,
}

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<Direction> {
    input.chars()
        .map(|c| match c {
            '(' => Direction::Up,
            ')' => Direction::Down,
            _ => panic!(),
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[Direction]) -> i32 {
    input.iter().fold(0, |floor, dir| match dir {
        Direction::Up => floor + 1,
        Direction::Down => floor - 1,
    })
}

#[aoc(day1, part2)]
fn part2(input: &[Direction]) -> usize {
    1 + input.iter()
        .enumerate()
        .fold((0, None), |(floor, pos), (new_pos, dir)| if let Some(pos) = pos {
            (floor, Some(pos))
        } else {
            let new_floor = match dir {
                Direction::Up => floor + 1,
                Direction::Down => floor - 1,
            };
            (new_floor, (new_floor < 0).then_some(new_pos))
        })
        .1.unwrap()
}
