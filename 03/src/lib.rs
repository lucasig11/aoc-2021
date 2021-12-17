use std::cmp::Ordering;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

fn other(ch: char) -> char {
    match ch {
        '1' => '0',
        '0' => '1',
        _ => unreachable!(),
    }
}

fn get_most_common_bit(repr: &str) -> Option<char> {
    let count = |n| repr.chars().filter(|&c| c == n).count();

    let ones_count = count('1');
    let zeros_count = count('0');

    match ones_count.cmp(&zeros_count) {
        Ordering::Equal => None,
        Ordering::Less => Some('0'),
        Ordering::Greater => Some('1'),
    }
}

fn group_by_position(input: &[&str]) -> Vec<String> {
    let num_positions = input[0].len();
    let mut sorted = vec![String::new(); num_positions];
    // Group characters by position
    for line in input {
        for (idx, ch) in line.chars().enumerate() {
            if let Some(s) = sorted.get_mut(idx) {
                s.push(ch);
            } else {
                sorted.push(String::from(ch));
            }
        }
    }

    sorted
}

fn gamma_epsilon(positioned: &[String]) -> (u32, u32) {
    let (gamma_rate, epsilon_rate) = positioned.iter().fold(
        (String::new(), String::new()),
        |(mut gamma_rate, mut epsilon_rate), s| {
            let most_common = get_most_common_bit(s).unwrap();
            gamma_rate.push(most_common);
            epsilon_rate.push(other(most_common));
            (gamma_rate, epsilon_rate)
        },
    );

    (
        u32::from_str_radix(&gamma_rate, 2).unwrap(),
        u32::from_str_radix(&epsilon_rate, 2).unwrap(),
    )
}

fn get_rate(bytes: &[&str], tiebraker: char) -> u32 {
    let mut rate = Vec::from(bytes);
    let byte_length = bytes[0].len();
    let use_least_common_bit = tiebraker == '0';

    for col in 0..byte_length {
        if rate.len() == 1 {
            break;
        }

        let positioned = group_by_position(&rate);
        let most_common_bit = get_most_common_bit(&positioned[col]);

        let most_common_bit = if use_least_common_bit {
            most_common_bit.map(other)
        } else {
            most_common_bit
        };

        rate = rate
            .into_iter()
            .filter(|s| s.chars().nth(col).unwrap() == most_common_bit.unwrap_or(tiebraker))
            .collect();
    }

    u32::from_str_radix(&rate.join(""), 2).unwrap()
}

fn solve_part_one(input: &[&str]) -> u32 {
    let positioned = group_by_position(input);
    let (gamma_rate, epsilon_rate) = gamma_epsilon(&positioned);

    gamma_rate * epsilon_rate
}

fn solve_part_two(input: &[&str]) -> u32 {
    let oxygen_rate = get_rate(input, '1');
    let co2_rate = get_rate(input, '0');
    oxygen_rate * co2_rate
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
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

        assert_eq!(result, 198);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&parse_input(INPUT));

        assert_eq!(result, 230);
    }
}
