use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filename).unwrap();
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_regions(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<Vec<Point>>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions: HashMap<char, Vec<Vec<Point>>> = HashMap::new();

    for row in 0..rows {
        for col in 0..cols {
            // Skip visited cells
            if !visited[row][col] {
                let plant = grid[row][col];
                // otherwise start a new flood fill
                let region = flood_fill(&grid, &mut visited, col, row, plant);
                // and add it to the regions
                regions.entry(plant).or_default().push(region);
            }
        }
    }

    regions
}

// this function is a simple flood fill algorithm
// that fills a region of the same type of plant
fn flood_fill(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start_x: usize,
    start_y: usize,
    target_plan: char,
) -> Vec<Point> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut region = Vec::new();
    let mut queue = vec![(start_x, start_y)];

    // we use bfs to fill the region
    while let Some((x, y)) = queue.pop() {
        if x >= cols as usize || y >= rows as usize || visited[y][x] || grid[y][x] != target_plan {
            continue;
        }

        visited[y][x] = true;
        region.push(Point {
            x: x as i32,
            y: y as i32,
        });

        for (dx, dy) in directions.iter() {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0 && new_x < cols && new_y >= 0 && new_y < rows {
                queue.push((new_x as usize, new_y as usize));
            }
        }
    }

    region
}

fn calculate_region_price(region: &Vec<Point>) -> i64 {
    let area = region.len() as i64;
    let perimeter = calculate_perimeter(region);
    area * perimeter
}

fn calculate_perimeter(region: &Vec<Point>) -> i64 {
    let point_set: HashSet<Point> = region.iter().cloned().collect();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut perimeter = 0;

    for point in region {
        for (dx, dy) in directions.iter() {
            let neighbor = Point {
                x: point.x + dx,
                y: point.y + dy,
            };

            if !point_set.contains(&neighbor) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn solve(regions: &HashMap<char, Vec<Vec<Point>>>) -> i64 {
    let mut total_price = 0;

    for regions_of_type in regions.values() {
        for region in regions_of_type {
            total_price += calculate_region_price(region);
        }
    }

    total_price
}

fn main() {
    let grid = read_input("plants.txt");
    let regions = find_regions(&grid);
    let total_price = solve(&regions);

    println!("{}", total_price);
}
