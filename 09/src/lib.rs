#![feature(test)]
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<Vec<u8>>;

const MASK: u8 = 0b0001_0000;

fn is_low_point(mat: &[Vec<u8>], x: usize, y: usize) -> bool {
    for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let row = x as isize + dx;
        let col = y as isize + dy;

        if row >= 0
            && row < mat.len() as isize
            && col >= 0
            && col < mat[0].len() as isize
            && mat[row as usize][col as usize] <= mat[x][y]
        {
            return false;
        }
    }

    true
}

fn find_low_points(mat: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();

    for (x, row) in mat.iter().enumerate() {
        for y in 0..row.len() {
            if is_low_point(mat, x, y) {
                low_points.push((x, y));
            }
        }
    }

    low_points
}

fn get_basin_length(mat: &[Vec<u8>], (x, y): (usize, usize)) -> Vec<Vec<u8>> {
    let mut ans = mat.to_owned();

    ans[x][y] |= MASK;

    for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let row = x as isize + dx;
        let col = y as isize + dy;

        if row >= 0
            && row < mat.len() as isize
            && col >= 0
            && col < mat[0].len() as isize
            && mat[row as usize][col as usize] != 9
            && mat[row as usize][col as usize] >= mat[x][y]
            && mat[row as usize][col as usize] & MASK == 0
        {
            ans = get_basin_length(&ans, (row as usize, col as usize));
        }
    }

    ans
}

fn solve_part_one(input: &ParsedInput) -> u32 {
    let mut risk_level = 0u32;

    for (x, y) in find_low_points(input) {
        risk_level += (input[x][y] + 1) as u32;
    }

    risk_level
}

fn solve_part_two(input: &ParsedInput) -> usize {
    let mut counts = vec![];

    for (x, y) in find_low_points(input) {
        let len = get_basin_length(input, (x, y));
        let count = len.iter().fold(0, |acc, row| {
            acc + row.iter().filter(|c| **c & MASK != 0).count()
        });
        counts.push(count);
    }

    counts.sort_unstable();

    counts.iter().rev().take(3).product()
}

fn parse_input(input: &str) -> ParsedInput {
    let mut parsed = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for num in line.chars() {
            row.push(num.to_digit(10).unwrap() as u8);
        }
        parsed.push(row);
    }

    parsed
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

#[allow(unused)]
fn dump_basin(mat: &[Vec<u8>]) {
    for row in mat {
        for num in row {
            if *num & MASK != 0 {
                print!("#");
            } else {
                print!("{}", num);
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 15);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 550);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_one(&input));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 1134);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 1100682);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
