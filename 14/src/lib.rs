#![feature(test)]

use std::{
    collections::HashMap,
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Sub, SubAssign},
};

extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type Rules = HashMap<u16, u8>;

type FrequencyMap = HashMap<u16, usize>;

type ParsedInput = (Vec<u8>, Rules);

fn pack_char_bytes<T: IntoIterator<Item = u8>>(chars: T) -> u16 {
    chars.into_iter().fold(0, |acc, ch| acc << 8 | (ch as u16))
}

fn unpack_chars(bytes: u16) -> (u8, u8) {
    ((bytes >> 8) as u8, (bytes & 0xff) as u8)
}

fn gen_frequency(s: &[u8]) -> FrequencyMap {
    let mut freq_map = HashMap::new();

    for pair in s.windows(2) {
        let pair = pack_char_bytes(pair.iter().copied());
        let freq = freq_map.entry(pair).or_insert(0);
        *freq += 1;
    }

    freq_map
}

fn replace(freq_map: &mut FrequencyMap, rules: &Rules) {
    let copy = freq_map.clone();
    for pair in copy.keys() {
        if let Some(ch) = rules.get(pair) {
            let (start, end) = unpack_chars(*pair);
            let freq = *copy.get(pair).unwrap();
            *freq_map.get_mut(pair).unwrap() -= freq;
            *freq_map.entry(pack_char_bytes([start, *ch])).or_insert(0) += freq;
            *freq_map.entry(pack_char_bytes([*ch, end])).or_insert(0) += freq;
        }
    }
}

fn answer((template, rules): &ParsedInput, n_iter: usize) -> usize {
    let mut freq_map = gen_frequency(template);

    for _ in 0..n_iter {
        replace(&mut freq_map, rules);
    }

    let mut count = [0; 26];

    for (pair, frequency) in freq_map {
        let (start, end) = unpack_chars(pair);
        count[(start - b'A') as usize] += frequency;
        count[(end - b'A') as usize] += frequency;
    }

    count[(template[0] - b'A') as usize] += 1;
    count[(template[template.len() - 1] - b'A') as usize] += 1;

    let (max, min) = count
        .iter()
        .filter(|n| **n > 0)
        .fold((0, usize::MAX), |(max, min), &count| {
            (max.max(count / 2), min.min(count / 2))
        });

    max - min
}

fn solve_part_one(input: &ParsedInput) -> usize {
    answer(input, 10)
}

fn solve_part_two(input: &ParsedInput) -> usize {
    answer(input, 40)
}

fn parse_input(input: &str) -> ParsedInput {
    let mut lines = input.lines();

    let initial_polymer = lines.by_ref().take(1).fold(Vec::new(), |mut acc, line| {
        acc.extend(line.bytes());
        acc
    });

    lines.next();

    let rules = lines
        .map(|l| {
            let (pair, ch) = l.split_once("->").unwrap();
            let pair = pack_char_bytes(pair.trim().bytes());
            let ch = ch.trim().bytes().next().unwrap();
            (pair, ch)
        })
        .collect();

    (initial_polymer, rules)
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
        assert_eq!(result, 1588);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 2509);
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
        assert_eq!(result, 2188189693529);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 2827627697643);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
