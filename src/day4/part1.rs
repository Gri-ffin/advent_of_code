use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Open the file
    let path = Path::new("src/xmas.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let result = find_xmas(&grid);
    println!("Result: {}", result);

    Ok(())
}

fn find_xmas(grid: &[Vec<char>]) -> usize {
    let word = "XMAS".chars().collect::<Vec<char>>();
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    // Directions to move (right, down, left, up and four diagonals)
    let directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    // Check all the positions on the grid
    for i in 0..rows {
        for j in 0..cols {
            for (dx, dy) in &directions {
                if can_find_word(grid, i, j, *dx, *dy, &word) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn can_find_word(grid: &[Vec<char>], x: usize, y: usize, dx: i32, dy: i32, word: &[char]) -> bool {
    let mut cx = x as i32;
    let mut cy = y as i32;

    for &ch in word {
        // Check if the current position is out of bounds
        if cx < 0 || cy < 0 || cx >= grid.len() as i32 || cy >= grid[0].len() as i32 {
            return false;
        }

        // Check if the current word matches
        if grid[cx as usize][cy as usize] != ch {
            return false;
        }

        cx += dx;
        cy += dy;
    }

    true
}
