// ---------------------------------------------------
//  Advent of Code 2016
//  Day 22: Grid Computing
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("day22.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------");
    println!(" Day 22: Grid Computing");
    println!("------------------------");

    let (result_one, node_grid) = puzzle_one();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", puzzle_two(node_grid));

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}

fn usize(s: &str) -> usize {
    s[..s.len() - 1].parse().unwrap()
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> (usize, Vec<Vec<(usize, usize)>>) {
    let mut storage_data = Vec::new();
    let mut viable_pairs = 0;

    for line in INPUT.lines() {
        if line.starts_with('/') {
            let parts: Vec<_> = line.split_whitespace().collect();
            let node: Vec<_> = parts[0].split('-').collect();
            let x: usize = node[1][1..].parse().unwrap();
            let y: usize = node[2][1..].parse().unwrap();
            let (s, u, a) = (usize(parts[1]), usize(parts[2]), usize(parts[3]));

            for (_, _, _, used, avail) in &storage_data {
                if u > 0 && u <= *avail || *used > 0 && *used < a {
                    viable_pairs += 1;
                }
            }

            storage_data.push((x, y, s, u, a));
        }
    }

    let mut node_grid = vec![Vec::new()];
    let mut idx = 0;

    for (x, _, s, u, _) in storage_data {
        if x > idx {
            node_grid.push(Vec::new());
            idx += 1;
        }

        node_grid[idx].push((s, u));
    }

    (viable_pairs, node_grid)
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_two(node_grid: Vec<Vec<(usize, usize)>>) -> usize {
    let max_x = node_grid.len() - 1;
    let max_y = node_grid[0].len() - 1;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut steps_to_empty_node = 0;

    queue.push_back((max_x as isize, 0, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        if seen.contains(&(x, y)) {
            continue;
        }

        seen.insert((x, y));

        if node_grid[x as usize][y as usize].1 == 0 {
            steps_to_empty_node = steps;
            break;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (xx, yy) = (x + dx, y + dy);

            if xx >= 0
                && yy >= 0
                && xx <= max_x as isize
                && yy <= max_y as isize
                && node_grid[xx as usize][yy as usize].0 >= node_grid[x as usize][y as usize].1
            {
                queue.push_back((xx, yy, steps + 1))
            }
        }
    }

    steps_to_empty_node + (max_x - 1) * 5
}
