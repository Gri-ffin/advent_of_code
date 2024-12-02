use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Open the file
    let path = Path::new("src/reports.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Read the reports to vectors
    let reports: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().expect("Invalid number"))
                .collect()
        })
        .collect();

    let safe_count = reports
        .iter()
        .filter(|&report| dampner_safety(report))
        .count();

    println!("safe count is {}", safe_count);

    Ok(())
}

fn dampner_safety(report: &Vec<i32>) -> bool {
    // if the report is already safe return true
    if safety_check(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if safety_check(&modified_report) {
            return true;
        }
    }

    false
}

// Checks if the report is safe
fn safety_check(report: &Vec<i32>) -> bool {
    // report is strictly increasing or strictly decreasing
    let differences: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();

    // differences are valid (between -3 and -1 or between 1 and 3)
    let all_valid = differences
        .iter()
        .all(|&diff| (diff >= 1 && diff <= 3) || (diff <= -1 && diff >= -3));

    // report is either strictly increasing or strictly decreasing
    let all_increasing = differences.iter().all(|&d| d > 0);
    let all_decreasing = differences.iter().all(|&d| d < 0);

    // Report is only safe if ALL differences are valid AND it's either all increasing or all decreasing
    all_valid && (all_increasing || all_decreasing)
}
