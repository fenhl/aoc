#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    let counts = input.iter()
        .map(|line| {
            let chars = line.iter().collect::<HashSet<_>>();
            chars.into_iter().map(|c| (c, line.iter().filter(|&iter_char| iter_char == c).count())).collect::<HashMap<_, _>>()
        })
        .collect_vec();
    let twos = counts.iter().filter(|line| line.values().any(|&count| count == 2)).count();
    let threes = counts.iter().filter(|line| line.values().any(|&count| count == 3)).count();
    twos * threes
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<char>]) -> String {
    let [id1, id2] = input.iter()
        .array_combinations()
        .find(|&[id1, id2]| id1.iter().zip_eq(id2).filter(|(c1, c2)| c1 != c2).count() == 1)
        .unwrap();
    id1.iter().zip_eq(id2).filter(|(c1, c2)| c1 == c2).map(|(c, _)| c).collect()
}
