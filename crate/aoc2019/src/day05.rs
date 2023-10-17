#[allow(unused)] use aoc::prelude::*;
use crate::intcode::*;

#[aoc_generator(day5)]
pub fn gen(input: &str) -> Result<Program, ParseIntError> {
    input.parse()
}

#[aoc(day5, part1)]
pub fn part1(input: &Program) -> isize {
    let mut program = input.clone();
    let mut last_output = program.run_with_input(1).unwrap_output();
    loop {
        match program.run() {
            Event::Input => panic!("unexpected input request"),
            Event::Output(output) => {
                if last_output != 0 {
                    panic!("self-test failed")
                }
                last_output = output;
            }
            Event::Halt => break,
        }
    }
    last_output
}

#[aoc(day5, part2)]
pub fn part2(input: &Program) -> isize {
    let mut program = input.clone();
    program.run_with_input(5).unwrap_output()
}
