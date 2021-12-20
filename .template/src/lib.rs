#![feature(test)]
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = u32;

fn solve_part_one(_input: &ParsedInput) -> u32 {
    todo!()
}

fn solve_part_two(_input: &ParsedInput) -> u32 {
    todo!()
}

fn parse_input(input: &str) -> ParsedInput {
    todo!()
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
