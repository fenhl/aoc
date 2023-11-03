#[allow(unused)] use aoc::prelude::*;

#[derive(Debug, Default)]
struct Data {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

#[aoc_generator(day16)]
fn gen(input: &str) -> [Data; 500] {
    input.lines()
        .map(|line| {
            let (_, data_text) = line.split_once(": ").unwrap();
            let mut data = Data::default();
            for attribute in data_text.split(", ") {
                let (kind, number) = attribute.split_once(": ").unwrap();
                let number = number.parse().unwrap();
                match kind {
                    "children" => data.children = Some(number),
                    "cats" => data.cats = Some(number),
                    "samoyeds" => data.samoyeds = Some(number),
                    "pomeranians" => data.pomeranians = Some(number),
                    "akitas" => data.akitas = Some(number),
                    "vizslas" => data.vizslas = Some(number),
                    "goldfish" => data.goldfish = Some(number),
                    "trees" => data.trees = Some(number),
                    "cars" => data.cars = Some(number),
                    "perfumes" => data.perfumes = Some(number),
                    _ => panic!(),
                }
            }
            data
        })
        .collect_vec()
        .try_into().unwrap()
}

#[aoc(day16, part1)]
fn part1(input: &[Data; 500]) -> usize {
    1 + input.iter().position(|data|
        data.children.map_or(true, |children| children == 3)
        && data.cats.map_or(true, |cats| cats == 7)
        && data.samoyeds.map_or(true, |samoyeds| samoyeds == 2)
        && data.pomeranians.map_or(true, |pomeranians| pomeranians == 3)
        && data.akitas.map_or(true, |akitas| akitas == 0)
        && data.vizslas.map_or(true, |vizslas| vizslas == 0)
        && data.goldfish.map_or(true, |goldfish| goldfish == 5)
        && data.trees.map_or(true, |trees| trees == 3)
        && data.cars.map_or(true, |cars| cars == 2)
        && data.perfumes.map_or(true, |perfumes| perfumes == 1)
    ).unwrap()
}

#[aoc(day16, part2)]
fn part2(input: &[Data; 500]) -> usize {
    1 + input.iter().position(|data|
        data.children.map_or(true, |children| children == 3)
        && data.cats.map_or(true, |cats| cats > 7)
        && data.samoyeds.map_or(true, |samoyeds| samoyeds == 2)
        && data.pomeranians.map_or(true, |pomeranians| pomeranians < 3)
        && data.akitas.map_or(true, |akitas| akitas == 0)
        && data.vizslas.map_or(true, |vizslas| vizslas == 0)
        && data.goldfish.map_or(true, |goldfish| goldfish < 5)
        && data.trees.map_or(true, |trees| trees > 3)
        && data.cars.map_or(true, |cars| cars == 2)
        && data.perfumes.map_or(true, |perfumes| perfumes == 1)
    ).unwrap()
}
