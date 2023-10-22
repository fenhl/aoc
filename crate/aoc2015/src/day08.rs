#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day8)]
fn gen(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

#[aoc(day8, part1)]
fn part1(input: &[String]) -> usize {
    input.iter()
        .map(|line| line.len() - {
            let mut chars = line[1..line.len() - 1].chars();
            let mut count = 0;
            while let Some(c) = chars.next() {
                if c == '\\' {
                    match chars.next().unwrap() {
                        '\\' | '"' => count += 1,
                        'x' => {
                            assert!(matches!(chars.next().unwrap(), '0'..='9' | 'A'..='F' | 'a'..='f'));
                            assert!(matches!(chars.next().unwrap(), '0'..='9' | 'A'..='F' | 'a'..='f'));
                            count += 1;
                        }
                        _ => panic!(),
                    }
                } else {
                    count += 1;
                }
            }
            count
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &[String]) -> usize {
    input.iter()
        .map(|line| 2 + line.chars().map(|c| if let '\\' | '"' = c { 2 } else { 1 }).sum::<usize>() - line.len())
        .sum()
}
