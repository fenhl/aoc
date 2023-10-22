#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day12)]
fn gen(input: &str) -> serde_json::Result<Json> {
    input.parse()
}

#[aoc(day12, part1)]
fn part1(input: &Json) -> i64 {
    match input {
        Json::Null | Json::Bool(_) => panic!(),
        Json::Number(n) => n.as_i64().unwrap(),
        Json::String(_) => 0,
        Json::Array(a) => a.iter().map(part1).sum(),
        Json::Object(o) => o.values().map(part1).sum(),
    }
}

#[aoc(day12, part2)]
fn part2(input: &Json) -> i64 {
    match input {
        Json::Null | Json::Bool(_) => panic!(),
        Json::Number(n) => n.as_i64().unwrap(),
        Json::String(_) => 0,
        Json::Array(a) => a.iter().map(part2).sum(),
        Json::Object(o) => if o.values().any(|v| v.as_str().map_or(false, |s| s == "red")) { 0 } else { o.values().map(part2).sum() },
    }
}
