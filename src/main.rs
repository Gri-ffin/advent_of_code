use std::collections::HashMap;
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

    // Create a hashmap to keep references
    let mut right_count = HashMap::new();
    for &num in &right_vec {
        *right_count.entry(num).or_insert(0) += 1;
    }

    let answer: i32 = left_vec
        .iter()
        .map(|&num| {
            if let Some(&count) = right_count.get(&num) {
                count * num
            } else {
                0
            }
        })
        .sum();

    println!("the answer is {}", answer);

    Ok(())
}
