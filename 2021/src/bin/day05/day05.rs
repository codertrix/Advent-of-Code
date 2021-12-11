// ---------------------------------------------------
//  Advent of Code 2021
//  Day 5: Hydrothermal Venture
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day05.in");

struct Line {
    p1: (isize, isize),
    p2: (isize, isize),
}

struct Canvas {
    canvas: HashMap<(isize, isize), isize>,
}

impl Canvas {
    fn new() -> Self {
        Self { canvas: HashMap::new() }
    }

    fn values(from: isize, to: isize) -> Vec<isize> {
        if from < to {
            (from..=to).collect()
        } else {
            (to..=from).rev().collect()
        }
    }

    fn draw_line(&mut self, line: &Line) {
        let xs = Canvas::values(line.p1.0, line.p2.0);
        let ys = Canvas::values(line.p1.1, line.p2.1);

        if xs.len() == ys.len() {
            for (&x, &y) in xs.iter().zip(ys.iter()) {
                self.canvas
                    .entry((x, y))
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        } else {
            for x in xs {
                for y in &ys {
                    self.canvas
                        .entry((x, *y))
                        .and_modify(|n| *n += 1)
                        .or_insert(1);
                }
            }
        }
    }

    fn overlapping_count(&self) -> usize {
        self.canvas
            .values()
            .filter(|&n| *n > 1)
            .count()
    }
}

pub fn run() {
    let mut lines: Vec<Line> = INPUT
        .lines()
        .map(|line| line
             .split_once(" -> ")
             .unwrap())
        .map(|p| {
            let p1 = p.0.split_once(",").unwrap();
            let p2 = p.1.split_once(",").unwrap();
            let point1 = (p1.0.parse().unwrap(), p1.1.parse().unwrap());
            let point2 = (p2.0.parse().unwrap(), p2.1.parse().unwrap());
            Line { p1: point1, p2: point2 }
        })
        .collect();
    let mut canvas = Canvas::new();

    println!("Result for puzzle 1: {}", puzzle_one(&mut lines, &mut canvas));
    println!("Result for puzzle 2: {}", puzzle_two(&mut lines, &mut canvas));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(lines: &mut Vec<Line>, canvas: &mut Canvas) -> usize {
    lines
        .iter()
        .filter(|line| line.p1.0 == line.p2.0 || line.p1.1 == line.p2.1)
        .for_each(|line| canvas.draw_line(line));

    canvas.overlapping_count()
}

// ----------
//  Puzzle 2
// ----------

fn puzzle_two(lines: &mut Vec<Line>, canvas: &mut Canvas) -> usize {
    lines
        .iter()
        .filter(|line| line.p1.0 != line.p2.0 && line.p1.1 != line.p2.1)
        .for_each(|line| canvas.draw_line(line));

    canvas.overlapping_count()
}
