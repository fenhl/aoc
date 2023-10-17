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
            program.run_with_input(phase).unwrap_input();
            value = program.run_with_input(value).unwrap_output();
        }
        value
    }).max().expect("no permutations")
}

#[aoc(day7, part2)]
fn part2(input: &Program) -> isize {
    (5..10).permutations(5).map(|perm| {
        let mut value = 0;
        let mut amps = vec![input.clone(); 5];
        for (phase, amp) in perm.into_iter().zip_eq(&mut amps) {
            amp.run_with_input(phase).unwrap_input();
        }
        loop {
            for amp in &mut amps {
                match amp.run_with_input(value) {
                    Event::Input => panic!("unexpected input request"),
                    Event::Output(new_value) => value = new_value,
                    Event::Halt => return value,
                }
            }
        }
    }).max().expect("no permutations")
}

#[test]
fn ex0() {
    assert_eq!(part1(&"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".parse().unwrap()), 43210);
}
