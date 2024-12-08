use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "freq.txt";
    let mut antennas = Vec::new();
    let mut map_height = 0;
    let mut map_width = 0;

    if let Ok(lines) = read_lines(path) {
        for (y, line) in lines.enumerate() {
            if let Ok(row) = line {
                map_height += 1;
                map_width = row.len();
                for (x, ch) in row.chars().enumerate() {
                    if ch != '.' {
                        antennas.push((x as i32, y as i32, ch));
                    }
                }
            }
        }
    }

    let mut antinodes = HashSet::new();

    // Iterate over all pairs of antennas
    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (x1, y1, freq1) = antennas[i];
            let (x2, y2, freq2) = antennas[j];

            // Check if they have the same frequency
            if freq1 == freq2 {
                // Calculate the midpoint
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Calculate potential antinodes
                let x3 = x1 - dx;
                let y3 = y1 - dy;
                let x4 = x2 + dx;
                let y4 = y2 + dy;

                // Add valid antinodes to the set (within map bounds)
                if is_within_bounds(x3, y3, map_width as i32, map_height as i32) {
                    antinodes.insert((x3, y3));
                }
                if is_within_bounds(x4, y4, map_width as i32, map_height as i32) {
                    antinodes.insert((x4, y4));
                }
            }
        }
    }

    // Print the count of unique antinodes
    println!("Unique antinode locations: {}", antinodes.len());
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Helper to check if a point is within the map bounds
fn is_within_bounds(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}
