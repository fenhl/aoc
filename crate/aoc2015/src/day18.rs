#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day18)]
fn gen(input: &str) -> [[bool; 100]; 100] {
    input.lines()
        .map(|line| line.chars().map(|c| c == '#').collect_vec().try_into().unwrap())
        .collect_vec()
        .try_into().unwrap()
}

#[aoc(day18, part1)]
fn part1(input: &[[bool; 100]; 100]) -> usize {
    let mut board = input.clone();
    for _ in 0..100 {
        board = (0usize..100).map(|y|
            (0usize..100).map(|x| {
                let mut live_neighbors = 0;
                if let Some(y) = y.checked_sub(1) {
                    if let Some(x) = x.checked_sub(1) {
                        if board[y][x] { live_neighbors += 1 }
                    }
                    if board[y][x] { live_neighbors += 1 }
                    if board[y].get(x + 1).map_or(false, |&light| light) { live_neighbors += 1 }
                }
                if let Some(x) = x.checked_sub(1) {
                    if board[y][x] { live_neighbors += 1 }
                }
                if board[y].get(x + 1).map_or(false, |&light| light) { live_neighbors += 1 }
                if let Some(row) = board.get(y + 1) {
                    if let Some(x) = x.checked_sub(1) {
                        if row[x] { live_neighbors += 1 }
                    }
                    if row[x] { live_neighbors += 1 }
                    if row.get(x + 1).map_or(false, |&light| light) { live_neighbors += 1 }
                }
                if board[y][x] { matches!(live_neighbors, 2 | 3) } else { live_neighbors == 3 }
            }).collect_vec().try_into().unwrap()
        ).collect_vec().try_into().unwrap()
    }
    board.into_iter().map(|row| row.into_iter().filter(|&light| light).count()).sum()
}

#[aoc(day18, part2)]
fn part2(input: &[[bool; 100]; 100]) -> usize {
    let mut board = input.clone();
    board[0][0] = true;
    board[0][99] = true;
    board[99][0] = true;
    board[99][99] = true;
    for _ in 0..100 {
        board = (0usize..100).map(|y|
            (0usize..100).map(|x| {
                if let [0 | 99, 0 | 99] = [x, y] {
                    true
                } else {
                    let mut live_neighbors = 0;
                    if let Some(y) = y.checked_sub(1) {
                        if let Some(x) = x.checked_sub(1) {
                            if board[y][x] { live_neighbors += 1 }
                        }
                        if board[y][x] { live_neighbors += 1 }
                        if board[y].get(x + 1).map_or(false, |&light| light) { live_neighbors += 1 }
                    }
                    if let Some(x) = x.checked_sub(1) {
                        if board[y][x] { live_neighbors += 1 }
                    }
                    if board[y].get(x + 1).map_or(false, |&light| light) { live_neighbors += 1 }
                    if let Some(row) = board.get(y + 1) {
                        if let Some(x) = x.checked_sub(1) {
                            if row[x] { live_neighbors += 1 }
                        }
                        if row[x] { live_neighbors += 1 }
                        if row.get(x + 1).map_or(false, |&light| light) { live_neighbors += 1 }
                    }
                    if board[y][x] { matches!(live_neighbors, 2 | 3) } else { live_neighbors == 3 }
                }
            }).collect_vec().try_into().unwrap()
        ).collect_vec().try_into().unwrap()
    }
    board.into_iter().map(|row| row.into_iter().filter(|&light| light).count()).sum()
}
