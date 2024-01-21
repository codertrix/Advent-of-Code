// ---------------------------------------------------
//  Advent of Code 2023
//  Day 22: Sand Slabs
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day22.in");

pub fn run() {
    let mut bricks: Vec<Brick> = INPUT
        .lines()
        .map(|line| line.split_once('~').unwrap())
        .map(|(a, b)| {
            let a: Vec<&str> = a.split(',').collect();
            let b: Vec<&str> = b.split(',').collect();

            let ax = a[0].parse().unwrap();
            let ay = a[1].parse().unwrap();
            let az: usize = a[2].parse().unwrap();
            let bx = b[0].parse().unwrap();
            let by = b[1].parse().unwrap();
            let bz: usize = b[2].parse().unwrap();

            Brick {
                height: bz - az + 1,
                x_start: ax,
                x_end: bx,
                y_start: ay,
                y_end: by,
                z_start: az,
                supported_by: Vec::new(),
                supporting: Vec::new(),
            }
        })
        .collect();

    bricks.sort_unstable_by(|a, b| a.z_start.cmp(&b.z_start));

    let stack = settle_bricks(bricks);

    println!("Result for puzzle 1: {}", puzzle_one(&stack));
    println!("Result for puzzle 2: {}", puzzle_two(&stack));
}

#[derive(Clone)]
struct Brick {
    height: usize,
    x_start: u32,
    x_end: u32,
    y_start: u32,
    y_end: u32,
    z_start: usize,
    supported_by: Vec<(usize, usize)>,
    supporting: Vec<(usize, usize)>,
}

impl Brick {
    fn overlaps(&self, other: &Brick) -> bool {
        !(self.x_end < other.x_start
            || self.x_start > other.x_end
            || self.y_end < other.y_start
            || self.y_start > other.y_end)
    }
}

fn settle_bricks(bricks: Vec<Brick>) -> Vec<Vec<Brick>> {
    let mut stack = vec![vec![Brick {
        height: 1,
        x_start: 0,
        x_end: u32::MAX,
        y_start: 0,
        y_end: u32::MAX,
        z_start: 0,
        supported_by: Vec::new(),
        supporting: Vec::new(),
    }]];

    for mut falling_brick in bricks {
        let mut overlapping_bricks = Vec::new();
        let mut z_idx = 0;

        for (p_idx, plane) in stack.iter().enumerate() {
            for (b_idx, brick) in plane.iter().enumerate() {
                if falling_brick.overlaps(brick) {
                    overlapping_bricks.push((p_idx, b_idx));
                    z_idx = z_idx.max(p_idx + brick.height);
                }
            }
        }

        while z_idx >= stack.len() {
            stack.push(Vec::new());
        }

        let fb_idx = stack[z_idx].len();

        for (p_idx, b_idx) in overlapping_bricks {
            if p_idx + stack[p_idx][b_idx].height == z_idx {
                stack[p_idx][b_idx].supporting.push((z_idx, fb_idx));
                falling_brick.supported_by.push((p_idx, b_idx));
            }
        }

        stack[z_idx].push(falling_brick);
    }

    stack
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(stack: &Vec<Vec<Brick>>) -> usize {
    let mut result = 0;

    for plane in stack {
        'p_loop: for brick in plane {
            for (p_idx, b_idx) in &brick.supporting {
                if stack[*p_idx][*b_idx].supported_by.len() == 1 {
                    continue 'p_loop;
                }
            }

            result += 1;
        }
    }

    result
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(stack: &[Vec<Brick>]) -> usize {
    let mut result = 0;

    for (p_idx, plane) in stack.iter().enumerate().skip(1) {
        for (b_idx, _) in plane.iter().enumerate() {
            let mut supporter = vec![(p_idx, b_idx)];

            for (z_idx, plane) in stack.iter().enumerate().skip(p_idx + 1) {
                'p_loop: for (b_idx, brick) in plane.iter().enumerate() {
                    for (pp_idx, bb_idx) in &brick.supported_by {
                        if !supporter.contains(&(*pp_idx, *bb_idx)) {
                            continue 'p_loop;
                        }
                    }

                    result += 1;
                    supporter.push((z_idx, b_idx));
                }
            }
        }
    }

    result
}
