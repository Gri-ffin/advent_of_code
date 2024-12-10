use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("trail.txt")?;
    let reader = BufReader::new(file);

    let map: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let total_rating = find_trailhead_ratings(&map);
    println!("Sum of trailhead ratings: {}", total_rating);

    Ok(())
}

fn find_trailhead_ratings(map: &Vec<Vec<u32>>) -> u32 {
    let rows = map.len();
    let cols = map[0].len();
    let mut total_rating = 0;

    // go through each cell and calculate the rating for each trailhead
    for r in 0..rows {
        for c in 0..cols {
            // the trailhead start at a cell with a value of 0
            if map[r][c] == 0 {
                // Calculate rating for this trailhead
                total_rating += calculate_trailhead_rating(map, r, c);
            }
        }
    }

    total_rating
}

fn calculate_trailhead_rating(map: &Vec<Vec<u32>>, start_r: usize, start_c: usize) -> u32 {
    let mut distinct_trails = HashSet::new();

    // use DFS from the trailhead to find all distinct trails
    let mut current_trail = Vec::new();
    current_trail.push((start_r, start_c));
    dfs(
        map,
        start_r,
        start_c,
        0,
        &mut current_trail,
        &mut distinct_trails,
    );

    // Return number of distinct trails
    distinct_trails.len() as u32
}

fn dfs(
    map: &Vec<Vec<u32>>,
    r: usize,
    c: usize,
    current_height: u32,
    current_trail: &mut Vec<(usize, usize)>,
    distinct_trails: &mut HashSet<Vec<(usize, usize)>>,
) {
    let rows = map.len();
    let cols = map[0].len();

    // Reached peak, record the trail
    if current_height == 9 {
        distinct_trails.insert(current_trail.clone());
        return;
    }

    // Explore adjacent cells acccording to the directions allowed by the problem statement
    // (up, right, down, left)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (dr, dc) in directions {
        let new_r = r as isize + dr;
        let new_c = c as isize + dc;

        // Check bounds and height progression
        if new_r >= 0 && new_r < rows as isize && new_c >= 0 && new_c < cols as isize {
            let new_r = new_r as usize;
            let new_c = new_c as usize;

            // Ensure height increases by exactly 1 at each step ex: 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9
            if map[new_r][new_c] == current_height + 1 {
                // Add to the trail and continue the search
                current_trail.push((new_r, new_c));
                dfs(
                    map,
                    new_r,
                    new_c,
                    map[new_r][new_c],
                    current_trail,
                    distinct_trails,
                );
                current_trail.pop(); // Backtrack to explore other paths
            }
        }
    }
}
