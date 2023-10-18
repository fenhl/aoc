#[allow(unused)] use aoc::prelude::*;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc_generator(day8)]
fn gen(input: &str) -> Vec<[[u32; WIDTH]; HEIGHT]> {
    input.chars().map(|c| c.to_digit(10).unwrap())
        .array_chunked::<{ WIDTH * HEIGHT }>()
        .map(|layer| layer.into_iter().array_chunked().collect_vec().try_into().unwrap())
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[[[u32; WIDTH]; HEIGHT]]) -> usize {
    let layer = input.iter()
        .min_by_key(|layer| layer.iter().map(|row| row.iter().filter(|&&digit| digit == 0).count()).sum::<usize>())
        .unwrap();
    layer.iter().map(|row| row.iter().filter(|&&digit| digit == 1).count()).sum::<usize>()
    * layer.iter().map(|row| row.iter().filter(|&&digit| digit == 2).count()).sum::<usize>()
}

#[aoc(day8, part2)]
fn part2(input: &[[[u32; WIDTH]; HEIGHT]]) -> String {
    let image = <[[u32; WIDTH]; HEIGHT]>::try_from((0..HEIGHT).map(|y| (0..WIDTH)
        .map(|x| input.iter().map(|layer| layer[y][x]).find(|&pixel| pixel != 2).unwrap())
        .collect_vec()
        .try_into()
        .unwrap()
    ).collect_vec()).unwrap();
    format!("\n{}", image.iter().map(|row| row.iter().map(|digit| match digit {
        0 => " ",
        1 => "X",
        _ => panic!(),
    }).format("")).format("\n"))
}
