#[allow(unused)] use aoc::prelude::*;

#[derive(Derivative)]
#[derivative(Default)]
enum Input {
    #[derivative(Default)]
    Const(u16),
    Wire(String),
}

impl Input {
    fn eval<'a>(&'a self, instructions: &HashMap<&str, &'a Instruction>, cache: &mut HashMap<&'a str, u16>) -> u16 {
        match self {
            Self::Const(value) => *value,
            Self::Wire(name) => if let Some(&value) = cache.get(&**name) {
                value
            } else {
                let value = instructions.get(&**name).map(|instr| instr.eval(instructions, cache)).unwrap_or_default();
                cache.insert(&**name, value);
                value
            },
        }
    }
}

impl FromStr for Input {
    type Err = Never;

    fn from_str(s: &str) -> Result<Self, Never> {
        Ok(if let Ok(value) = s.parse() {
            Self::Const(value)
        } else {
            Self::Wire(s.to_owned())
        })
    }
}

#[derive(Derivative)]
#[derivative(Default)]
enum Instruction {
    #[derivative(Default)]
    Id(Input),
    Not(Input),
    And(Input, Input),
    Or(Input, Input),
    Shl(Input, u8),
    Shr(Input, u8),
}

impl Instruction {
    fn eval<'a>(&'a self, instructions: &HashMap<&str, &'a Self>, cache: &mut HashMap<&'a str, u16>) -> u16 {
        match self {
            Self::Id(input) => input.eval(instructions, cache),
            Self::Not(input) => !input.eval(instructions, cache),
            Self::And(a, b) => a.eval(instructions, cache) & b.eval(instructions, cache),
            Self::Or(a, b) => a.eval(instructions, cache) | b.eval(instructions, cache),
            Self::Shl(input, offset) => input.eval(instructions, cache) << offset,
            Self::Shr(input, offset) => input.eval(instructions, cache) >> offset,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if regex_is_match!("^([0-9]+|[a-z]+)$", s) {
            Ok(Self::Id(s.parse().never_unwrap()))
        } else if let Some((_, input)) = regex_captures!("^NOT ([0-9]+|[a-z]+)$", s) {
            Ok(Self::Not(input.parse().never_unwrap()))
        } else if let Some((_, a, b)) = regex_captures!("^([0-9]+|[a-z]+) AND ([0-9]+|[a-z]+)$", s) {
            Ok(Self::And(a.parse().never_unwrap(), b.parse().never_unwrap()))
        } else if let Some((_, a, b)) = regex_captures!("^([0-9]+|[a-z]+) OR ([0-9]+|[a-z]+)$", s) {
            Ok(Self::Or(a.parse().never_unwrap(), b.parse().never_unwrap()))
        } else if let Some((_, input, offset)) = regex_captures!("^([0-9]+|[a-z]+) LSHIFT ([0-9]+)$", s) {
            Ok(Self::Shl(input.parse().never_unwrap(), offset.parse().map_err(|_| s.to_owned())?))
        } else if let Some((_, input, offset)) = regex_captures!("^([0-9]+|[a-z]+) RSHIFT ([0-9]+)$", s) {
            Ok(Self::Shr(input.parse().never_unwrap(), offset.parse().map_err(|_| s.to_owned())?))
        } else {
            Err(s.to_owned())
        }
    }
}

#[aoc_generator(day7)]
fn gen(input: &str) -> Vec<(Instruction, String)> {
    input.lines()
        .map(|line| {
            let (instr, target) = line.rsplit_once(" -> ").unwrap();
            (instr.parse().unwrap(), target.to_owned())
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[(Instruction, String)]) -> u16 {
    let instructions = input.iter().map(|(instr, target)| (&**target, instr)).collect::<HashMap<_, _>>();
    let mut cache = HashMap::default();
    Input::Wire(format!("a")).eval(&instructions, &mut cache)
}

#[aoc(day7, part2)]
fn part2(input: &[(Instruction, String)]) -> u16 {
    let instructions = input.iter().map(|(instr, target)| (&**target, instr)).collect::<HashMap<_, _>>();
    let mut cache = HashMap::default();
    let mut cache = collect!["b" => Input::Wire(format!("a")).eval(&instructions, &mut cache)];
    Input::Wire(format!("a")).eval(&instructions, &mut cache)
}
