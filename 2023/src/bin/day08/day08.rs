// ---------------------------------------------------
//  Advent of Code 2023
//  Day 8: Haunted Wasteland
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day08.in");

pub fn run() {
    let mut lines = INPUT.lines();
    let lr: Vec<_> = lines.next().unwrap().chars().collect();
    let _ = lines.next();
    let mut nodes = HashMap::new();
    let mut a_nodes = Vec::new();

    for line in lines {
        let (node, lr_nodes) = line.split_once(" = ").unwrap();

        nodes.insert(node, (&lr_nodes[1..4], &lr_nodes[6..9]));

        if node.ends_with('A') {
            a_nodes.push(node);
        }
    }

    println!("Result for puzzle 1: {}", puzzle_one(&lr, &nodes));
    println!("Result for puzzle 2: {}", puzzle_two(&lr, &nodes, &a_nodes));
}

fn required_steps(lr: &[char], start_node: &str, nodes: &HashMap<&str, (&str, &str)>) -> usize {
    let mut steps = 0;
    let mut current_node = start_node;

    while !current_node.ends_with('Z') {
        let (left, right) = nodes.get(&current_node).unwrap();

        current_node = if lr[steps % lr.len()] == 'L' {
            left
        } else {
            right
        };
        steps += 1;
    }

    steps
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(lr: &[char], nodes: &HashMap<&str, (&str, &str)>) -> usize {
    required_steps(lr, "AAA", nodes)
}


// ----------
//  Puzzle 2
// ----------

fn gcd(a: usize, b: usize) -> usize {
    let mut aa;
    let mut bb = a.max(b);
    let mut rr = a.min(b);

    while rr > 0 {
        aa = bb;
        bb = rr;
        rr = aa % bb;
    }

    bb
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn puzzle_two(lr: &[char], nodes: &HashMap<&str, (&str, &str)>, a_nodes: &[&str]) -> usize {
    let steps: Vec<_> = a_nodes
        .iter()
        .map(|start_node| required_steps(lr, start_node, nodes))
        .collect();
    let first_steps = steps[0];

    steps.iter().skip(1).fold(first_steps, |a, b| lcm(a, *b))
}
