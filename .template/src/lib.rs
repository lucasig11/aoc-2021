#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

fn solve_part_one(_input: &[u32]) -> u32 {
    todo!()
}

fn solve_part_two(_input: &[u32]) -> u32 {
    todo!()
}

fn parse_input(input: &str) -> Vec<u32> {
    let mut vec = Vec::new();
    for line in input.lines() {
        vec.push(line.parse().unwrap());
    }
    vec
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
        let _result = solve_part_one(&parse_input(INPUT));
    }

    #[test]
    fn test_part_two() {
        let _result = solve_part_two(&parse_input(INPUT));
    }
}
