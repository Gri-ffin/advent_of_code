use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Open the file
    let path = Path::new("src/xmas.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Read the grid from the file
    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok) // Filter out any errors while reading lines
        .map(|line| line.chars().collect()) // Convert each line into a Vec<char>
        .collect();

    // Find and count the occurrences of the X-MAS pattern
    let count = find_xmas_pattern(&grid);
    println!("X-MAS appears {} times", count);

    Ok(())
}

fn find_xmas_pattern(grid: &[Vec<char>]) -> usize {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    // Traverse the grid
    for i in 1..rows - 1 {
        // Center of the "X" should not be on the borders
        for j in 1..cols - 1 {
            // Check if we can form the X-MAS shape from this center
            if check_xmas(grid, i, j) {
                count += 1;
            }
        }
    }

    count
}

// Function to check if "X-MAS" can be formed from the center (i, j)
fn check_xmas(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    let word = "MAS".chars().collect::<Vec<char>>();

    // Directions for the two diagonals of the "X" shape
    let directions = [
        // First direction: top-left to bottom-right (dx1, dy1) and bottom-left to top-right (dx2, dy2)
        [(-1, -1), (1, 1)],
        // Second direction: top-right to bottom-left (dx2, dy2) and bottom-right to top-left (dx1, dy1)
        [(1, -1), (-1, 1)],
    ];

    // Check both diagonal directions for the "MAS" pattern
    for direction in &directions {
        // Check both diagonals for the first "MAS" and second "MAS"
        if is_mas_in_direction(grid, x, y, direction[0].0, direction[0].1, &word)
            && is_mas_in_direction(grid, x, y, direction[1].0, direction[1].1, &word)
        {
            return true;
        }
    }

    false
}

// Function to check if "MAS" can be found in the given direction from (x, y)
fn is_mas_in_direction(
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
    word: &[char],
) -> bool {
    let mut cx = x as i32;
    let mut cy = y as i32;

    // Traverse the word in the given direction
    for &ch in word {
        cx += dx;
        cy += dy;

        // Check bounds and character match
        if cx < 0
            || cy < 0
            || cx >= grid.len() as i32
            || cy >= grid[0].len() as i32
            || grid[cx as usize][cy as usize] != ch
        {
            return false;
        }
    }

    true
}
