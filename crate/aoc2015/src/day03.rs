#[allow(unused)] use aoc::prelude::*;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl AddAssign<Direction> for [i32; 2] {
    fn add_assign(&mut self, dir: Direction) {
        let [x, y] = self;
        match dir {
            Direction::North => *y -= 1,
            Direction::South => *y += 1,
            Direction::East => *x += 1,
            Direction::West => *x -= 1,
        }
    }
}

#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<Direction> {
    input.chars().map(|c| match c {
        '^' => Direction::North,
        'v' => Direction::South,
        '>' => Direction::East,
        '<' => Direction::West,
        _ => panic!(),
    }).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Direction]) -> usize {
    let mut pos = [0, 0];
    let mut visited = collect![as HashSet<_>: pos];
    for dir in input {
        pos += *dir;
        visited.insert(pos);
    }
    visited.len()
}

#[aoc(day3, part2)]
fn part2(input: &[Direction]) -> usize {
    let mut santa_pos = [0, 0];
    let mut robo_santa_pos = [0, 0];
    let mut visited = collect![as HashSet<_>: [0, 0]];
    for (idx, dir) in input.iter().enumerate() {
        if idx % 2 == 0 {
            santa_pos += *dir;
            visited.insert(santa_pos);
        } else {
            robo_santa_pos += *dir;
            visited.insert(robo_santa_pos);
        }
    }
    visited.len()
}
