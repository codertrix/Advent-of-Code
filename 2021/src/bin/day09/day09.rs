// ---------------------------------------------------
//  Advent of Code 2021
//  Day 9: Smoke Basin
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day09.in");

pub fn run() {
    let mut heightmap: Vec<Vec<u8>> = INPUT
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&line| line
             .bytes()
             .map(|n| n - 48)
             .collect())
        .collect();

    let (sum_risk_level, low_points) = puzzle_one(&heightmap);

    println!("Result for puzzle 1: {}", sum_risk_level);
    println!("Result for puzzle 2: {}", puzzle_two(&mut heightmap, &low_points));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(map: &[Vec<u8>]) -> (usize, Vec<(usize, usize)>) {
    let mut low_points = Vec::new();
    let mut sum_risk_level = 0;
    let x_max = map[0].len() - 1;
    let y_max = map.len() - 1;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let height = map[y][x];

            if x > 0 && map[y][x - 1] <= height { continue; }
            if x < x_max && map[y][x + 1] <= height { continue; }
            if y > 0 && map[y - 1][x] <= height { continue; }
            if y < y_max && map[y + 1][x] <= height { continue; }

            low_points.push((x, y));
            sum_risk_level += height as usize + 1;
        }
    }

    (sum_risk_level, low_points)
}


// ----------
//  Puzzle 2
// ----------

fn size_for(x: usize, y: usize, map: &mut [Vec<u8>]) -> usize {
    let mut size = 0;
    let x_max = map[0].len() - 1;
    let y_max = map.len() - 1;

    if map[y][x] < 9 {
        size = 1;
        map[y][x] = 9;

        while x > 0 && map[y][x - 1] < 9 {
            size += size_for(x - 1, y, map);
        }

        while y > 0 && map[y - 1][x] < 9 {
            size += size_for(x, y - 1, map);
        }

        while y < y_max && map[y + 1][x] < 9 {
            size += size_for(x, y + 1, map);
        }

        while x < x_max && map[y][x + 1] < 9 {
            size += size_for(x + 1, y, map);
        }

        while y > 0 && map[y - 1][x] < 9 {
            size += size_for(x, y - 1, map);
        }

        while y < y_max && map[y + 1][x] < 9 {
            size += size_for(x, y + 1, map);
        }
    }

    size
}

fn puzzle_two(map: &mut [Vec<u8>], low_points: &[(usize, usize)]) -> usize {
    let mut basins = Vec::new();

    for (x, y) in low_points {
        basins.push(size_for(*x, *y, map));
    }

    basins.sort_unstable();
    basins
        .iter()
        .rev()
        .take(3)
        .product()
}
