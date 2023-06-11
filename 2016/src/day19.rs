// ---------------------------------------------------
//  Advent of Code 2016
//  Day 19: An Elephant Named Joseph
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: usize = 3018458;

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n----------------------------------");
    println!(" Day 19: An Elephant Named Joseph");
    println!("----------------------------------");

    println!("Result for puzzle 1: {}", puzzle_one());
    println!("Result for puzzle 2: {}", puzzle_two());

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> usize {
    let mut elves = vec![1; INPUT + 1];

    for (idx, elf) in elves.iter_mut().enumerate().take(INPUT).skip(1) {
        *elf = idx + 1;
    }

    let mut current_elf = 1;

    loop {
        if elves[current_elf] == current_elf {
            return current_elf;
        }

        elves[current_elf] = elves[elves[current_elf]];
        current_elf = elves[current_elf];
    }
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two() -> usize {
    let mut elves = vec![1; INPUT + 1];

    for (idx, elf) in elves.iter_mut().enumerate().take(INPUT).skip(1) {
        *elf = idx + 1;
    }

    let mut current_elf = 1;
    let mut elves_count = INPUT;
    let mut idx_before_across = elves_count / 2;

    loop {
        if elves[current_elf] == current_elf {
            return current_elf;
        }

        elves[idx_before_across] = elves[elves[idx_before_across]];
        current_elf = elves[current_elf];
        elves_count -= 1;

        if elves_count % 2 == 0 {
            idx_before_across = elves[idx_before_across];
        }
    }
}
