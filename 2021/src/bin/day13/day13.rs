// ---------------------------------------------------
//  Advent of Code 2021
//  Day 13: Transparent Origami
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use core::cmp::max;
use std::collections::HashSet;

const INPUT: &str = include_str!("day13.in");

pub fn run() {
    let mut paper: HashSet<(u32, u32)> = HashSet::new();
    let (dots, folds) = INPUT
        .split_once("\n\n")
        .unwrap();

    for line in dots.lines() {
        let (x, y) = line
            .split_once(',')
            .unwrap();

        paper.insert((x.parse().unwrap(), y.parse().unwrap()));
    }

    let folds: Vec<(char, u32)> = folds
        .lines()
        .map(|line| {
            let (along, value) = line
                .split_once('=')
                .unwrap();

            (along.chars().last().unwrap(), value.parse().unwrap())
        })
        .collect();

    paper = fold_paper(paper, folds[0]);

    let dot_count = paper.len();

    for fold in folds.iter().skip(1) {
        paper = fold_paper(paper, *fold);
    }

    println!("Result for puzzle 1: {}", dot_count);
    println!("Result for puzzle 2:\n\n{}", printed_paper(&paper));
}

fn fold_paper(paper: HashSet<(u32, u32)>, folds: (char, u32)) -> HashSet<(u32, u32)> {
    let mut new_paper = HashSet::new();

    if folds.0 == 'x' {
        for (x, y) in paper.iter() {
            new_paper.insert(if *x > folds.1 {
                (folds.1 - (*x -folds.1), *y)
            } else {
                (*x, *y)
            });
        }
    } else {
        for (x, y) in paper.iter() {
            new_paper.insert(if *y > folds.1 {
                (*x, folds.1 - (*y -folds.1))
            } else {
                (*x, *y)
            });
        }
    }

    new_paper
}

fn printed_paper(paper: &HashSet<(u32, u32)>) -> String {
    let (x_max, y_max) = paper
        .iter()
        .fold((0, 0), |(xa, ya), (xb, yb)| (max(xa, *xb), max(ya, *yb)));
    let mut code = String::new();

    for y in 0..=y_max {
        for x in 0..=x_max {
            if paper.contains(&(x,y)) {
                code.push('#');
            } else {
                code.push(' ');
            }
        }

        code.push('\n');
    }

    code
}
