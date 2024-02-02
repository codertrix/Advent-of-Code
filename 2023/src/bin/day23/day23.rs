// ---------------------------------------------------
//  Advent of Code 2023
//  Day 23: A Long Walk
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day23.in");

pub fn run() {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let (result_one, result_two) = puzzle_one_two(&grid);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

type Destinations = HashSet<(usize, usize, usize)>;

fn longest_path(nodes: &HashMap<(usize, usize), Destinations>) -> usize {
    let mut queue = Vec::new();
    let mut longest_path = 0;

    queue.push((vec![(0, 1)], 0));

    while let Some((path, steps)) = queue.pop() {
        let next_nodes = nodes.get(&path[path.len() - 1]).unwrap();

        if next_nodes.is_empty() {
            longest_path = longest_path.max(steps);
            continue;
        }

        for (y, x, s) in next_nodes {
            if !path.contains(&(*y, *x)) {
                let mut new_path = path.clone();

                new_path.push((*y, *x));
                queue.push((new_path, steps + s));
            }
        }
    }

    longest_path
}

fn puzzle_one_two(grid: &[Vec<char>]) -> (usize, usize) {
    let destination = (grid.len() - 1, grid[0].len() - 2);
    let mut nodes = HashMap::new();
    let mut queue = Vec::new();
    let mut result_one = 0;

    queue.push((HashSet::new(), 0, 1, 0, 1, 1));

    while let Some((mut visited, y, x, mut py, mut px, mut ps)) = queue.pop() {
        if (y, x) == destination {
            nodes.insert((y, x), HashSet::new());
            nodes
                .entry((py, px))
                .or_insert(HashSet::new())
                .insert((y, x, visited.len() - ps + 1));
            result_one = result_one.max(visited.len());
            continue;
        }

        if visited.contains(&(y, x)) {
            continue;
        }

        visited.insert((y, x));

        let mut next_positions = Vec::new();

        if y == 0 {
            next_positions.push((1, 1, 'v'));
        } else {
            for (yy, xx, d) in [ (y - 1, x, '^'), (y, x + 1, '>'), (y + 1, x, 'v'), (y, x - 1, '<'), ] {
                if grid[yy][xx] != '#' {
                    next_positions.push((yy, xx, d));
                }
            }
        }

        if next_positions.len() > 2 {
            nodes
                .entry((py, px))
                .or_insert(HashSet::new())
                .insert((y, x, visited.len() - ps));
            nodes
                .entry((y, x))
                .or_insert(HashSet::new())
                .insert((py, px, visited.len() - ps));
            py = y;
            px = x;
            ps = visited.len();
        }

        for (yy, xx, d) in next_positions {
            if !visited.contains(&(yy, xx)) && grid[yy][xx] == '.' || grid[yy][xx] == d {
                queue.push((visited.clone(), yy, xx, py, px, ps));
            }
        }
    }

    (result_one, longest_path(&nodes))
}
