use std::collections::HashMap;
use std::fs;

fn main() {
    // Read input from file
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let parts: Vec<&str> = input.split("\n\n").collect();

    if parts.len() != 2 {
        panic!("Invalid input format");
    }

    let map_lines: Vec<&str> = parts[0].lines().collect();
    let movements: String = parts[1].lines().collect();

    let mut warehouse: Vec<Vec<char>> = map_lines
        .iter()
        .map(|&line| line.chars().collect())
        .collect();

    let mut robot_position = (0, 0);

    // Locate the robot's initial position
    for (i, row) in warehouse.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '@' {
                robot_position = (i, j);
                break;
            }
        }
    }

    // Directions map for robot movement
    let directions: HashMap<char, (isize, isize)> =
        HashMap::from([('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]);

    for movement in movements.chars() {
        if let Some(&(di, dj)) = directions.get(&movement) {
            let (next_i, next_j) = (
                (robot_position.0 as isize + di) as usize,
                (robot_position.1 as isize + dj) as usize,
            );

            // Check if robot can move
            if warehouse[next_i][next_j] == '.' {
                warehouse[robot_position.0][robot_position.1] = '.';
                robot_position = (next_i, next_j);
                warehouse[robot_position.0][robot_position.1] = '@';
            } else if warehouse[next_i][next_j] == 'O' {
                // Try to push the box
                let (box_next_i, box_next_j) = (
                    (next_i as isize + di) as usize,
                    (next_j as isize + dj) as usize,
                );

                if warehouse[box_next_i][box_next_j] == '.' {
                    warehouse[robot_position.0][robot_position.1] = '.';
                    warehouse[next_i][next_j] = '@';
                    warehouse[box_next_i][box_next_j] = 'O';
                    robot_position = (next_i, next_j);
                }
            }
        }
    }

    // Calculate the sum of GPS coordinates of all boxes
    let mut gps_sum = 0;

    for (i, row) in warehouse.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'O' {
                gps_sum += 100 * i + j;
            }
        }
    }

    println!("Sum of GPS coordinates: {}", gps_sum);
}
