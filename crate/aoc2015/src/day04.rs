#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day4)]
fn gen(input: &str) -> String {
    input.to_owned()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let mut ctx = md5::Context::new();
    ctx.consume(input);
    (0..).find(|n| {
        let mut ctx = ctx.clone();
        ctx.consume(n.to_string());
        format!("{:x}", ctx.compute()).starts_with("00000")
    }).unwrap()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let mut ctx = md5::Context::new();
    ctx.consume(input);
    (0..).find(|n| {
        let mut ctx = ctx.clone();
        ctx.consume(n.to_string());
        ctx.compute().into_iter().take(3).all(|byte| byte == 0)
    }).unwrap()
}
