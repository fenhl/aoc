#[allow(unused)] use aoc::prelude::*;

enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

#[aoc_generator(day6)]
fn gen(input: &str) -> Vec<(Instruction, [RangeInclusive<usize>; 2])> {
    input.lines()
        .map(|line| {
            let (_, instr, left, top, right, bottom) = regex_captures!("^(turn on|turn off|toggle) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)$", line).unwrap();
            (
                match instr { "turn on" => Instruction::TurnOn, "turn off" => Instruction::TurnOff, "toggle" => Instruction::Toggle, _ => unreachable!() },
                [left.parse().unwrap()..=right.parse().unwrap(), top.parse().unwrap()..=bottom.parse().unwrap()],
            )
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[(Instruction, [RangeInclusive<usize>; 2])]) -> usize {
    let mut grid = [[false; 1000]; 1000];
    for (instr, [rows, cols]) in input {
        for row in rows.clone() {
            for col in cols.clone() {
                match instr {
                    Instruction::TurnOn => grid[row][col] = true,
                    Instruction::TurnOff => grid[row][col] = false,
                    Instruction::Toggle => grid[row][col] = !grid[row][col],
                }
            }
        }
    }
    grid.iter().map(|row| row.iter().filter(|&&x| x).count()).sum()
}

#[aoc(day6, part2)]
fn part2(input: &[(Instruction, [RangeInclusive<usize>; 2])]) -> u64 {
    let mut grid = Box::new([[0u64; 1000]; 1000]);
    for (instr, [rows, cols]) in input {
        for row in rows.clone() {
            for col in cols.clone() {
                match instr {
                    Instruction::TurnOn => grid[row][col] += 1,
                    Instruction::TurnOff => grid[row][col] = grid[row][col].saturating_sub(1),
                    Instruction::Toggle => grid[row][col] += 2,
                }
            }
        }
    }
    grid.iter().map(|row| row.iter().sum::<u64>()).sum()
}
