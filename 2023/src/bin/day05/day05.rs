// ---------------------------------------------------
//  Advent of Code 2023
//  Day 5: If You Give A Seed A Fertilizer
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::ops::Range;

const INPUT: &str = include_str!("day05.in");

pub fn run() {
    let mut lines = INPUT.lines();

    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|seed| seed.parse().unwrap())
        .collect();
    lines.next();

    let mut maps = Vec::new();

    while lines.next().is_some() {
        let mut map = Vec::new();

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let values: Vec<i64> = line.split(' ').map(|v| v.parse().unwrap()).collect();
            map.push((values[1]..values[1] + values[2], values[0] - values[1]));
        }

        map.sort_unstable_by(|a, b| a.0.start.cmp(&b.0.start));
        maps.push(map);
    }

    println!("Result for puzzle 1: {}", puzzle_one(&seeds, &maps));
    println!("Result for puzzle 2: {}", puzzle_two(&seeds, &maps));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(seeds: &[i64], maps: &[Vec<(Range<i64>, i64)>]) -> i64 {
    let mut lowest_location = i64::MAX;

    for seed in seeds {
        let mut location = *seed;

        for map in maps {
            for (range, offset) in map {
                if range.contains(&location) {
                    location += offset;
                    break;
                }
            }
        }

        lowest_location = lowest_location.min(location);
    }

    lowest_location
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(seeds: &[i64], maps: &[Vec<(Range<i64>, i64)>]) -> i64 {
    let mut seed_ranges: Vec<Range<i64>> = seeds
        .chunks(2)
        .map(|seed| seed[0]..seed[0] + seed[1])
        .collect();

    for map in maps {
        let mut next_ranges = Vec::new();

        for mut seed_range in seed_ranges {
            let mut map_idx = 0;

            while map_idx < map.len() {
                let (range, offset) = &map[map_idx];

                if seed_range.end <= range.start {
                    next_ranges.push(seed_range);
                    break;
                }

                if seed_range.start >= range.end {
                    map_idx += 1;

                    if map_idx == map.len() {
                        next_ranges.push(seed_range);
                        break;
                    }

                    continue;
                }

                if seed_range.start < range.start {
                    next_ranges.push(seed_range.start..range.start);
                    seed_range = range.start..seed_range.end;
                    continue;
                }

                if seed_range.end <= range.end {
                    next_ranges.push(seed_range.start + offset..seed_range.end + offset);
                    break;
                }

                next_ranges.push(seed_range.start + offset..range.end + offset);
                seed_range = range.end..seed_range.end;
                map_idx += 1;
            }
        }

        seed_ranges = next_ranges;
    }

    seed_ranges
        .into_iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}
