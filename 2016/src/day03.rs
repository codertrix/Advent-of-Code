// ---------------------------------------------------
//  Advent of Code 2016
//  Day 3: Squares With Three Sides
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day03.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n---------------------------------");
    println!(" Day 3: Squares With Three Sides");
    println!("---------------------------------");

    let triangles: Vec<Vec<usize>> = INPUT
        .lines()
        .map(|line| line
             .split_whitespace()
             .map(|s| s.parse().unwrap())
             .collect())
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one(&triangles));
    println!("Result for puzzle 2: {}", puzzle_two(&triangles));

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


fn is_possible_triangle(a: usize, b: usize, c: usize) -> bool {
    a + b > c && b + c > a && a + c > b
}

// ----------
//  Puzzle 1
// ----------

fn puzzle_one(triangles: &[Vec<usize>]) -> usize {
    let mut result = 0;

    for lengths in triangles {
        if is_possible_triangle(lengths[0], lengths[1], lengths[2]) {
            result += 1;
        }
    }

    result
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(triangles: &[Vec<usize>]) -> usize {
    let mut result = 0;
    let mut idx = 0;

    while idx < triangles.len() - 2 {
        if is_possible_triangle(triangles[idx][0], triangles[idx + 1][0], triangles[idx + 2][0]) {
            result += 1;
        }
        if is_possible_triangle(triangles[idx][1], triangles[idx + 1][1], triangles[idx + 2][1]) {
            result += 1;
        }
        if is_possible_triangle(triangles[idx][2], triangles[idx + 1][2], triangles[idx + 2][2]) {
            result += 1;
        }

        idx += 3;
    }

    result
}
