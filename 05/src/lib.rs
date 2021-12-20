#![feature(test)]

use std::{fmt::Display, ops::RangeInclusive};
extern crate test;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

// (All coordinates parsed, Maximum x and y points)
type ParsedInput = (Vec<Line>, Coord);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Line {
    pub start: Coord,
    pub end: Coord,
}

struct Diagram {
    lines: Vec<Vec<u8>>,
    // Number of points visited at least twice
    visited_twice_cnt: usize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y,)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

impl Display for Diagram {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "  ")?;
        for x in 0..self.lines[0].len() {
            write!(f, "{:2} ", x)?;
        }
        writeln!(f)?;
        for (idx, row) in self.lines.iter().enumerate() {
            write!(f, "{} ", idx)?;
            for &cell in row {
                if cell == 0 {
                    write!(f, " . ")?;
                } else {
                    write!(f, "{:>2} ", cell)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Coord { x, y }
    }
}

impl Line {
    pub fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    // Creates an inclusive range from two positions
    fn range_from(p1: isize, p2: isize) -> RangeInclusive<usize> {
        let min = p1.min(p2);
        let max = p1.max(p2);

        min as usize..=max as usize
    }

    pub fn as_ranges(&self) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
        let x_range = Self::range_from(self.start.x, self.end.x);
        let y_range = Self::range_from(self.start.y, self.end.y);

        (x_range, y_range)
    }

    pub fn displacement(&self) -> Coord {
        Coord {
            x: self.end.x - self.start.x,
            y: self.end.y - self.start.y,
        }
    }
}

impl Diagram {
    pub fn new(max_coord: &Coord) -> Self {
        Diagram {
            lines: vec![vec![0; max_coord.x as usize + 1]; max_coord.y as usize + 1],
            visited_twice_cnt: 0,
        }
    }

    pub fn visit(&mut self, line: &Line) {
        let (x_range, y_range) = line.as_ranges();

        if !line.is_straight() {
            return self.visit_angled(line);
        }

        self.lines[y_range].iter_mut().for_each(|row| {
            for x in row[x_range.clone()].iter_mut() {
                *x += 1;

                if *x == 2 {
                    self.visited_twice_cnt += 1;
                }
            }
        });
    }

    fn visit_point(&mut self, x: usize, y: usize) {
        self.lines[y][x] += 1;

        if self.lines[y][x] == 2 {
            self.visited_twice_cnt += 1;
        }
    }

    pub fn visit_angled(&mut self, line: &Line) {
        let (mut x_range, y_range) = line.as_ranges();
        let Coord { x: dx, y: dy } = line.displacement();

        if dx < 0 && dy > 0 || dx > 0 && dy < 0 {
            let mut x_range = x_range.rev();
            for i in y_range {
                self.visit_point(x_range.next().unwrap(), i);
            }
            return;
        }

        for i in y_range {
            self.visit_point(x_range.next().unwrap(), i);
        }
    }

    pub fn count_visited_twice(&self) -> usize {
        self.visited_twice_cnt
    }
}

fn solve_part_one((lines, max_coord): &ParsedInput) -> usize {
    let mut diagram = Diagram::new(max_coord);

    let lines = lines.iter().filter(|l| l.is_straight());

    for line in lines {
        diagram.visit(line);
    }

    // println!("{}", diagram);

    diagram.count_visited_twice()
}

fn solve_part_two((lines, max_coord): &ParsedInput) -> usize {
    let mut diagram = Diagram::new(max_coord);

    for line in lines {
        diagram.visit(line);
    }

    // println!("{}", diagram);

    diagram.count_visited_twice()
}

fn parse_input(input: &str) -> ParsedInput {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut res = vec![];
    for line in input.lines() {
        if let [ref start, ref end] = line
            .split("->")
            .map(|s| {
                s.trim()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()[..]
        {
            let start = Coord::new(start[0], start[1]);
            let end = Coord::new(end[0], end[1]);

            let line_max_x = end.x.max(start.x);
            let line_max_y = end.y.max(start.y);

            max_x = max_x.max(line_max_x);
            max_y = max_y.max(line_max_y);

            res.push(Line { start, end });
        }
    }

    let max_coord = Coord::new(max_x, max_y);

    (res, max_coord)
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
        assert_eq!(result, 5);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 7644);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        b.iter(|| solve_part_one(&parse_input(INPUT)));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 12);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 18627);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        b.iter(|| solve_part_two(&parse_input(INPUT)));
    }
}
