#![feature(test)]
extern crate test;

use std::str::FromStr;

use bitflags::bitflags;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput<'input> = Vec<Vec<Vec<Segments>>>;

bitflags! {
    struct Segments: u8 {
        const A = 0b0000_0001;
        const B = 0b0000_0010;
        const C = 0b0000_0100;
        const D = 0b0000_1000;
        const E = 0b0001_0000;
        const F = 0b0010_0000;
        const G = 0b0100_0000;
    }
}

impl FromStr for Segments {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Segments::empty();
        for c in s.chars() {
            match c {
                'a' => res.insert(Segments::A),
                'b' => res.insert(Segments::B),
                'c' => res.insert(Segments::C),
                'd' => res.insert(Segments::D),
                'e' => res.insert(Segments::E),
                'f' => res.insert(Segments::F),
                'g' => res.insert(Segments::G),
                _ => return Err(format!("Invalid segment {}", c)),
            }
        }

        Ok(res)
    }
}

#[inline]
fn common_segments_count(a: Segments, b: Segments) -> u32 {
    a.intersection(b).bits().count_ones()
}

fn decode_signals(signals: &[Segments]) -> Vec<Segments> {
    let mut signals = signals.to_vec();
    let mut res = vec![Segments::empty(); 10];
    signals.sort_by_key(|a| a.bits().count_ones());

    for signal in signals {
        match signal.bits().count_ones() {
            // Unique length
            2 => res[1] = signal,
            3 => res[7] = signal,
            4 => res[4] = signal,
            7 => res[8] = signal,
            // 2, 3, 5 use 5 segments
            5 => {
                // 3 uses both of 1's segments
                if common_segments_count(signal, res[1]) == 2 {
                    res[3] = signal;
                    continue;
                }
                // 5 has 3 segments in common with 4
                if common_segments_count(signal, res[4]) == 3 {
                    res[5] = signal;
                    continue;
                }
                res[2] = signal;
            }
            // 0, 6, 9 use 6 segments
            6 => {
                // 6 uses only one segment in common with 1
                if common_segments_count(signal, res[1]) == 1 {
                    res[6] = signal;
                    continue;
                }
                // 9 has 4 segments in common with 4
                if common_segments_count(signal, res[4]) == 4 {
                    res[9] = signal;
                    continue;
                }
                res[0] = signal;
            }
            _ => unreachable!(),
        }
    }

    res
}

fn decode_output(decoded_signals: &[Segments], output: &[Segments]) -> usize {
    let mut res = 0;
    for (pos, &signal) in output.iter().rev().enumerate() {
        let factor = 10usize.pow(pos as u32);
        let num = decoded_signals.iter().position(|&x| x == signal).unwrap() * factor;
        res += num;
    }

    res
}

fn solve_part_one(input: &ParsedInput) -> u32 {
    let mut res = 0;
    for entry in input {
        if let [_, ref output] = entry[..] {
            res += output
                .iter()
                .filter(|&x| [2, 3, 4, 7].contains(&x.bits().count_ones()))
                .count() as u32;
        }
    }
    res
}

fn solve_part_two(input: &ParsedInput) -> usize {
    let mut sum = 0;
    for entry in input {
        if let [ref signals, ref output] = entry[..] {
            let decoded = decode_signals(signals);
            let res = decode_output(&decoded, output);
            sum += res;
        }
    }

    sum
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            line.split('|')
                .map(|s| {
                    s.trim()
                        .split_whitespace()
                        .map(|s| s.parse::<Segments>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect()
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
        assert_eq!(result, 26);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 344);
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
        assert_eq!(result, 61229);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 1048410);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
