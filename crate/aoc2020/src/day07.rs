#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day7)]
fn gen(input: &str) -> HashMap<String, HashMap<String, u32>> {
    input.lines()
        .map(|line| {
            let (_, color, contents) = regex_captures!("^(.+?) bags contain (.+?)\\.$", line).unwrap();
            (color.to_owned(), contents.split(", ").filter_map(|content| {
                if content == "no other bags" { None } else {
                    let (_, amount, color) = regex_captures!("^([0-9]+) (.+?) bags?$", content).unwrap();
                    Some((color.to_owned(), amount.parse().unwrap()))
                }
            }).collect())
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<String, HashMap<String, u32>>) -> usize {
    let mut eligible_bags = collect![as HashSet<_>: "shiny gold"];
    loop {
        let mut new_found = false;
        for (color, contents) in input {
            if !eligible_bags.contains(&color[..]) && contents.iter().any(|(color, _)| eligible_bags.contains(&color[..])) {
                new_found = true;
                eligible_bags.insert(color);
            }
        }
        if !new_found { break }
    }
    eligible_bags.len() - 1
}

fn num_bags(input: &HashMap<String, HashMap<String, u32>>, color: &str) -> u32 {
    1 + input[color].iter().map(|(color, amount)| amount * num_bags(input, color)).sum::<u32>()
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<String, HashMap<String, u32>>) -> u32 {
    num_bags(input, "shiny gold") - 1
}
