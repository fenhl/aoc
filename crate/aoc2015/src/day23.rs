#[allow(unused)] use aoc::prelude::*;

enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => Err(()),
        }
    }
}

enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

fn run(mut a: u32, mut b: u32, instructions: &[Instruction]) -> u32 {
    let mut ip = 0;
    while let Some(instruction) = instructions.get(ip) {
        match instruction {
            Instruction::Half(reg) => match reg {
                Register::A => a /= 2,
                Register::B => b /= 2,
            },
            Instruction::Triple(reg) => match reg {
                Register::A => a *= 3,
                Register::B => b *= 3,
            },
            Instruction::Increment(reg) => match reg {
                Register::A => a += 1,
                Register::B => b += 1,
            },
            Instruction::Jump(offset) => if let Some(new_ip) = ip.checked_add_signed(*offset) {
                ip = new_ip;
                continue
            } else {
                break
            },
            Instruction::JumpIfEven(reg, offset) => if match reg {
                Register::A => a,
                Register::B => b,
            } % 2 == 0 {
                if let Some(new_ip) = ip.checked_add_signed(*offset) {
                    ip = new_ip;
                    continue
                } else {
                    break
                }
            },
            Instruction::JumpIfOne(reg, offset) => if match reg {
                Register::A => a,
                Register::B => b,
            } == 1 {
                if let Some(new_ip) = ip.checked_add_signed(*offset) {
                    ip = new_ip;
                    continue
                } else {
                    break
                }
            },
        }
        ip += 1;
    }
    b
}

#[aoc_generator(day23)]
fn gen(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| {
            let (name, args) = line.split_once(' ').unwrap();
            match name {
                "hlf" => Instruction::Half(args.parse().unwrap()),
                "tpl" => Instruction::Triple(args.parse().unwrap()),
                "inc" => Instruction::Increment(args.parse().unwrap()),
                "jmp" => Instruction::Jump(args.parse().unwrap()),
                "jie" => {
                    let (register, offset) = args.split_once(", ").unwrap();
                    Instruction::JumpIfEven(register.parse().unwrap(), offset.parse().unwrap())
                }
                "jio" => {
                    let (register, offset) = args.split_once(", ").unwrap();
                    Instruction::JumpIfOne(register.parse().unwrap(), offset.parse().unwrap())
                }
                _ => panic!(),
            }
        })
        .collect()
}

#[aoc(day23, part1)]
fn part1(input: &[Instruction]) -> u32 {
    run(0, 0, input)
}

#[aoc(day23, part2)]
fn part2(input: &[Instruction]) -> u32 {
    run(1, 0, input)
}
