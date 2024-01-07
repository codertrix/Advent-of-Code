// ---------------------------------------------------
//  Advent of Code 2023
//  Day 12: Hot Springs
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day12.in");

pub fn run() {
    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

#[allow(clippy::type_complexity)]
fn arrangements<'a>(sprs: &'a [char], grps: &'a [usize], grp: Option<usize>, seen: &mut HashMap<(&'a [char], &'a [usize], Option<usize>), usize>,) -> usize {
    if let Some(arrangements) = seen.get(&(sprs, grps, grp)) {
        return *arrangements;
    }

    if sprs.is_empty() {
        let arrangements = if grps.is_empty() && (grp.is_none() || grp == Some(0)) {
            1
        } else {
            0
        };

        seen.insert((sprs, grps, grp), arrangements);
        return arrangements;
    }

    let arrangements = match (sprs[0], grp) {
        ('.', None) | ('.', Some(0)) | ('?', Some(0)) => {
            arrangements(&sprs[1..], grps, None, seen)
        }
        ('.', _) | ('#', Some(0)) => 0,
        ('#', Some(grp)) | ('?', Some(grp)) => {
            arrangements(&sprs[1..], grps, Some(grp - 1), seen)
        }
        ('#', None) => {
            if grps.is_empty() {
                0
            } else {
                arrangements(&sprs[1..], &grps[1..], Some(grps[0] - 1), seen)
            }
        }
        ('?', None) => {
            let damaged = if grps.is_empty() {
                0
            } else {
                arrangements(&sprs[1..], &grps[1..], Some(grps[0] - 1), seen)
            };
            damaged + arrangements(&sprs[1..], grps, None, seen)
        }
        _ => panic!("Invalid input!"),
    };

    seen.insert((sprs, grps, grp), arrangements);
    arrangements
}

fn puzzle_one_two() -> (usize, usize) {
    let mut result_one = 0;
    let mut result_two = 0;

    for line in INPUT.lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs: Vec<_> = springs.chars().collect();
        let groups: Vec<usize> = groups.split(',').map(|c| c.parse().unwrap()).collect();

        result_one += arrangements(&springs, &groups, None, &mut HashMap::new());

        let mut five_springs = springs.clone();
        let mut five_groups = groups.clone();
        for _ in 0..4 {
            five_springs.push('?');
            five_springs.append(&mut springs.clone());
            five_groups.append(&mut groups.clone());
        }

        result_two += arrangements(&five_springs, &five_groups, None, &mut HashMap::new());
    }

    (result_one, result_two)
}
