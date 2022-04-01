// ---------------------------------------------------
//  Advent of Code 2021
//  Day 22: Reactor Reboot
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::ops::RangeInclusive;

const INPUT: &str = include_str!("day22.in");

pub fn run() {
    let mut reboot_steps = Vec::new();

    for line in INPUT.lines() {
        let (state, ranges) = line
            .split_once(' ')
            .unwrap();
        let mut ranges = ranges.split(',');
        let x_range = ranges
            .next()
            .unwrap()
            .trim_start_matches("x=")
            .split_once("..")
            .unwrap();
        let x_range = x_range.0.parse().unwrap()..=x_range.1.parse().unwrap();
        let y_range = ranges
            .next()
            .unwrap()
            .trim_start_matches("y=")
            .split_once("..")
            .unwrap();
        let y_range = y_range.0.parse().unwrap()..=y_range.1.parse().unwrap();
        let z_range = ranges
            .next()
            .unwrap()
            .trim_start_matches("z=")
            .split_once("..")
            .unwrap();
        let z_range = z_range.0.parse().unwrap()..=z_range.1.parse().unwrap();

        reboot_steps.push(Cube::new(state, x_range, y_range, z_range));
    }

    println!("Result for puzzle 1: {}", reboot_reactor(&reboot_steps, true));
    println!("Result for puzzle 2: {}", reboot_reactor(&reboot_steps, false));
}

#[derive(Clone)]
struct Cube {
    state: isize,
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    z_range: RangeInclusive<isize>,
}

impl Cube {
    fn new(state: &str, x_range: RangeInclusive<isize>, y_range: RangeInclusive<isize>, z_range: RangeInclusive<isize>) -> Self {
        let state = if state == "on" { 1 } else { -1 };

        Self { state, x_range, y_range, z_range }
    }

    fn overlapping_cube(&self, cube: &Cube) -> Option<Cube> {
        let &dx_min = self.x_range.start().max(cube.x_range.start());
        let &dx_max = self.x_range.end().min(cube.x_range.end());
        let &dy_min = self.y_range.start().max(cube.y_range.start());
        let &dy_max = self.y_range.end().min(cube.y_range.end());
        let &dz_min = self.z_range.start().max(cube.z_range.start());
        let &dz_max = self.z_range.end().min(cube.z_range.end());

        if dx_min > dx_max || dy_min > dy_max || dz_min > dz_max {
            return None;
        }

        Some(Cube { state: 0, x_range: dx_min..=dx_max, y_range: dy_min..=dy_max, z_range: dz_min..=dz_max })
    }

    fn count(&self) -> isize {
        let x_value = self.x_range.end() - self.x_range.start() + 1;
        let y_value = self.y_range.end() - self.y_range.start() + 1;
        let z_value = self.z_range.end() - self.z_range.start() + 1;

        x_value * y_value * z_value * self.state
    }
}

fn reboot_reactor(reboot_steps: &[Cube], initialization: bool) -> isize {
    let mut reactor: Vec<Cube> = Vec::new();

    for next_cube in reboot_steps {
        if initialization && (*next_cube.x_range.start() < -50 || *next_cube.x_range.end() > 50) {
            break;
        }

        for idx in 0..reactor.len() {
            if let Some(mut cube) = reactor[idx].overlapping_cube(next_cube) {
                cube.state = -reactor[idx].state;
                reactor.push(cube);
            }
        }

        if next_cube.state == 1 {
            reactor.push(next_cube.clone());
        }
    }

    reactor
        .iter()
        .fold(0, |a, b| a + b.count())
}
