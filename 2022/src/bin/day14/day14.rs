// ---------------------------------------------------
//  Advent of Code 2022
//  Day 14: Regolith Reservoir
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day14.in");

pub fn run() {
    let (mut cave, start_x) = create_cave();
    let (result_one, result_two) = puzzle_one_two(&mut cave, start_x);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}

fn create_cave() -> (Vec<Vec<char>>, usize) {
    let mut walls = Vec::new();
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;

    for line in INPUT.lines() {
        let coordinates: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|coordinate| {
                let Some((x, y)) = coordinate.split_once(',') else { panic!() };
                let Ok(xx) = x.parse() else { panic!() };
                let Ok(yy) = y.parse() else { panic!() };

                min_x = min_x.min(xx - 1);
                max_x = max_x.max(xx);
                min_y = min_y.min(yy);
                max_y = max_y.max(yy);
                (xx, yy)
            })
            .collect();

        walls.push(coordinates);
    }

    let height = max_y + 2;
    let width = (height + 1) * 2 + 1;
    let offset = width / 2 - (500 - min_x);
    let mut cave: Vec<Vec<_>> = vec![vec!['.'; width]; height];

    for wall in walls {
        for rocks in wall.windows(2) {
            if rocks[0].0 == rocks[1].0 {
                let from = rocks[0].1.min(rocks[1].1);
                let to = rocks[0].1.max(rocks[1].1);

                for pos in cave.iter_mut().take(to + 1).skip(from) {
                    pos[rocks[0].0 - min_x + offset] = '#';
                }
            } else {
                let from = rocks[0].0.min(rocks[1].0);
                let to = rocks[0].0.max(rocks[1].0);

                for x in from..=to {
                    cave[rocks[0].1][x - min_x + offset] = '#';
                }
            }
        }
    }

    (cave, width / 2)
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(cave: &mut Vec<Vec<char>>, start_x: usize) -> (usize, usize) {
    let mut result_one = 0;
    let mut result_two = 0;
    let mut x = start_x;
    let mut y = 0;

    loop {
        y += 1;

        if y == cave.len() {
            result_one = result_two;
            cave.push((0..cave[0].len()).map(|_| '#').collect());
        }

        if "#o".contains(cave[y][x]) {
            match (cave[y][x - 1], cave[y][x + 1]) {
                ('.', _) => x -= 1,
                (_, '.') => x += 1,
                _ => {
                    cave[y - 1][x] = 'o';
                    result_two += 1;
                    x = start_x;
                    y = 0;

                    if cave[1][x + 1] == 'o' {
                        cave[0][x] = 'o';
                        result_two += 1;
                        break;
                    }
                }
            }
        }
    }

    (result_one, result_two)
}
