use {
    std::{
        num::ParseIntError,
        str::FromStr,
    },
    derive_more::{
        Index,
        IndexMut,
    },
    num::Integer as _,
};

#[derive(Clone, Copy)]
enum Instruction {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

use Instruction::*;

impl Instruction {
    fn num_params(&self) -> usize {
        match self {
            Add => 3,
            Multiply => 3,
            Input => 1,
            Output => 1,
            JumpIfTrue => 2,
            JumpIfFalse => 2,
            LessThan => 3,
            Equals => 3,
            Halt => 0,
        }
    }
}

impl TryFrom<isize> for Instruction {
    type Error = isize;

    fn try_from(opcode: isize) -> Result<Instruction, isize> {
        match opcode {
            1 => Ok(Add),
            2 => Ok(Multiply),
            3 => Ok(Input),
            4 => Ok(Output),
            5 => Ok(JumpIfTrue),
            6 => Ok(JumpIfFalse),
            7 => Ok(LessThan),
            8 => Ok(Equals),
            99 => Ok(Halt),
            _ => Err(opcode),
        }
    }
}

enum Arg {
    Pos {
        pos: usize,
        value: isize,
    },
    Immediate(isize),
}

impl Arg {
    fn new(mode: isize, raw_arg: isize, program: &Program) -> Arg {
        match mode {
            0 => Arg::Pos {
                pos: raw_arg as usize,
                value: program[raw_arg as usize],
            },
            1 => Arg::Immediate(raw_arg),
            _ => panic!("unknown argument mode: {mode}"),
        }
    }

    fn read(&self) -> isize {
        match *self {
            Arg::Pos { value, .. } => value,
            Arg::Immediate(value) => value,
        }
    }
}

enum InstrResult {
    MoveTo(usize),
    RequestInput,
    ConsumeInput(usize),
    Output(isize, usize),
    Halt,
}

#[must_use]
pub(crate) enum Event {
    Input,
    Output(isize),
    Halt,
}

impl Event {
    #[track_caller]
    pub(crate) fn unwrap_halt(self) {
        if !matches!(self, Self::Halt) {
            panic!("expected halt")
        }
    }

    #[track_caller]
    pub(crate) fn unwrap_input(self) {
        if !matches!(self, Self::Input) {
            panic!("expected input request")
        }
    }

    #[track_caller]
    pub(crate) fn unwrap_output(self) -> isize {
        if let Self::Output(output) = self {
            output
        } else {
            panic!("no output")
        }
    }
}

#[derive(Clone, Index, IndexMut)]
pub struct Program {
    #[index]
    #[index_mut]
    memory: Vec<isize>,
    ip: usize,
}

impl Program {
    pub(crate) fn new(memory: Vec<isize>) -> Program {
        Program {
            memory,
            ip: 0,
        }
    }

    fn write(&mut self, arg: &Arg, value: isize) {
        match *arg {
            Arg::Pos { pos, .. } => { self[pos] = value; }
            Arg::Immediate(_) => panic!("can't write to immediate-mode argument"),
        }
    }

    fn apply_with_args(&mut self, instr: Instruction, args: Vec<Arg>, input: Option<isize>) -> InstrResult {
        match instr {
            Add => self.write(&args[2], args[0].read() + args[1].read()),
            Multiply => self.write(&args[2], args[0].read() * args[1].read()),
            Input => {
                let Some(input) = input else { return InstrResult::RequestInput };
                self.write(&args[0], input);
                return InstrResult::ConsumeInput(self.ip + 1 + instr.num_params())
            }
            Output => return InstrResult::Output(args[0].read(), self.ip + 1 + instr.num_params()),
            JumpIfTrue => if args[0].read() != 0 { return InstrResult::MoveTo(args[1].read() as usize) },
            JumpIfFalse => if args[0].read() == 0 { return InstrResult::MoveTo(args[1].read() as usize) },
            LessThan => self.write(&args[2], if args[0].read() < args[1].read() { 1 } else { 0 }),
            Equals => self.write(&args[2], if args[0].read() == args[1].read() { 1 } else { 0 }),
            Halt => return InstrResult::Halt,
        }
        InstrResult::MoveTo(self.ip + 1 + instr.num_params())
    }

    fn step(&mut self, input: Option<isize>) -> InstrResult {
        let (mut modes, instr) = self[self.ip].div_rem(&100);
        let instr = Instruction::try_from(instr).expect(&format!("unknown instruction at position {}", self.ip));
        let raw_args = self[self.ip + 1..self.ip + 1 + instr.num_params()].to_vec();
        let mut args = Vec::default();
        for i in 0..instr.num_params() {
            let (next_modes, mode) = modes.div_rem(&10);
            args.push(Arg::new(mode, raw_args[i], &self));
            modes = next_modes;
        }
        self.apply_with_args(instr, args, input)
    }

    fn run_inner(&mut self, mut input: Option<isize>) -> Event {
        loop {
            match self.step(input) {
                InstrResult::MoveTo(new_ip) => self.ip = new_ip,
                InstrResult::RequestInput => break Event::Input,
                InstrResult::ConsumeInput(new_ip) => {
                    self.ip = new_ip;
                    input = None;
                }
                InstrResult::Output(output, new_ip) => {
                    self.ip = new_ip;
                    break Event::Output(output)
                }
                InstrResult::Halt => break Event::Halt,
            }
        }
    }

    pub(crate) fn run(&mut self) -> Event {
        self.run_inner(None)
    }

    pub(crate) fn run_with_input(&mut self, input: isize) -> Event {
        self.run_inner(Some(input))
    }
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Program, ParseIntError> {
        Ok(Program::new(s.split(',').map(str::parse).collect::<Result<_, _>>()?))
    }
}
