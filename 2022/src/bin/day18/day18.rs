// ---------------------------------------------------
//  Advent of Code 2022
//  Day 18: Boiling Boulders
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashSet;

const INPUT: &str = include_str!("day18.in");
const DXYZ: [(isize, isize, isize); 6] = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];

pub fn run() {
    let cubes: Vec<(isize, isize, isize)> = INPUT
        .lines()
        .map(|line| {
            let n: Vec<_> = line
                .split(',')
                .collect();

            (n[0].parse().unwrap(), n[1].parse().unwrap(), n[2].parse().unwrap())
        })
        .collect();

    let result_one = puzzle_one(&cubes);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", puzzle_two(&cubes, result_one));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(cubes: &[(isize, isize, isize)]) -> usize {
    let mut obsidian = HashSet::new();
    let mut surface_area = 0;

    for (x, y, z) in cubes {
        surface_area += 6;

        for (dx, dy, dz) in DXYZ {
            if obsidian.contains(&(dx + *x, dy + *y, dz + *z)) {
                surface_area -= 2;
            }
        }

        obsidian.insert((*x, *y, *z));
    }

    surface_area
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(cubes: &Vec<(isize, isize, isize)>, surface_area: usize) -> usize {
    let len_x = cubes.iter().map(|(x, _, _)| *x).max().unwrap() + 2;
    let len_y = cubes.iter().map(|(_, y, _)| *y).max().unwrap() + 2;
    let len_z = cubes.iter().map(|(_, _, z)| *z).max().unwrap() + 2;
    let mut obsidian = vec![vec![vec![0; len_x as usize]; len_y as usize]; len_z as usize];
    let mut queue = Vec::new();
    let mut surface_area = surface_area;

    for (x, y, z) in cubes {
        obsidian[*z as usize][*y as usize][*x as usize] = 1;
    }

    queue.push((0, 0, 0));

    while let Some((x, y, z)) = queue.pop() {
        if obsidian[z][y][x] == 0 {
            obsidian[z][y][x] = 2;

            for (dx, dy, dz) in DXYZ {
                let xx = dx + x as isize;
                let yy = dy + y as isize;
                let zz = dz + z as isize;

                if xx < 0 || xx == len_x || yy < 0 || yy == len_y || zz < 0 || zz == len_z {
                    continue;
                }

                if obsidian[zz as usize][yy as usize][xx as usize] == 0 {
                    queue.push((xx as usize, yy as usize, zz as usize));
                }
            }
        }
    }

    for z in 0..len_z {
        for y in 0..len_y {
            for x in 0..len_x {
                if obsidian[z as usize][y as usize][x as usize] == 0 {
                    for (dx, dy, dz) in DXYZ {
                        let xx = dx + x;
                        let yy = dy + y;
                        let zz = dz + z;

                        surface_area -= obsidian[zz as usize][yy as usize][xx as usize];
                    }
                }
            }
        }
    }

    surface_area
}
