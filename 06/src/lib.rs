#![feature(test)]

extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = [u64];

fn update_generation(state: &[u64]) -> Vec<u64> {
    let mut new_state = state.to_owned();

    let zeroes = state[0];

    for i in (0..state.len()).rev() {
        if let 1..=8 = i {
            new_state[i - 1] = state[i];
            continue;
        }

        // Reset internal timers
        new_state[6] += zeroes;

        // Create a new lanternfish
        new_state[8] = zeroes;
    }

    new_state
}

fn get_population_count(days: u64, initial_state: &[u64]) -> u64 {
    let mut initial_state = initial_state.to_vec();

    for _ in 0..days {
        initial_state = update_generation(&initial_state);
    }

    initial_state.iter().sum()
}

fn solve_part_one(input: &ParsedInput) -> u64 {
    get_population_count(80, input)
}

fn solve_part_two(input: &ParsedInput) -> u64 {
    get_population_count(256, input)
}

fn parse_input(input: &str) -> [u64; 9] {
    input.split(',').fold([0; 9], |mut acc, x| {
        let x: usize = x.trim().parse().unwrap();
        acc[x] += 1;
        acc
    })
}

#[allow(unused)]
fn dump_population(state: &[u64]) {
    for (idx, &v) in state.iter().enumerate() {
        for _ in 0..v {
            print!("{}, ", idx);
        }
    }
    println!();
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
        assert_eq!(result, 5934);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 353274);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        b.iter(|| solve_part_one(&parse_input(INPUT)));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 26984457539);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 1609314870967);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        b.iter(|| solve_part_two(&parse_input(INPUT)));
    }
}
