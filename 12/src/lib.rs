#![feature(test)]
extern crate test;

use std::{collections::HashMap, fs::OpenOptions, io::Write};

use petgraph::{
    dot::{Config, Dot},
    prelude::*,
};

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../sample.TXT");

#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.TXT");

type ParsedInput<'i> = Graph<&'i str, usize, Undirected>;

fn is_big_cave(cave: &str) -> bool {
    cave.to_ascii_uppercase() == cave
}

fn count_all_paths_rec(
    graph: &ParsedInput,
    start: NodeIndex,
    end: NodeIndex,
    limit: u8,
    buf: &mut Vec<NodeIndex>,
    visited: &mut [u8],
    count: &mut usize,
) {
    if !is_big_cave(graph[start]) {
        if graph[start] == "start" {
            visited[start.index()] = limit;
        } else {
            visited[start.index()] += 1;
        }
    }

    buf.push(start);

    if start == end {
        // New path completed.
        *count += 1;
    } else {
        for neighbor in graph.neighbors(start) {
            if visited[neighbor.index()] < limit {
                if visited[neighbor.index()] > 0 && visited[1..].contains(&limit) {
                    continue;
                }
                count_all_paths_rec(graph, neighbor, end, limit, buf, visited, count);
            }
        }
    }

    if !is_big_cave(graph[start]) {
        visited[start.index()] -= 1;
    }

    buf.pop();
}

fn count_all_paths(graph: &ParsedInput, limit: u8) -> usize {
    let mut buf = Vec::new();
    let mut visited = vec![0; graph.node_count()];
    let mut count = 0;

    count_all_paths_rec(
        graph,
        0.into(),
        1.into(),
        limit,
        &mut buf,
        &mut visited,
        &mut count,
    );

    count
}

fn solve_part_one(input: &ParsedInput) -> usize {
    count_all_paths(input, 1)
}

fn solve_part_two(input: &ParsedInput) -> usize {
    count_all_paths(input, 2)
}

fn parse_input(input: &str) -> ParsedInput {
    let mut graph = Graph::new_undirected();
    let mut nodes = HashMap::new();

    // Determine the start and end nodes
    nodes.insert("start", graph.add_node("start"));
    nodes.insert("end", graph.add_node("end"));

    for line in input.lines() {
        if let [p1, p2] = line.split('-').collect::<Vec<_>>()[..] {
            let p1 = *nodes.entry(p1).or_insert_with(|| graph.add_node(p1));
            let p2 = *nodes.entry(p2).or_insert_with(|| graph.add_node(p2));
            graph.add_edge(p1, p2, 1);
        }
    }

    // dump_dotfile(&graph, "target/graph.dot");

    graph
}

pub fn solve() {
    let input = parse_input(INPUT);

    let result = solve_part_one(&input);
    println!("Part #1: {}", result);

    let result = solve_part_two(&input);
    println!("Part #2: {}", result);
}

#[allow(unused)]
fn dump_dotfile(graph: &ParsedInput, filename: &str) {
    let mut fd = OpenOptions::open(
        OpenOptions::new().create(true).write(true).append(false),
        filename,
    )
    .expect("Failed to open file.");

    fd.write_all(
        Dot::with_config(&graph, &[Config::EdgeNoLabel])
            .to_string()
            .as_bytes(),
    )
    .expect("Failed to write graph to file.");

    println!("Graph dotfile dumped to '{}'.", filename);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&parse_input(INPUT));

        #[cfg(debug_assertions)]
        assert_eq!(result, 226);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 5157);
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
        assert_eq!(result, 3509);

        #[cfg(not(debug_assertions))]
        assert_eq!(result, 144309);
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| solve_part_two(&input));
    }
}
