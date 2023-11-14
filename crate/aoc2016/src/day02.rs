#[allow(unused)] use aoc::prelude::*;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<Vec<Direction>> {
    input.lines()
        .map(|line| line.chars().map(|c| match c {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => panic!(),
        }).collect())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<Direction>]) -> String {
    let mut buf = String::with_capacity(input.len());
    let mut x = 1u8;
    let mut y = 1u8;
    for row in input {
        for dir in row {
            match dir {
                Direction::Up => y = y.saturating_sub(1),
                Direction::Right => x = 2.min(x + 1),
                Direction::Down => y = 2.min(y + 1),
                Direction::Left => x = x.saturating_sub(1),
            }
        }
        buf.push(match [x, y] {
            [0, 0] => '1',
            [1, 0] => '2',
            [2, 0] => '3',
            [0, 1] => '4',
            [1, 1] => '5',
            [2, 1] => '6',
            [0, 2] => '7',
            [1, 2] => '8',
            [2, 2] => '9',
            _ => unreachable!(),
        });
    }
    buf
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<Direction>]) -> String {
    let mut buf = String::with_capacity(input.len());
    let mut x = 0u8;
    let mut y = 2u8;
    for row in input {
        for dir in row {
            match dir {
                Direction::Up => y = y.saturating_sub(1).max(match x {
                    0 | 4 => 2,
                    1 | 3 => 1,
                    2 => 0,
                    _ => unreachable!(),
                }),
                Direction::Right => x = (x + 1).min(match y {
                    0 | 4 => 2,
                    1 | 3 => 3,
                    2 => 4,
                    _ => unreachable!(),
                }),
                Direction::Down => y = (y + 1).min(match x {
                    0 | 4 => 2,
                    1 | 3 => 3,
                    2 => 4,
                    _ => unreachable!(),
                }),
                Direction::Left => x = x.saturating_sub(1).max(match y {
                    0 | 4 => 2,
                    1 | 3 => 1,
                    2 => 0,
                    _ => unreachable!(),
                }),
            }
        }
        buf.push(match [x, y] {
            [2, 0] => '1',
            [1, 1] => '2',
            [2, 1] => '3',
            [3, 1] => '4',
            [0, 2] => '5',
            [1, 2] => '6',
            [2, 2] => '7',
            [3, 2] => '8',
            [4, 2] => '9',
            [1, 3] => 'A',
            [2, 3] => 'B',
            [3, 3] => 'C',
            [2, 4] => 'D',
            _ => unreachable!(),
        });
    }
    buf
}
