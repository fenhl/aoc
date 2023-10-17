#[allow(unused)] use aoc::prelude::*;
use crate::intcode::*;

#[aoc_generator(day7)]
fn gen(input: &str) -> Result<Program, ParseIntError> {
    input.parse()
}

#[aoc(day7, part1)]
fn part1(input: &Program) -> isize {
    (0..5).permutations(5).map(|perm| {
        let mut value = 0;
        for phase in perm {
            let mut program = input.clone();
            program.run().unwrap_input();
            program.run_with_input(phase).unwrap_input();
            value = program.run_with_input(value).unwrap_output();
        }
        value
    }).max().expect("no permutations")
}

/*
#[aoc(day7, part2)]
fn part2(input: &!) -> ! {
    unimplemented!()
}
*/

#[test]
fn ex0() {
    assert_eq!(part1(&"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".parse().unwrap()), 43210);
}
