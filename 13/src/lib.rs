#![feature(test)]
#![feature(int_abs_diff)]
extern crate test;
use std::collections::HashSet;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

#[derive(Clone, Copy)]
pub enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Point(pub usize, pub usize);

type ParsedInput = (HashSet<Point>, Vec<Fold>);

fn fold_at(points: &HashSet<Point>, fold: Fold) -> HashSet<Point> {
    points
        .iter()
        .map(|Point(x, y)| match fold {
            Fold::X(f) => Point(f.abs_diff(f.abs_diff(*x)), *y),
            Fold::Y(f) => Point(*x, f.abs_diff(f.abs_diff(*y))),
        })
        .collect()
}

fn solve_part_one((points, folds): &ParsedInput) -> usize {
    fold_at(points, *folds.first().unwrap()).len()
}

fn solve_part_two((points, folds): &ParsedInput) -> usize {
    let points = folds
        .iter()
        .fold(points.clone(), |acc, fold| fold_at(&acc, *fold));

    dump_sheet(&points);

    0
}

fn parse_input(input: &str) -> ParsedInput {
    let mut lines = input.lines();

    let points = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Point(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let folds = lines
        .map(|l| {
            let (axis, num) = l.split_once('=').unwrap();
            let num = num.parse().unwrap();
            match axis.chars().last().unwrap() {
                'x' => Fold::X(num),
                'y' => Fold::Y(num),
                _ => panic!("Invalid axis"),
            }
        })
        .collect();

    (points, folds)
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    println!("Part #2:");
    solve_part_two(&input);
}

fn dump_sheet(points: &HashSet<Point>) {
    let (w, h) = points
        .iter()
        .fold((0, 0), |(w, h), Point(x, y)| (*x.max(&w), *y.max(&h)));

    let mut grid = vec![vec![false; w + 1]; h + 1];

    for Point(x, y) in points {
        grid[*y][*x] = true;
    }

    for row in grid {
        for cell in row {
            print!("{}", if cell { '▓' } else { '░' });
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 17);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 755);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_one(&input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
