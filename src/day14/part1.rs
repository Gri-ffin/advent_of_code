use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let input_path = "robots.txt";
    let robots = read_input(input_path).unwrap();

    let width = 101;
    let height = 103;
    let steps = 100;

    let final_positions = simulate_robots(&robots, width, height, steps);

    let safety_factor = calculate_safety_factor(&final_positions, width, height);

    println!("Safety factor: {}", safety_factor);
}

fn read_input<P: AsRef<Path>>(filename: P) -> io::Result<Vec<((i32, i32), (i32, i32))>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut robots = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if let Some((p, v)) = line.split_once(" v=") {
            let p = parse_coords(p.strip_prefix("p=").unwrap());
            let v = parse_coords(v);
            robots.push((p, v));
        }
    }
    Ok(robots)
}

fn parse_coords(coords: &str) -> (i32, i32) {
    let parts: Vec<i32> = coords.split(",").map(|s| s.parse().unwrap()).collect();
    (parts[0], parts[1])
}

fn simulate_robots(
    robots: &[((i32, i32), (i32, i32))],
    width: i32,
    height: i32,
    steps: i32,
) -> Vec<(i32, i32)> {
    robots
        .iter()
        .map(|&((x, y), (vx, vy))| {
            let new_x = (x + steps * vx).rem_euclid(width);
            let new_y = (y + steps * vy).rem_euclid(height);
            (new_x, new_y)
        })
        .collect()
}

fn calculate_safety_factor(positions: &[(i32, i32)], width: i32, height: i32) -> i32 {
    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut quadrants = [0; 4];

    for &(x, y) in positions {
        if x == mid_x || y == mid_y {
            continue;
        } else if x > mid_x && y < mid_y {
            quadrants[0] += 1;
        } else if x < mid_x && y < mid_y {
            quadrants[1] += 1;
        } else if x < mid_x && y > mid_y {
            quadrants[2] += 1;
        } else if x > mid_x && y > mid_y {
            quadrants[3] += 1;
        }
    }

    quadrants.iter().product()
}
