use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_step(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
        }
    }
}

fn track_guard_path(grid: &Vec<Vec<char>>) -> usize {
    let mut visited = HashSet::new();
    let mut current_pos = find_guard_start(grid);
    let mut current_dir = find_initial_direction(grid);

    visited.insert(current_pos);

    loop {
        let next_pos = current_dir.move_step(current_pos);

        // if the guard left the map, we found our solution
        if is_out_of_bound(grid, next_pos) {
            break;
        }

        // if the path ahead is blocked, the guard would turn right, and we try again
        if is_blocked(grid, next_pos) {
            current_dir = current_dir.turn_right();
            continue;
        }

        // Move to the next position
        current_pos = next_pos;
        visited.insert(current_pos);
    }

    visited.len()
}

fn find_guard_start(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if "^>v<".contains(cell) {
                return (r as i32, c as i32);
            }
        }
    }
    panic!("No guard found in the map");
}

fn find_initial_direction(grid: &Vec<Vec<char>>) -> Direction {
    for row in grid {
        for &cell in row {
            match cell {
                '^' => return Direction::Up,
                '>' => return Direction::Right,
                'v' => return Direction::Down,
                '<' => return Direction::Left,
                _ => continue,
            }
        }
    }
    panic!("No guard found in the map");
}

fn is_out_of_bound(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    pos.0 < 0 || pos.0 >= grid.len() as i32 || pos.1 < 0 || pos.1 >= grid[0].len() as i32
}

fn is_blocked(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    grid[pos.0 as usize][pos.1 as usize] == '#'
}

fn read_map(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn main() {
    let grid = read_map("guard.txt");
    let distinct_positions = track_guard_path(&grid);
    println!(
        "Distinct positions visited by the guard: {}",
        distinct_positions
    );
}
