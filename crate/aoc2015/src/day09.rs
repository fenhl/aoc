#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day9)]
fn gen(input: &str) -> HashMap<String, HashMap<String, u32>> {
    let mut map = HashMap::<_, HashMap<_, _>>::default();
    for line in input.lines() {
        let (_, a, b, dist) = regex_captures!("^(.+) to (.+) = ([0-9]+)$", line).unwrap();
        let dist = dist.parse().unwrap();
        map.entry(a.to_owned()).or_default().insert(b.to_owned(), dist);
        map.entry(b.to_owned()).or_default().insert(a.to_owned(), dist);
    }
    map
}

#[aoc(day9, part1)]
fn part1(input: &HashMap<String, HashMap<String, u32>>) -> u32 {
    input.keys()
        .permutations(input.len())
        .map(|perm| perm.into_iter().array_windows().map(|[a, b]| input[a][b]).sum())
        .min().unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &HashMap<String, HashMap<String, u32>>) -> u32 {
    input.keys()
        .permutations(input.len())
        .map(|perm| perm.into_iter().array_windows().map(|[a, b]| input[a][b]).sum())
        .max().unwrap()
}
