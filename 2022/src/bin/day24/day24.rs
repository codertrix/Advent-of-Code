// ---------------------------------------------------
//  Advent of Code 2022
//  Day 24: Blizzard Basin
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{VecDeque, HashMap, HashSet};

const INPUT: &str = include_str!("day24.in");

pub fn run() {
    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut valley_map = HashMap::new();

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x as isize - 1);
            max_y = max_y.max(y as isize - 1);

            if c != '.' && c != '#' {
                valley_map.insert((x as isize, y as isize), vec![c]);
            }
        }
    }

    let start = (1, 0);
    let end = (max_x, max_y + 1);
    let mut valley_maps = vec![valley_map];
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result_one = 0;

    queue.push_back((start.0, start.1, 0, false, false));

    while let Some((ex, ey, mut minute, end_reached, start_reached)) = queue.pop_front() {
        if seen.contains(&(ex, ey, minute, end_reached, start_reached)) {
            continue;
        }

        seen.insert((ex, ey, minute, end_reached, start_reached));
        minute += 1;

        if minute + 1 > valley_maps.len() - 1 {
            valley_map = HashMap::new();

            for ((x, y), b) in &valley_maps[valley_maps.len() - 1] {
                for bb in b {
                    let (xx, yy) = match bb {
                        '^' => if *y > 1 { (*x, *y - 1) } else { (*x, max_y) },
                        'v' => if *y < max_y { (*x, *y + 1) } else { (*x, 1) },
                        '<' => if *x > 1 { (*x - 1, *y) } else { (max_x, *y) },
                        '>' => if *x < max_x { (*x + 1, *y) } else { (1, *y) },
                        _ => unreachable!()
                    };

                    valley_map.entry((xx, yy)).or_default().push(*bb);
                }
            }

            valley_maps.push(valley_map);
        }

        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1), (0, 0)] {
            let xx = ex + dx;
            let yy = ey + dy;

            match (end_reached, start_reached) {
                (true, true) => if (xx, yy) == end {
                    return (result_one, minute);
                }
                (false, false) => if (xx, yy) == end {
                    if result_one == 0 {
                        result_one = minute;
                    }
                    queue.push_back((xx, yy, minute, true, false));
                }
                (true, false) => if (xx, yy) == start {
                    queue.push_back((xx, yy, minute, true, true));
                }
                _ => {}
            }

            if xx > 0 && xx <= max_x && yy > 0 && yy <= max_y && !valley_maps[minute].contains_key(&(xx, yy)) || (xx, yy) == start || (xx, yy) == end {
                queue.push_back((xx, yy, minute, end_reached, start_reached));
            }
        }
    }

    unreachable!("Can't reach the goal!")
}
