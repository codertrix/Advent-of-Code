// ---------------------------------------------------
//  Advent of Code 2023
//  Day 19: Aplenty
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day19.in");

pub fn run() {
    let mut workflows = HashMap::new();
    let mut parts = Vec::<Vec<usize>>::new();
    let mut lines = INPUT.lines();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (workflow, rules) = line.split_once('{').unwrap();
        let rules: Vec<Rule> = rules[..rules.len() - 1]
            .split(',')
            .map(Rule::from)
            .collect();

        workflows.insert(workflow, rules);
    }

    for line in lines {
        parts.push(
            line[1..line.len() - 1]
                .split(',')
                .map(|p| p.split_once('=').unwrap().1.parse().unwrap())
                .collect(),
        );
    }

    println!("Result for puzzle 1: {}", puzzle_one(&workflows, &parts));
    println!("Result for puzzle 2: {}", puzzle_two(&workflows));
}

struct Rule {
    part: usize,
    operator: char,
    value: usize,
    workflow: &'static str,
}

impl Rule {
    fn apply(&self, part: &[usize]) -> Option<&str> {
        match self.operator {
            '<' => {
                if part[self.part] < self.value {
                    Some(self.workflow)
                } else {
                    None
                }
            }
            '>' => {
                if part[self.part] > self.value {
                    Some(self.workflow)
                } else {
                    None
                }
            }
            _ => Some(self.workflow),
        }
    }

    fn valid_range(&self, range: (usize, usize)) -> Option<(usize, usize)> {
        if (range.0..=range.1).contains(&self.value) {
            match self.operator {
                '<' => Some((range.0, self.value - 1)),
                '>' => Some((self.value + 1, range.1)),
                _ => None,
            }
        } else if self.value == 0 {
            Some(range)
        } else {
            None
        }
    }

    fn invalid_range(&self, range: (usize, usize)) -> Option<(usize, usize)> {
        if (range.0..=range.1).contains(&self.value) {
            match self.operator {
                '<' => Some((self.value, range.1)),
                '>' => Some((range.0, self.value)),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl From<&'static str> for Rule {
    fn from(s: &'static str) -> Self {
        let mut part = 0;
        let mut operator = ' ';
        let mut value: usize = 0;
        let s: Vec<&str> = s.split(':').collect();

        if s.len() == 1 {
            Self {
                part,
                operator,
                value,
                workflow: s[0],
            }
        } else {
            let mut chars = s[0].chars();
            part = match chars.next().unwrap() {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => panic!("Error in INPUT!"),
            };
            operator = chars.next().unwrap();

            for c in chars {
                value = value * 10 + c.to_digit(10).unwrap() as usize;
            }

            Self {
                part,
                operator,
                value,
                workflow: s[1],
            }
        }
    }
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(workflows: &HashMap<&str, Vec<Rule>>, parts: &[Vec<usize>]) -> usize {
    let mut result = 0;

    for part in parts {
        let mut workflow = "in";

        loop {
            match workflow {
                "A" => {
                    result += part.iter().sum::<usize>();
                    break;
                }
                "R" => break,
                _ => {
                    let rules = workflows.get(&workflow).unwrap();

                    for rule in rules {
                        if let Some(w) = rule.apply(part) {
                            workflow = w;
                            break;
                        }
                    }
                }
            }
        }
    }

    result
}


// ----------
//  Puzzle 2
// ----------

fn combinations_for( workflow: &str, workflows: &HashMap<&str, Vec<Rule>>, xmas: [(usize, usize); 4],) -> usize {
    if workflow == "A" {
        return xmas.into_iter().map(|r| r.1 - r.0 + 1).product();
    }

    if workflow == "R" {
        return 0;
    }

    let rules = workflows.get(&workflow).unwrap();
    let mut xmases = Vec::new();
    let mut valid_xmas = xmas;
    let mut invalid_xmas = xmas;

    for rule in rules {
        if let Some(r) = rule.valid_range(valid_xmas[rule.part]) {
            valid_xmas[rule.part] = r;
            xmases.push((rule.workflow, valid_xmas));
        }

        if let Some(r) = rule.invalid_range(invalid_xmas[rule.part]) {
            invalid_xmas[rule.part] = r;
            valid_xmas = invalid_xmas;
        } else {
            break;
        }
    }

    xmases
        .into_iter()
        .map(|(w, r)| combinations_for(w, workflows, r))
        .sum()
}

fn puzzle_two(workflows: &HashMap<&str, Vec<Rule>>) -> usize {
    combinations_for("in", workflows, [(1, 4000); 4])
}
