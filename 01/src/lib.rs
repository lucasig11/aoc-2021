#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

fn solve_part_one(input: &[u32]) -> u32 {
    let mut sum = 0;
    for (idx, i) in input.iter().enumerate() {
        if let Some(j) = input.get(idx + 1) {
            if i < j {
                sum += 1;
            }
        }
    }

    sum
}

fn solve_part_two(input: &[u32]) -> u32 {
    let win_len = 3;
    let mut sums = Vec::new();
    let mut start = 0;
    loop {
        let end = start + win_len;
        if end > input.len() {
            break;
        }
        let win = &input[start..end];
        sums.push(win.iter().sum::<u32>());
        start += 1;
    }
    solve_part_one(&sums)
}

fn parse_input(input: &str) -> Vec<u32> {
    let mut vec = Vec::new();
    for line in input.lines() {
        vec.push(line.parse().unwrap());
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&parse_input(INPUT));

        assert_eq!(result, 5);
    }
}
