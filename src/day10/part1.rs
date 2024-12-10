use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("trail.txt").unwrap();
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

    let total_score = find_trailhead_scores(&map);
    println!("Total score: {}", total_score);

    Ok(())
}

// Calculate the score for each trailhead
fn find_trailhead_scores(map: &Vec<Vec<u32>>) -> u32 {
    let rows = map.len();
    let cols = map[0].len();
    let mut total_score = 0;

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                total_score += calculate_trailhead_score(map, r, c);
            }
        }
    }

    total_score
}

// Calculate the score for a single trailhead
fn calculate_trailhead_score(map: &Vec<Vec<u32>>, start_r: usize, start_c: usize) -> u32 {
    let rows = map.len();
    let cols = map[0].len();
    // Use a hash set to keep track of visited cells
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    // Use a hash set to keep track of peak positions we have visited for this trailhead
    let mut peak_positions: HashSet<(usize, usize)> = HashSet::new();

    // we need a queue to perform a breadth-first search
    let mut queue: VecDeque<(usize, usize, u32)> = VecDeque::new();
    queue.push_back((start_r, start_c, 0));

    // now for each cell in the queue, we will check if it is a peak
    while let Some((r, c, current_height)) = queue.pop_front() {
        // check if the cell is out of bounds or already visited
        if r >= rows || c >= cols || visited.contains(&(r, c)) {
            continue;
        }

        // check if the cell is at the same height as the trailhead
        if map[r][c] != current_height {
            continue;
        }

        // otherwise mark the cell as visited
        visited.insert((r, c));

        // if the cell is 9 its the peak position based on the problem statement
        if map[r][c] == 9 {
            peak_positions.insert((r, c));
        }

        // possible directions in the problem are defined as (right, down, left, up)
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        // now we check each next cell in the directions of the current cell
        for (dr, dc) in directions {
            let new_r = r as i32 + dr;
            let new_c = c as i32 + dc;

            // if the new cell is within bounds, we add it to the queue
            if new_r >= 0 && new_r < rows as i32 && new_c >= 0 && new_c < cols as i32 {
                // we add the new cell to the queue with the height increased by 1
                queue.push_back((new_r as usize, new_c as usize, current_height + 1));
            }
        }
    }

    // we return the number of 9s we have visited
    peak_positions.len() as u32
}
