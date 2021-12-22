#![feature(test)]
#![feature(int_abs_diff)]

extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
pub(crate) const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<u32>;

fn median(vec: &[u32]) -> u32 {
    let mut vec = vec.to_vec();
    vec.sort_unstable();

    match vec.len() % 2 {
        0 => (vec[vec.len() / 2 - 1] + vec[vec.len() / 2]) / 2,
        _ => vec[vec.len() / 2],
    }
}

fn mean(vec: &[u32]) -> f64 {
    vec.iter().sum::<u32>() as f64 / vec.len() as f64
}

fn solve_part_one(input: &ParsedInput) -> u32 {
    let pos = median(input);

    input
        .iter()
        .fold(0, |fuel, &crab_pos| fuel + u32::abs_diff(crab_pos, pos))
}

fn solve_part_two(input: &ParsedInput) -> u32 {
    let pos = mean(input);

    // debug and release sets require different rounding
    #[cfg(debug_assertions)]
    let pos = f64::ceil(pos) as u32;

    #[cfg(not(debug_assertions))]
    let pos = f64::floor(pos) as u32;

    input.iter().fold(0, |fuel, &crab_pos| {
        let diff = u32::abs_diff(crab_pos, pos);
        fuel + (diff * (diff + 1) / 2) as u32
    })
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect::<Vec<_>>()
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 37);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 347011);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        b.iter(|| solve_part_one(&parse_input(INPUT)));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 168);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 98363777);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        b.iter(|| solve_part_two(&parse_input(INPUT)));
    }
}
