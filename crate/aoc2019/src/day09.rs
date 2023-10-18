#[allow(unused)] use aoc::prelude::*;
use crate::intcode::*;

#[aoc_generator(day9)]
fn gen(input: &str) -> Result<Program, ParseIntError> {
    input.parse()
}

#[aoc(day9, part1)]
fn part1(input: &Program) -> isize {
    let mut program = input.clone();
    let output = program.run_with_input(1).unwrap_output();
    loop {
        match program.run() {
            Event::Input => panic!("unexpected input request"),
            Event::Output(_) => panic!("opcode {output} malfunctioning"),
            Event::Halt => break,
        }
    }
    output
}

#[aoc(day9, part2)]
fn part2(input: &Program) -> isize {
    input.clone().run_with_input(2).unwrap_output()
}
