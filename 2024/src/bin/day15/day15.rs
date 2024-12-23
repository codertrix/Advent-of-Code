// ---------------------------------------------------
//  Advent of Code 2024
//  Day 15: Warehouse Woes
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day15.in");

pub fn run() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut moves = Vec::new();
    let mut start = (0, 0);
    let mut lines = INPUT.lines();

    for (y, line) in lines.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        let mut l = Vec::new();

        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                start = (x as i32, y as i32);
                l.push('.');
            } else {
                l.push(c);
            }
        }

        grid.push(l);
    }

    for line in lines {
        moves.append(&mut line.chars().collect::<Vec<char>>());
    }

    println!("Result for puzzle 1: {}", puzzle_one(&grid, &moves, start));
    println!("Result for puzzle 2: {}", puzzle_two(&grid, &moves, start));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(grid: &[Vec<char>], moves: &[char], start: (i32, i32)) -> usize {
    let mut grid = grid.to_owned();
    let (mut x, mut y) = start;

    for m in moves {
        match m {
            '^' => y -= move_robot_one(&mut grid, x, y, 0, -1),
            'v' => y += move_robot_one(&mut grid, x, y, 0, 1),
            '<' => x -= move_robot_one(&mut grid, x, y, -1, 0),
            '>' => x += move_robot_one(&mut grid, x, y, 1, 0),
            _ => {}
        }
    }

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .fold(0, |a, (x, c)| if *c == 'O' { a + 100 * y + x } else { a })
        })
        .sum()
}

fn move_robot_one(grid: &mut [Vec<char>], x: i32, y: i32, dx: i32, dy: i32) -> i32 {
    match grid[(y + dy) as usize][(x + dx) as usize] {
        '.' => 1,
        '#' => 0,
        _ => {
            let mut dist = 2;

            loop {
                match grid[(y + dist * dy) as usize][(x + dist * dx) as usize] {
                    '#' => break 0,
                    'O' => dist += 1,
                    _ => {
                        grid[(y + dy) as usize][(x + dx) as usize] = '.';
                        grid[(y + dist * dy) as usize][(x + dist * dx) as usize] = 'O';
                        break 1;
                    }
                }
            }
        }
    }
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(grid: &[Vec<char>], moves: &[char], start: (i32, i32)) -> usize {
    let (mut x, mut y) = (start.0 * 2, start.1);
    let mut grid: Vec<Vec<char>> = grid
        .iter()
        .map(|row| {
            let mut new_row = Vec::new();

            for c in row {
                if *c == 'O' {
                    new_row.push('[');
                    new_row.push(']');
                } else {
                    new_row.push(*c);
                    new_row.push(*c);
                }
            }

            new_row
        })
        .collect();

    for m in moves {
        match m {
            '^' => y -= move_robot_two(&mut grid, x, y, 0, -1),
            'v' => y += move_robot_two(&mut grid, x, y, 0, 1),
            '<' => x -= move_robot_two(&mut grid, x, y, -1, 0),
            '>' => x += move_robot_two(&mut grid, x, y, 1, 0),
            _ => {}
        }
    }

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .fold(0, |a, (x, c)| if *c == '[' { a + 100 * y + x } else { a })
        })
        .sum()
}

fn move_robot_two(grid: &mut [Vec<char>], x: i32, y: i32, dx: i32, dy: i32) -> i32 {
    match grid[(y + dy) as usize][(x + dx) as usize] {
        '.' => 1,
        '#' => 0,
        _ => {
            if dy == 0 {
                let mut dist = 2;

                loop {
                    match grid[y as usize][(x + dist * dx) as usize] {
                        '#' => break 0,
                        '[' | ']' => dist += 1,
                        _ => {
                            if dx == 1 {
                                grid[y as usize].remove((x + dist * dx) as usize);
                                grid[y as usize].insert(x as usize, '.');
                            } else {
                                grid[y as usize].insert(x as usize, '.');
                                grid[y as usize].remove((x + dist * dx) as usize);
                            }

                            break 1;
                        }
                    }
                }
            } else {
                let bx = if grid[(y + dy) as usize][x as usize] == '[' {
                    (x, x + 1, y + 2 * dy)
                } else {
                    (x - 1, x, y + 2 * dy)
                };

                if check_box(grid, bx, dy, false) == 1 {
                    check_box(grid, bx, dy, true)
                } else {
                    0
                }
            }
        }
    }
}

fn check_box(grid: &mut [Vec<char>], bx: (i32, i32, i32), dy: i32, move_box: bool) -> i32 {
    let x1 = bx.0 as usize;
    let x2 = bx.1 as usize;
    let y = bx.2 as usize;

    if grid[y][x1] == '#' || grid[y][x2] == '#' {
        return 0;
    }

    if grid[y][x1] == '[' && check_box(grid, (bx.0, bx.0 + 1, bx.2 + dy), dy, move_box) == 0 {
        return 0;
    }

    if grid[y][x1] == ']' && check_box(grid, (bx.0 - 1, bx.0, bx.2 + dy), dy, move_box) == 0 {
        return 0;
    }

    if grid[y][x2] == '[' && check_box(grid, (bx.1, bx.1 + 1, bx.2 + dy), dy, move_box) == 0 {
        return 0;
    }

    if grid[y][x2] == ']' && check_box(grid, (bx.1 - 1, bx.1, bx.2 + dy), dy, move_box) == 0 {
        return 0;
    }

    if move_box {
        grid[y][x1] = grid[(bx.2 - dy) as usize][x1];
        grid[y][x2] = grid[(bx.2 - dy) as usize][x2];
        grid[(bx.2 - dy) as usize][x1] = '.';
        grid[(bx.2 - dy) as usize][x2] = '.';
    }

    1
}
