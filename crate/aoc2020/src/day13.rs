#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day13)]
fn gen(input: &str) -> (u32, Vec<Option<u32>>) {
    let (my_arrival, bus_ids) = input.lines().collect_tuple().unwrap();
    (my_arrival.parse().unwrap(), bus_ids.split(',').map(|id| if id == "x" { None } else { Some(id.parse().unwrap()) }).collect())
}

#[aoc(day13, part1)]
fn part1((my_arrival, bus_ids): &(u32, Vec<Option<u32>>)) -> u32 {
    let (id, dep) = bus_ids.iter()
        .copied()
        .filter_map(identity)
        .map(|id| (id, (0..).step_by(id as usize).filter(|dep| dep >= my_arrival).next().unwrap()))
        .min_by_key(|(_, dep)| *dep)
        .unwrap();
    id * (dep - my_arrival)
}

/*
#[aoc(day13, part2)]
fn part2(input: &!) -> ! {
    unimplemented!()
}
*/
