// ---------------------------------------------------
//  Advent of Code 2022
//  Day 7: No Space Left On Device
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::str::Lines;

const INPUT: &str = include_str!("day07.in");

pub fn run() {
    let mut root = Directory { children: Vec::new(), size: 0 };
    let mut sizes = Vec::new();
    let mut lines = INPUT.lines();

    lines.next();
    parse_commands(&mut lines, &mut root);

    println!("Result for puzzle 1: {}", puzzle_one(&root, &mut sizes));
    println!("Result for puzzle 2: {}", puzzle_two(&root, &mut sizes));
}

struct Directory {
    children: Vec<Directory>,
    size: usize
}

fn parse_commands(commands: &mut Lines, directory: &mut Directory) {
    while let Some(line) = commands.next() {
        let Some(parts) = line.split_once(' ') else { panic!("Parse error!") };

        if parts.1 == "cd .." {
            return;
        }

        if parts.1.starts_with("cd ") {
            let mut child = Directory { children: Vec::new(), size: 0 };

            parse_commands(commands, &mut child);
            directory.size += child.size;
            directory.children.push(child);
        }

        if let Ok(size) = parts.0.parse::<usize>() {
            directory.size += size;
        }
    }
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(root: &Directory, sizes: &mut Vec<usize>) -> usize {
    let mut result = root.children
        .iter()
        .fold(0, |a, b| {
            sizes.push(b.size);
            a + if b.size <= 100000 { b.size } else { 0 }
        });

    if !root.children.is_empty() {
        result += root.children
            .iter()
            .map(|child| puzzle_one(child, sizes))
            .sum::<usize>();
    }

    result
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(root: &Directory, sizes: &mut Vec<usize>) -> usize {
    let size_to_delete = 30_000_000 - (70_000_000 - root.size);
    let mut result = root.size;

    for size in sizes {
        if *size >= size_to_delete {
            result = result.min(*size);
        }
    }

    result
}
