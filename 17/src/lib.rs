#![feature(test)]
extern crate test;

#[cfg(debug_assertions)]
const AREA: (Pair, Pair) = (Pair::new(20, -10), Pair::new(30, -5));

#[cfg(not(debug_assertions))]
const AREA: (Pair, Pair) = (Pair::new(70, -179), Pair::new(96, -124));

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Pair {
    x: i32,
    y: i32,
}

struct Probe {
    pub p: Pair,
    pub v: Pair,
}

impl Probe {
    fn step(&mut self) {
        self.p.x += self.v.x;
        self.p.y += self.v.y;
        self.v.x += -self.v.x.signum();
        self.v.y -= 1;
    }
}

impl Pair {
    pub const fn new(x: i32, y: i32) -> Self {
        Pair { x, y }
    }

    pub fn in_area(&self, (p1, p2): &(Self, Self)) -> bool {
        self.x >= p1.x && self.y >= p1.y && self.x <= p2.x && self.y <= p2.y
    }
}

fn simulate(v0: &Pair, max_y: &mut i32) -> bool {
    let mut probe = Probe {
        p: Pair::new(0, 0),
        v: *v0,
    };

    while !probe.p.in_area(&AREA) && probe.p.x <= AREA.1.x && probe.p.y >= AREA.0.y {
        *max_y = (*max_y).max(probe.p.y);
        probe.step();
    }

    probe.p.in_area(&AREA)
}

fn solve_part_one() -> i32 {
    let (min, max) = AREA;
    let mut max_y = i32::MIN;
    let mut answer = i32::MIN;

    for y in min.y..min.x * min.x {
        for x in 1..max.x + 1 {
            let pair = Pair::new(x, y);
            if simulate(&pair, &mut max_y) {
                answer = answer.max(max_y);
            }
        }
    }

    answer
}

fn solve_part_two() -> usize {
    let (min, max) = AREA;
    let mut answer = 0;

    for y in min.y..=min.x * min.x {
        for x in 1..max.x + 1 {
            let pair = Pair::new(x, y);
            if simulate(&pair, &mut 0) {
                answer += 1;
            }
        }
    }

    answer
}

pub fn solve() {
    let result = solve_part_one();
    println!("Part #1: {}", result);

    let result = solve_part_two();
    println!("Part #2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one();

        #[cfg(debug_assertions)]
        assert_eq!(result, 45);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 15931);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        b.iter(|| solve_part_one());
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two();

        #[cfg(debug_assertions)]
        assert_eq!(result, 112);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 2555);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        b.iter(|| solve_part_two());
    }
}
