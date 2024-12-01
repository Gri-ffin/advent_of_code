use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    // Open the file
    let path = Path::new("src/input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;

        // Split the lines into words and parse them into numbers
        let words: Vec<&str> = line.split_whitespace().collect();
        if let (Ok(left), Ok(right)) = (words[0].parse::<i32>(), words[1].parse::<i32>()) {
            left_vec.push(left);
            right_vec.push(right);
        }
    }

    left_vec.sort();
    right_vec.sort();

    let mut answer: i32 = 0;

    for i in 0..left_vec.len() {
        let count = (left_vec[i] - right_vec[i]).abs();

        answer += count;
    }

    println!("the answer is {}", answer);

    Ok(())
}
