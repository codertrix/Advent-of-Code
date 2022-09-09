// ---------------------------------------------------
//  Advent of Code 2015
//  Day 20: Infinite Elves and Infinite Houses
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: usize = 36000000;
const MAX: usize = 1_000_001;

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one());
    println!("Result for puzzle 2: {}", puzzle_two());
}

fn lowest_house_number(present_factor: usize, max_houses: usize) -> usize {
    let mut houses: [usize; MAX] = [0; MAX];

    for elf in 1.. {
        let presents_to_deliver = elf * present_factor;
        let house_iter = houses
            .iter_mut()
            .enumerate()
            .skip(elf)
            .step_by(elf)
            .take(max_houses);

        for (house, presents) in house_iter {
            *presents += presents_to_deliver;

            if *presents >= INPUT {
                return house;
            }
        }
    }

    unreachable!();
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> usize {
    lowest_house_number(10, MAX)
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two() -> usize {
    lowest_house_number(11, 50)
}
