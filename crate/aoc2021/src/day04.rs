#[allow(unused)] use aoc::prelude::*;

#[aoc_generator(day4)]
fn gen(input: &str) -> (Vec<u32>, Vec<[[u32; 5]; 5]>) {
    let (calls, boards) = input.split_once("\n\n").unwrap();
    (
        calls.split(',').map(|call| call.parse().unwrap()).collect(),
        boards.split("\n\n")
            .map(|board| board.lines()
                .map(|line| line.trim()
                    .split_whitespace()
                    .map(|square| square.parse().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap()
                ).collect_vec()
                .try_into()
                .unwrap()
            ).collect(),
    )
}

fn board_score(board: &[[u32; 5]; 5], calls: &[u32]) -> Option<u32> {
    if (0..5).any(|row_idx|
        board.iter().all(|row| calls.contains(&row[row_idx]))
        || board[row_idx].iter().all(|cell| calls.contains(cell))
    ) {
        Some(
            board.iter().flatten().filter(|cell| !calls.contains(cell)).sum::<u32>()
                * calls[calls.len() - 1]
        )
    } else {
        None
    }
}

#[aoc(day4, part1)]
fn part1((calls, boards): &(Vec<u32>, Vec<[[u32; 5]; 5]>)) -> u32 {
    for call_idx in 4..calls.len() {
        for board in boards {
            if let Some(score) = board_score(board, &calls[..=call_idx]) {
                return score
            }
        }
    }
    panic!()
}

#[aoc(day4, part2)]
fn part2((calls, boards): &(Vec<u32>, Vec<[[u32; 5]; 5]>)) -> u32 {
    let mut uncalled_boards = boards.clone();
    for call_idx in 4..calls.len() {
        if uncalled_boards.len() == 1 {
            if let Some(score) = board_score(&uncalled_boards[0], &calls[..=call_idx]) {
                return score
            }
        } else {
            uncalled_boards.retain(|board| board_score(board, &calls[..=call_idx]).is_none());
        }
    }
    panic!()
}
