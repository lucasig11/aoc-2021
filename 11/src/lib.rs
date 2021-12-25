#![feature(test)]
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = [[u8; 10]; 10];

const MASK: u8 = 0b0001_0000;

fn flash(mat: &mut ParsedInput, x: usize, y: usize) {
    mat[y][x] |= MASK;

    for &(dx, dy) in &[
        (0, 1),
        (1, 0),
        (0, -1),
        (1, 1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ] {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if (0..10).contains(&nx)
            && (0..10).contains(&ny)
            && mat[ny as usize][nx as usize] & MASK == 0
        {
            if mat[ny as usize][nx as usize] == 9 {
                flash(mat, nx as usize, ny as usize);
                continue;
            }
            mat[ny as usize][nx as usize] += 1;
        }
    }
}

fn step(mat: &mut ParsedInput) -> usize {
    let mut flashes = 0;

    for y in 0..10 {
        for x in 0..10 {
            if mat[y][x] == 9 {
                flash(mat, x, y);
                continue;
            }

            mat[y][x] += 1;
        }
    }

    mat.iter_mut().for_each(|row| {
        for cell in row.iter_mut() {
            if *cell & MASK != 0 {
                *cell = 0;
                flashes += 1;
            }
        }
    });

    flashes
}

fn solve_part_one(input: &ParsedInput) -> usize {
    let mut initial_state = *input;

    (0..100).fold(0, |acc, _| acc + step(&mut initial_state))
}

fn solve_part_two(input: &ParsedInput) -> usize {
    let mut initial_state = *input;
    let mut flashes = 0;
    let mut count = 0;

    while flashes != 100 {
        flashes = step(&mut initial_state);
        count += 1;
    }

    count
}

fn parse_input(input: &str) -> ParsedInput {
    let mut energy_levels: [[u8; 10]; 10] = [[0; 10]; 10];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            energy_levels[y][x] = c.to_digit(10).unwrap() as u8;
        }
    }
    energy_levels
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

pub fn dump(mat: [[u8; 10]; 10]) {
    for y in &mat {
        for x in y {
            if x == &0 {
                print!(" .");
            } else {
                print!("{:>2}", x);
            }
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
        assert_eq!(result, 1656);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 1615);
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
        assert_eq!(result, 1924);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 249);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
