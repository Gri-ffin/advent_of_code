use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Open the file
    let path = Path::new("src/memory.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Define the regex to max mul(x, y) where x and y are numbers
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total = 0;

    for line in reader.lines() {
        let line = line?;

        // Iterate over the captures in the line
        for cap in re.captures_iter(&line) {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();

            total += x * y;
        }
    }

    println!("Total: {}", total);

    Ok(())
}
