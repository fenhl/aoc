#[allow(unused)] use aoc::prelude::*;
use self::{
    Direction::*,
    Turn::*,
};

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl AddAssign<Direction> for [i32; 2] {
    fn add_assign(&mut self, dir: Direction) {
        let [x, y] = self;
        match dir {
            North => *y -= 1,
            East => *x += 1,
            South => *y += 1,
            West => *x -= 1,
        }
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Right,
}

impl AddAssign<Turn> for Direction {
    fn add_assign(&mut self, turn: Turn) {
        *self = match (*self, turn) {
            (West, Right) | (East, Left) => North,
            (North, Right) | (South, Left) => East,
            (East, Right) | (West, Left) => South,
            (South, Right) | (North, Left) => West,
        }
    }
}

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<(Turn, u32)> {
    input.split(", ")
        .map(|part| {
            let (_, turn, distance) = regex_captures!("^(L|R)([0-9]+)$", part).unwrap();
            (match turn {
                "L" => Left,
                "R" => Right,
                _ => unreachable!(),
            }, distance.parse().unwrap())
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[(Turn, u32)]) -> u32 {
    let mut pos = [0, 0];
    let mut facing = North;
    for &(turn, distance) in input {
        facing += turn;
        for _ in 0..distance {
            pos += facing;
        }
    }
    pos.into_iter().map(|coord| coord.unsigned_abs()).sum()
}

/*
#[aoc(day1, part2)]
fn part2(input: &!) -> ! {
    unimplemented!()
}
*/
