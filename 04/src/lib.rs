#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = (Vec<u32>, Boards);
type Boards = Vec<Board>;

#[derive(Default, Clone, Debug)]
pub struct Board {
    pub rows: Vec<Vec<u32>>,
    pub cols: Vec<Vec<u32>>,
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, row: Vec<u32>) {
        if self.cols.len() < row.len() {
            self.cols.resize(row.len(), Vec::new());
        }
        for (i, &v) in row.iter().enumerate() {
            self.cols.get_mut(i).unwrap().push(v);
        }
        self.rows.push(row);
    }

    pub fn mark(&mut self, num: u32) {
        self.rows = self
            .rows
            .iter()
            .map(|row| row.iter().filter(|v| **v != num).cloned().collect())
            .collect();

        self.cols = self
            .cols
            .iter()
            .map(|col| col.iter().filter(|v| **v != num).cloned().collect())
            .collect();
    }

    pub fn won(&self) -> bool {
        self.rows.iter().any(|row| row.is_empty()) || self.cols.iter().any(|col| col.is_empty())
    }
}

fn get_winning_board(nums: &[u32], boards: &mut Boards) -> (u32, Board) {
    for num in nums {
        for board in boards.iter_mut() {
            board.mark(*num);
            if board.won() {
                return (*num, board.clone());
            }
        }
    }

    unreachable!()
}

fn get_board_points(board: &Board) -> u32 {
    board
        .rows
        .iter()
        .map(|row| row.iter().sum::<u32>())
        .sum::<u32>()
}

fn solve_part_one((nums, boards): &ParsedInput) -> u32 {
    let mut boards = boards.clone();
    let (last_num, winning_board) = get_winning_board(nums, &mut boards);

    get_board_points(&winning_board) * last_num
}

fn solve_part_two((nums, boards): &ParsedInput) -> u32 {
    let mut boards = boards.clone();
    let mut last_num;
    loop {
        let (last, _) = get_winning_board(nums, &mut boards);

        last_num = last;

        if boards.len() == 1 {
            break;
        }

        boards.retain(|board| !board.won());
    }

    print_boards(&boards);

    get_board_points(&boards[0]) * last_num
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

fn parse_input(input: &str) -> ParsedInput {
    let mut lines = input.lines();
    let nums: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let boards = lines.fold(vec![], |mut acc, line| {
        if line.is_empty() {
            acc.push(Board::new());
            return acc;
        }
        let row = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        acc.last_mut().unwrap().push(row);
        acc
    });

    (nums, boards)
}

#[allow(dead_code)]
fn print_boards(boards: &Boards) {
    for board in boards {
        for row in &board.rows {
            for num in row {
                print!("{:>2} ", num);
            }
            println!();
        }
        println!();
    }
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
