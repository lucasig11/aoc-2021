#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

fn solve_part_one(input: &[&str]) -> u32 {
    let byte_length = input[0].len();
    let mut positioned: Vec<String> = vec![String::new(); byte_length];

    for line in input {
        for (idx, ch) in line.chars().enumerate() {
            if let Some(s) = positioned.get_mut(idx) {
                s.push(ch);
            } else {
                positioned.push(ch.to_string());
            }
        }
    }

    let (gamma_rate, epsilon_rate) = positioned.iter().fold(
        (String::new(), String::new()),
        |(mut gamma_rate, mut epsilon_rate), s| {
            let s = s.chars().collect::<Vec<_>>();
            let zero_count = s.iter().filter(|&&c| c == '0').count();
            // Here we have more zeroes than ones
            let (most_common, least_common) = if zero_count >= (input.len() / 2 + 1) {
                ('0', '1')
            } else {
                ('1', '0')
            };

            gamma_rate.push(most_common);
            epsilon_rate.push(least_common);

            (gamma_rate, epsilon_rate)
        },
    );

    let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap();

    gamma_rate * epsilon_rate
}

fn solve_part_two(_input: &[&str]) -> u32 {
    0
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
        let _result = solve_part_two(&parse_input(INPUT));
    }
}
