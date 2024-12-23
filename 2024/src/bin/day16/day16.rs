// ---------------------------------------------------
//  Advent of Code 2024
//  Day 16: Reindeer Maze
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("day16.in");

pub fn run() {
    let maze: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let (score_one, score_two) = puzzle_one_two(&maze);

    println!("Result for puzzle 1: {}", score_one);
    println!("Result for puzzle 2: {}", score_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(maze: &[Vec<char>]) -> (usize, usize) {
    let mut positions = BinaryHeap::new();
    let mut seen = HashMap::new();
    let mut score_one = 0;
    let mut steps = VecDeque::new();
    let mut tiles = HashSet::new();

    positions.push(Reverse((0, 1i32, maze.len() as i32 - 2, 1, 0, 0, 0)));

    while let Some(Reverse((score, x, y, dx, dy, px, py))) = positions.pop() {
        if maze[y as usize][x as usize] == 'E' {
            score_one = score;
            steps.push_back((px, py, score));
            tiles.insert((x, y));
            break;
        }

        if seen.contains_key(&(x, y, dx, dy)) {
            continue;
        }

        seen.insert((x, y, dx, dy), (px, py, score));

        for (ndx, ndy, nscore) in [
            (dx, dy, score + 1),
            (-dy, dx, score + 1001),
            (dy, -dx, score + 1001),
        ] {
            if maze[(y + ndy) as usize][(x + ndx) as usize] != '#' {
                positions.push(Reverse((nscore, x + ndx, y + ndy, ndx, ndy, x, y)));
            }
        }
    }

    while let Some((x, y, score)) = steps.pop_front() {
        if score == 0 {
            continue;
        }

        tiles.insert((x, y));

        let itr = seen
            .iter()
            .filter_map(|(k, v)| if k.0 == x && k.1 == y { Some(*v) } else { None });

        for step in itr {
            let d = score as isize - step.2 as isize;

            if d == 1 || d == 1001 {
                steps.push_back(step);
            }
        }
    }

    (score_one, tiles.len())
}
