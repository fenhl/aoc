#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day5)]
fn gen(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day5, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    input.iter()
        .filter(|line|
            line.iter().filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')).count() >= 3
            && line.iter().array_windows().any(|[c1, c2]| c1 == c2)
            && !line.iter().array_windows().any(|window| matches!(window, ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y']))
        )
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    input.iter()
        .filter(|line|
            line.len() >= 4
            && (2..=line.len() - 2).any(|split_idx| {
                let (prefix, suffix) = line.split_at(split_idx);
                let [a, b] = prefix[prefix.len() - 2..] else { unreachable!() };
                suffix.iter().array_windows().any(|[&c, &d]| a == c && b == d)
            })
            && line.iter().array_windows().any(|[a, _, b]| a == b)
        )
        .count()
}
