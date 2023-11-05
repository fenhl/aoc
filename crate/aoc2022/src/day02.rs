#[allow(unused)] use aoc::prelude::*;
use self::{
    Rps::*,
    Symbol::*,
};

#[derive(Clone, Copy)]
enum Symbol {
    X,
    Y,
    Z,
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "X" => Ok(X),
            "Y" => Ok(Y),
            "Z" => Ok(Z),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn for_part2(opponent: Self, sym: Symbol) -> Self {
        match (opponent, sym) {
            (Paper, X) | (Rock, Y) | (Scissors, Z) => Rock,
            (Scissors, X) | (Paper, Y) | (Rock, Z) => Paper,
            (Rock, X) | (Scissors, Y) | (Paper, Z) => Scissors,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn outcome_score(&self, other: &Self) -> u32 {
        match (self, other) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 0,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 6,
        }
    }
}

impl FromStr for Rps {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            _ => Err(()),
        }
    }
}

impl From<Symbol> for Rps {
    fn from(sym: Symbol) -> Self {
        match sym {
            X => Rock,
            Y => Paper,
            Z => Scissors,
        }
    }
}

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<(Rps, Symbol)> {
    input.lines()
        .map(|line| {
            let (opponent, sym) = line.split_once(" ").unwrap();
            (opponent.parse().unwrap(), sym.parse().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(Rps, Symbol)]) -> u32 {
    input.iter()
        .map(|(opponent, sym)| {
            let you = Rps::from(*sym);
            you.score() + you.outcome_score(opponent)
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(Rps, Symbol)]) -> u32 {
    input.iter()
        .map(|(opponent, sym)| {
            let you = Rps::for_part2(*opponent, *sym);
            you.score() + you.outcome_score(opponent)
        })
        .sum()
}
