#[allow(unused)] use aoc::prelude::*;

fn increment(password: &String) -> String {
    let mut new_password = password.chars().collect_vec();
    for c in new_password.iter_mut().rev() {
        if let 'z' = c {
            *c = 'a'
        } else {
            *c = char::from(u8::try_from(*c).unwrap() + 1);
            break
        }
    }
    String::from_iter(new_password)
}

#[aoc_generator(day11)]
fn gen(input: &str) -> String {
    input.to_owned()
}

#[aoc(day11, part1)]
fn part1(input: &str) -> String {
    itertools::iterate(input.to_owned(), increment).filter(|password| {
        password.chars().array_windows().any(|window| {
            let [a, b, c] = window.map(u32::from);
            b == a + 1 && c == a + 2
        })
        && !password.contains(&['i', 'o', 'l'][..])
        && {
            let mut windows = password.chars().array_windows();
            windows.find(|[a, b]| a == b).is_some()
            && windows.next().is_some()
            && windows.find(|[a, b]| a == b).is_some()
        }
    }).next().unwrap()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> String {
    itertools::iterate(input.to_owned(), increment).filter(|password| {
        password.chars().array_windows().any(|window| {
            let [a, b, c] = window.map(u32::from);
            b == a + 1 && c == a + 2
        })
        && !password.contains(&['i', 'o', 'l'][..])
        && {
            let mut windows = password.chars().array_windows();
            windows.find(|[a, b]| a == b).is_some()
            && windows.next().is_some()
            && windows.find(|[a, b]| a == b).is_some()
        }
    }).skip(1).next().unwrap()
}
