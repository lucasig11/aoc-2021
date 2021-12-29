#![feature(test)]
#![feature(int_abs_diff)]
extern crate test;

use std::io::Write;
use std::{cmp::Reverse, collections::BinaryHeap};

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput = Vec<Vec<usize>>;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub node: usize,
    pub weight: usize,
}

// Dijkstra's algorithm
fn find_shortest_path(adj_list: &[Vec<Edge>]) -> Option<usize> {
    // Distance from source node.
    let mut dist = vec![usize::MAX; adj_list.len()];

    // Priority queue of nodes to visit.
    let mut queue = BinaryHeap::new();

    dist[0] = 0;
    queue.push(Reverse((0, 0)));

    while let Some(Reverse((cost, position))) = queue.pop() {
        // We found the bottom-right node.
        if position == adj_list.len() - 1 {
            return Some(cost);
        }

        // Not worth visiting.
        if cost > dist[position] {
            continue;
        }

        // Visit all adjacent nodes.
        for edge in &adj_list[position] {
            let (next_cost, next_pos) = (cost + edge.weight, edge.node);

            if next_cost < dist[next_pos] {
                queue.push(Reverse((next_cost, next_pos)));
                dist[next_pos] = next_cost;
            }
        }
    }

    None
}

fn gen_adj_list(map: &[Vec<usize>]) -> Vec<Vec<Edge>> {
    map.iter()
        .enumerate()
        .fold(vec![], |mut adj_list, (y, row)| {
            for x in 0..row.len() {
                let edges =
                    [(0, -1), (-1, 0), (0, 1), (1, 0)]
                        .iter()
                        .fold(vec![], |mut acc, (dx, dy)| {
                            let (x, y) = (x as i32 + dx, y as i32 + dy);
                            if x >= 0
                                && y >= 0
                                && (x as usize) < row.len()
                                && (y as usize) < row.len()
                            {
                                let pos = x as usize + ((y as usize) * row.len());
                                acc.push(Edge {
                                    node: pos as usize,
                                    weight: map[y as usize][x as usize],
                                });
                            }

                            acc
                        });
                adj_list.push(edges);
            }
            adj_list
        })
}

fn expand_map(map: &[Vec<usize>], factor: usize) -> Vec<Vec<usize>> {
    let mut new_map = vec![vec![0; map.len() * factor]; map.len() * factor];

    for y in 0..map.len() {
        for x in 0..map.len() {
            let weight = map[y][x];
            for dy in 0..factor {
                for dx in 0..factor {
                    let weight = if dx != 0 || dy != 0 {
                        (((weight + dy + dx) - 1) % 9) + 1
                    } else {
                        weight
                    };
                    let dy = y + map.len() * dy;
                    let dx = x + map.len() * dx;
                    new_map[dy][dx] = weight;
                }
            }
            new_map[y][x] = map[y][x];
        }
    }

    new_map
}

fn solve_part_one(input: &ParsedInput) -> usize {
    let adj_list = gen_adj_list(input);
    // dump_edges(&adj_list);
    find_shortest_path(&adj_list).unwrap()
}

fn solve_part_two(input: &ParsedInput) -> usize {
    let map = expand_map(input, 5);
    let adj_list = gen_adj_list(&map);
    // dump_map(&map, 10);
    // dump_edges(&adj_list);
    find_shortest_path(&adj_list).unwrap()
}

fn parse_input(input: &str) -> ParsedInput {
    input.lines().fold(vec![], |mut acc, line| {
        acc.push(
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>(),
        );
        acc
    })
}

#[allow(unused)]
fn dump_map(map: &[Vec<usize>], len: usize) {
    for (n, line) in map.iter().enumerate() {
        if n % len == 0 {
            println!();
        }
        for (i, cell) in line.iter().enumerate() {
            if i % len == 0 {
                print!(" ");
            }
            print!("{}", cell);
        }
        println!();
    }
}

#[allow(unused)]
fn dump_edges(adj_list: &[Vec<Edge>]) {
    let mut fd = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("edges.dot")
        .unwrap();

    writeln!(fd, "digraph {{").unwrap();
    writeln!(fd, "\tnode [shape=box];").unwrap();
    for (idx, edges) in adj_list.iter().enumerate() {
        writeln!(fd, "\t{} [ label = \"{}\" ];", idx, idx).unwrap();
        for edge in edges {
            writeln!(
                fd,
                "\t{} -> {} [ label = \"{}\" ];",
                idx, edge.node, edge.weight
            )
            .unwrap();
        }
    }
    writeln!(fd, "}}").unwrap();
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
        assert_eq!(result, 40);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 707);
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
        assert_eq!(result, 315);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 2942);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
