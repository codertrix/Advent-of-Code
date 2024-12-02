// ---------------------------------------------------
//  Advent of Code 2024
//  Day 2: Red-Nosed Reports
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day02.in");

pub fn run() {
    let mut reports = Vec::new();

    for line in INPUT.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|level| level.parse().unwrap())
            .collect();
        reports.push(report);
    }

    let (result_one, result_two) = puzzle_one_two(&reports);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(reports: &[Vec<i32>]) -> (usize, usize) {
    let mut safe_reports_one = 0;
    let mut safe_reports_two = 0;

    for report in reports {
        if is_safe_report(report) {
            safe_reports_one += 1;
        } else {
            for idx in 0..report.len() {
                let mut damped_report = report.clone();
                damped_report.remove(idx);

                if is_safe_report(&damped_report) {
                    safe_reports_two += 1;
                    break;
                }
            }
        }
    }

    (safe_reports_one, safe_reports_one + safe_reports_two)
}

fn is_safe_report(report: &[i32]) -> bool {
    let is_increasing = report[0] < report[1];

    for levels in report.windows(2) {
        if (levels[0] < levels[1]) != is_increasing
            || !(1..=3).contains(&(levels[0] - levels[1]).abs())
        {
            return false;
        }
    }

    true
}
