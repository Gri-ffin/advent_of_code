use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("towels.txt").unwrap();

    // Split the input into unlimited towels and towel patterns
    let mut sections = input.split("\n\n");
    let unlimited_towels = sections.next().unwrap();
    let towel_designs = sections.next().unwrap();

    // Parse the unlimited towels into a vec of String and the towel designs into a vec of &str
    let unlimimted_towels: Vec<String> = unlimited_towels
        .split(',')
        .map(|towel| towel.trim().to_string())
        .collect();

    let towel_designs: Vec<&str> = towel_designs.lines().map(str::trim).collect();

    // go through each towel design and check if it can be formed by the unlimited towels
    let mut memo = HashMap::new();
    let possible_designs = towel_designs
        .iter()
        .filter(|&&design| can_form_design(design, &unlimimted_towels, &mut memo))
        .count();

    println!("{}", possible_designs);
}

fn can_form_design(
    design: &str,
    towel_patterns: &[String],
    memo: &mut HashMap<String, bool>,
) -> bool {
    // Check if we have already computed the result for this design
    if let Some(&result) = memo.get(design) {
        return result;
    }

    // If the design is empty it can be formed trivially
    if design.is_empty() {
        return true;
    }

    // Try to match each towel pattern with the design
    for pattern in towel_patterns {
        if design.starts_with(pattern) {
            let remaining = &design[pattern.len()..];
            if can_form_design(remaining, towel_patterns, memo) {
                memo.insert(design.to_string(), true);
                return true;
            }
        }
    }

    // If no towel pattern can be matched with the design
    memo.insert(design.to_string(), false);
    return false;
}
