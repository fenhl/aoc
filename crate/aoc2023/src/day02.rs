#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<(u32, Vec<HashMap<String, u32>>)> {
    input.lines()
        .map(|line| {
            let (_, id, subsets) = regex_captures!("^Game ([0-9]+): (.*)$", line).unwrap();
            (
                id.parse().unwrap(),
                subsets.split("; ").map(|subset| subset.split(", ").map(|color_count| {
                    let (count, color) = color_count.split_once(' ').unwrap();
                    (color.to_owned(), count.parse().unwrap())
                }).collect()).collect(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(u32, Vec<HashMap<String, u32>>)]) -> u32 {
    input.iter()
        .filter(|(_, subsets)| subsets.iter()
            .all(|subset| subset.iter().all(|(color, &count)| match &**color {
                "red" => count <= 12,
                "green" => count <= 13,
                "blue" => count <= 14,
                _ => count == 0,
            }))
        )
        .map(|&(id, _)| id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(u32, Vec<HashMap<String, u32>>)]) -> u32 {
    input.iter()
        .map(|(_, subsets)| {
            let mins = subsets.iter().fold(HashMap::<_, u32>::default(), |mut mins, subset| {
                for (color, count) in subset {
                    let entry = mins.entry(&**color).or_default();
                    *entry = (*entry).max(*count);
                }
                mins
            });
            mins.get(&"red").copied().unwrap_or_default()
            * mins.get(&"green").copied().unwrap_or_default()
            * mins.get(&"blue").copied().unwrap_or_default()
        })
        .sum()
}
