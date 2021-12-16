use std::str::FromStr;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

pub enum Instruction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ix = s.split_whitespace().collect::<Vec<_>>();

        if let [ix, dist] = ix[..] {
            return match ix {
                "forward" => Ok(Instruction::Forward(dist.parse::<u32>()?)),
                "up" => Ok(Instruction::Up(dist.parse::<u32>()?)),
                "down" => Ok(Instruction::Down(dist.parse::<u32>()?)),
                _ => unreachable!(),
            };
        }

        Err(From::from("Invalid instruction"))
    }
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result: u32 = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result: u32 = solve_part_two(&input);
    println!("Part #2: {}", result);
}

fn solve_part_one(input: &[Instruction]) -> u32 {
    let x_pos = input.iter().fold(0, |acc, ix| match ix {
        Instruction::Forward(dist) => acc + dist,
        _ => acc,
    });

    let depth = input.iter().fold(0, |acc, ix| match ix {
        Instruction::Up(dist) => acc - dist,
        Instruction::Down(dist) => acc + dist,
        _ => acc,
    });

    depth * x_pos
}

fn solve_part_two(input: &[Instruction]) -> u32 {
    let mut aim = 0;
    let mut x_pos = 0;
    let mut depth = 0;

    for ix in input {
        match ix {
            Instruction::Forward(dist) => {
                x_pos += dist;
                depth += aim * dist;
            }
            Instruction::Up(dist) => {
                aim -= dist;
            }
            Instruction::Down(dist) => {
                aim += dist;
            }
        }
    }

    depth * x_pos
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        assert_eq!(result, 150);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&parse_input(INPUT));

        assert_eq!(result, 900);
    }
}
