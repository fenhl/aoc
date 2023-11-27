#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day13)]
fn gen(input: &str) -> HashMap<String, HashMap<String, i32>> {
    let mut map = HashMap::<_, HashMap<_, _>>::default();
    for line in input.lines() {
        let (_, a, sign, happiness, b) = regex_captures!("^(.+) would (gain|lose) ([0-9]+) happiness units? by sitting next to (.+)\\.$", line).expect(line);
        let happiness = if sign == "lose" { -1 } else { 1 } * happiness.parse::<i32>().unwrap();
        map.entry(a.to_owned()).or_default().insert(b.to_owned(), happiness);
    }
    map
}

#[aoc(day13, part1)]
fn part1(input: &HashMap<String, HashMap<String, i32>>) -> i32 {
    input.keys()
        .permutations(input.len())
        .map(|perm| perm.into_iter().circular_tuple_windows().map(|(a, b)| input[a][b] + input[b][a]).sum())
        .max().unwrap()
}

#[aoc(day13, part2)]
fn part2(input: &HashMap<String, HashMap<String, i32>>) -> i32 {
    input.keys()
        .permutations(input.len())
        .map(|perm| perm.into_iter().array_windows().map(|[a, b]| input[a][b] + input[b][a]).sum())
        .max().unwrap()
}
