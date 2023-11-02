#[allow(unused)] use aoc::prelude::*;

#[derive(Clone)]
enum State {
    Flying {
        remaining_seconds: u32,
    },
    Resting {
        remaining_seconds: u32,
    },
}

#[derive(Clone)]
struct Reindeer {
    speed: u32,
    duration: u32,
    rest: u32,
    distance: u32,
    state: State,
    score: u32,
}

#[aoc_generator(day14)]
fn gen(input: &str) -> Vec<Reindeer> {
    input.lines()
        .map(|line| {
            let (_, speed, duration, rest) = regex_captures!("^[^ ]+ can fly ([0-9]+) km/s for ([0-9]+) seconds?, but then must rest for ([0-9]+) seconds?\\.$", line).unwrap();
            Reindeer {
                speed: speed.parse().unwrap(),
                duration: duration.parse().unwrap(),
                rest: rest.parse().unwrap(),
                distance: 0,
                state: State::Resting { remaining_seconds: 0 },
                score: 0,
            }
        })
        .collect()
}

fn move_reindeer(reindeer: &mut [Reindeer]) {
    for reindeer in reindeer {
        match reindeer.state {
            State::Flying { remaining_seconds: 0 } => reindeer.state = State::Resting { remaining_seconds: reindeer.rest },
            State::Resting { remaining_seconds: 0 } => reindeer.state = State::Flying { remaining_seconds: reindeer.duration },
            _ => {}
        }
        match reindeer.state {
            State::Flying { ref mut remaining_seconds } => {
                *remaining_seconds -= 1;
                reindeer.distance += reindeer.speed;
            }
            State::Resting { ref mut remaining_seconds } => {
                *remaining_seconds -= 1;
            }
        }
    }
}

#[aoc(day14, part1)]
fn part1(input: &[Reindeer]) -> u32 {
    let mut reindeer = input.to_owned();
    for _ in 0..2503 {
        move_reindeer(&mut reindeer);
    }
    reindeer.into_iter().map(|reindeer| reindeer.distance).max().unwrap()
}

#[aoc(day14, part2)]
fn part2(input: &[Reindeer]) -> u32 {
    let mut reindeer = input.to_owned();
    for _ in 0..2503 {
        move_reindeer(&mut reindeer);
        for reindeer in reindeer.iter_mut().max_set_by_key(|reindeer| reindeer.distance) {
            reindeer.score += 1;
        }
    }
    reindeer.into_iter().map(|reindeer| reindeer.score).max().unwrap()
}
