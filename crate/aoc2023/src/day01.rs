#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> u32 {
    input.par_iter()
        .map(|line| {
            let digits = line.chars().filter_map(|c| c.to_digit(10)).collect_vec();
            let tens = *digits.first().unwrap();
            let ones = *digits.last().unwrap();
            10 * tens + ones
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[String]) -> u32 {
    input.par_iter()
        .map(|line| {
            let tens = line.char_indices()
                .find_map(|(idx, c)| match c {
                    '0'..='9' => c.to_digit(10),
                    'o' => line.get(idx + 1..idx + 3).filter(|&rest| rest == "ne").map(|_| 1),
                    't' => match line.get(idx + 1..idx + 3) {
                        Some("wo") => Some(2),
                        Some("hr") => line.get(idx + 3..idx + 5).filter(|&rest| rest == "ee").map(|_| 3),
                        _ => None,
                    },
                    'f' => match line.get(idx + 1..idx + 4) {
                        Some("our") => Some(4),
                        Some("ive") => Some(5),
                        _ => None,
                    },
                    's' => match line.get(idx + 1..idx + 3) {
                        Some("ix") => Some(6),
                        Some("ev") => line.get(idx + 3..idx + 5).filter(|&rest| rest == "en").map(|_| 7),
                        _ => None,
                    },
                    'e' => line.get(idx + 1..idx + 5).filter(|&rest| rest == "ight").map(|_| 8),
                    'n' => line.get(idx + 1..idx + 4).filter(|&rest| rest == "ine").map(|_| 9),
                    _ => None,
                })
                .unwrap();
            let ones = line.char_indices()
                .rev()
                .find_map(|(idx, c)| match c {
                    '0'..='9' => c.to_digit(10),
                    'e' => if let Some(start) = idx.checked_sub(2) {
                        match &line[start..idx] {
                            "on" => Some(1),
                            "re" => idx.checked_sub(4).filter(|&start| &line[start..start + 2] == "th").map(|_| 3),
                            "iv" => idx.checked_sub(3).filter(|&start| &line[start..start + 1] == "f").map(|_| 5),
                            "in" => idx.checked_sub(3).filter(|&start| &line[start..start + 1] == "n").map(|_| 9),
                            _ => None,
                        }
                    } else {
                        None
                    },
                    'o' => idx.checked_sub(2).filter(|&start| &line[start..idx] == "tw").map(|_| 2),
                    'r' => idx.checked_sub(3).filter(|&start| &line[start..idx] == "fou").map(|_| 4),
                    'x' => idx.checked_sub(2).filter(|&start| &line[start..idx] == "si").map(|_| 6),
                    'n' => idx.checked_sub(4).filter(|&start| &line[start..idx] == "seve").map(|_| 7),
                    't' => idx.checked_sub(4).filter(|&start| &line[start..idx] == "eigh").map(|_| 8),
                    _ => None,
                })
                .unwrap();
            10 * tens + ones
        }).sum()
}
