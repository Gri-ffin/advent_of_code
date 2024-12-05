use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("report.txt").expect("Failed to read the file");

    // Split the input into rules and updates
    let sections: Vec<&str> = input.split("\n\n").collect();
    let rules_str = sections[0];
    let updates_str = sections[1];

    // Parse the rules to a HashMap of page numbers
    let mut rules = HashMap::new();
    for line in rules_str.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        let before: i32 = parts[0].parse().expect("Invalid rule format");
        let after: i32 = parts[1].parse().expect("Invalid rule format");

        rules
            .entry(before)
            .or_insert_with(HashSet::new)
            .insert(after);
    }

    // Parse the updates to a Vec of Vec of page numbers
    let updates: Vec<Vec<i32>> = updates_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.trim().parse().expect("Invalid page number"))
                .collect()
        })
        .collect();

    // Check each update and sum the middle page numbers for valid updates
    let mut total = 0;

    for update in updates {
        if is_valid_order(&update, &rules) {
            let middle_index = update.len() / 2;
            let middle_page = update[middle_index];
            total += middle_page;
        }
    }

    // Output the result
    println!("Sum of middle page numbers: {}", total);
}

// Function to check if the update is valid according to the rules
fn is_valid_order(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    // Iterate over all pairs in the update
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            let page_before = update[i];
            let page_after = update[j];

            // page_before must come before page_after, so check the rule
            if let Some(after_pages) = rules.get(&page_before) {
                if after_pages.contains(&page_after) {
                    continue;
                }
            }

            // otherwise the page order doesn't respect the rule
            return false;
        }
    }
    true
}
