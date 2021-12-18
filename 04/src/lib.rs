#![feature(test)]
extern crate test;

use std::{fmt::Display, sync::Arc};

use threadpool::ThreadPool;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = (Vec<u32>, Vec<Board>);

const BOARD_SIZE: usize = 5;
const MARK_MASK: u32 = 0x1000;

fn is_marked(v: &u32) -> bool {
    v & MARK_MASK != 0
}

#[derive(Default, Clone, Debug)]
pub struct Board {
    pub values: Vec<u32>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            values: Vec::with_capacity(BOARD_SIZE * BOARD_SIZE),
        }
    }

    pub fn push(&mut self, row: &[u32]) {
        self.values.extend(row);
    }

    pub fn mark(&mut self, num: u32) {
        self.values.iter_mut().for_each(|v| {
            if *v == num {
                *v |= MARK_MASK;
            }
        });
    }

    pub fn rows(&self) -> impl Iterator<Item = &[u32]> {
        self.values.chunks(BOARD_SIZE)
    }

    pub fn columns(&self) -> Vec<Vec<u32>> {
        self.values
            .iter()
            .enumerate()
            .fold(vec![vec![]; 5], |mut acc, (idx, &val)| {
                acc[idx % 5].push(val);
                acc
            })
    }

    pub fn won(&self) -> bool {
        self.rows().any(|row| row.iter().all(is_marked))
            || self.columns().iter().any(|col| col.iter().all(is_marked))
    }

    pub fn get_points(&self) -> u32 {
        self.values.iter().filter(|&v| !is_marked(v)).sum::<u32>()
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        let map = |&v| if is_marked(&v) { v & !MARK_MASK } else { v };
        self.values.iter().map(map).collect::<Vec<_>>()
            == other.values.iter().map(map).collect::<Vec<_>>()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.rows() {
            for val in row {
                if is_marked(val) {
                    write!(f, "{:>2} ", val & !MARK_MASK)?;
                } else {
                    write!(f, "{:>2} ", val)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn get_winning_board(nums: &[u32], boards: &[Board]) -> (usize, Board) {
    let mut boards = boards.to_vec();
    for (idx, num) in nums.iter().enumerate() {
        for board in boards.iter_mut() {
            board.mark(*num);
            if board.won() {
                return (idx, board.clone());
            }
        }
    }

    unreachable!("Nobody won :(");
}

fn solve_part_one((nums, boards): &ParsedInput) -> u32 {
    let (last_round, winning_board) = get_winning_board(nums, boards);

    winning_board.get_points() * nums[last_round]
}

fn solve_part_two((nums, boards): &ParsedInput) -> u32 {
    let workers_cnt = boards.len().min(12);
    let chunk_sz = boards.len() / workers_cnt;
    let pool = ThreadPool::new(workers_cnt);
    let nums = Arc::new(nums.clone());
    let (tx, rx) = std::sync::mpsc::channel();

    for chunk in boards.chunks(chunk_sz) {
        let nums = Arc::clone(&nums);
        let chunk = Arc::new(chunk.to_vec());
        let tx = tx.clone();

        pool.execute(move || {
            let mut boards = (*chunk).clone();

            let (last_round, last_winner) = loop {
                let (last_round, winner) = get_winning_board(&nums, &boards);

                if boards.len() == 1 {
                    break (last_round, winner);
                }

                boards.retain(|b| b != &winner);
            };

            tx.send((last_round, last_winner)).unwrap();
            drop(tx);
        });
    }
    drop(tx);

    let (mut last_round, mut last_winner) = (0, Board::default());

    while let Ok((last, winning_board)) = rx.recv() {
        if last > last_round {
            last_round = last;
            last_winner = winning_board;
        }
    }

    pool.join();

    nums[last_round] * last_winner.get_points()
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

fn parse_input(input: &str) -> ParsedInput {
    let mut lines = input.lines();
    let nums: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let boards = lines.fold(Vec::new(), |mut boards, line| {
        if line.is_empty() {
            boards.push(Board::new());
            return boards;
        }

        let row = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        boards.last_mut().unwrap().push(&row);

        boards
    });

    (nums, boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 4512);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 27027);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        b.iter(|| solve_part_one(&parse_input(INPUT)));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 1924);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 36975);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        b.iter(|| solve_part_two(&parse_input(INPUT)));
    }
}
