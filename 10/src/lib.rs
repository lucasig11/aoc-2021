#![feature(test)]

use std::iter::Peekable;
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<Vec<char>>;

type ParseResult<T> = Result<T, Error>;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    CorruptedChunk { at: char },
    UnterminatedChunk { expected: char },
}

fn is_closed(&tk: &char) -> bool {
    tk == ')' || tk == ']' || tk == '}' || tk == '>'
}

fn closed(tk: &char) -> char {
    match tk {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Invalid delimiter {}", tk),
    }
}

fn parse_chunks(stream: &mut Peekable<std::slice::Iter<'_, char>>) -> ParseResult<()> {
    if let Some(tk) = stream.next() {
        let delim = closed(tk);

        // Empty chunk.
        if matches!(stream.peek(), Some(&&cl) if cl == delim) {
            stream.next();
            return Ok(());
        }

        // Consume inner chunks.
        while matches!(stream.peek(), Some(tk) if !is_closed(tk)) {
            parse_chunks(stream)?;
        }

        // Get the closing token.
        if let Some(closing) = stream.next() {
            if is_closed(closing) && closing != &delim {
                return Err(Error::CorruptedChunk { at: *closing });
            }

            if let Some(next) = stream.peek() {
                if !is_closed(next) {
                    return parse_chunks(stream);
                }
            }

            return Ok(());
        }

        return Err(Error::UnterminatedChunk { expected: delim });
    }

    Ok(())
}

fn solve_part_one(input: &ParsedInput) -> usize {
    let mut score = 0;
    for line in input {
        let mut stream = line.iter().peekable();
        if let Err(Error::CorruptedChunk { at: found }) = parse_chunks(&mut stream) {
            match found {
                ')' => score += 3,
                ']' => score += 57,
                '}' => score += 1197,
                '>' => score += 25137,
                _ => unreachable!(),
            }
        }
    }

    score
}

fn solve_part_two(input: &ParsedInput) -> usize {
    let mut input = input.clone();
    let mut scores = Vec::new();

    for line in input.iter_mut() {
        let mut score = 0;
        while let Err(Error::UnterminatedChunk { expected }) =
            parse_chunks(&mut line.iter().peekable())
        {
            score *= 5;
            score += match expected {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            };

            line.push(expected);
        }

        if score > 0 {
            scores.push(score);
        }
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn parse_input(input: &str) -> ParsedInput {
    input.lines().map(|l| l.chars().collect()).collect()
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
        assert_eq!(result, 26397);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 374061);
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
        assert_eq!(result, 288957);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 2116639949);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
