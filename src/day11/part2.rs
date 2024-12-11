use std::collections::HashMap;

fn main() {
    let input = "6 11 33023 4134 564 0 8922422 688775";

    let initial_stones: Vec<u64> = input
        .split_whitespace()
        .map(|num| num.parse::<u64>().expect("Invalid number"))
        .collect();

    // Number of blinks (use 75 for the second part)
    let blinks = 75;

    let final_count = evolve_stones_optimized(initial_stones, blinks);

    println!("After {} blinks, there are {} stones.", blinks, final_count);
}

// we use a hashmap for the second part instead of storing every additional stone
// Basically we keep track of how many stones there are using a hashmap
// and in the next iteration we get that value and apply the rule over it
// so not to keep a very large vec
// ex {1: 3, 6: 1} is better than [1, 1, 1, 6]
// this means that in our current blink we have 3 stones of 1 and one stone of 6
// then in the next blink we get the key and value and apply the rule to it
// basically we know all of the zeroes will become -> 1 so we just add the value
// of the key 0 to the key 1 etc...
fn evolve_stones_optimized(initial_stones: Vec<u64>, blinks: usize) -> usize {
    // Initialize frequency map
    let mut stone_counts: HashMap<u64, usize> = HashMap::new();
    for stone in initial_stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut next_stone_counts: HashMap<u64, usize> = HashMap::new();
        for (&stone, &count) in &stone_counts {
            if stone == 0 {
                *next_stone_counts.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let digits = stone.to_string();
                let mid = digits.len() / 2;
                let left: u64 = digits[0..mid].parse().unwrap();
                let right: u64 = digits[mid..].parse().unwrap();
                *next_stone_counts.entry(left).or_insert(0) += count;
                *next_stone_counts.entry(right).or_insert(0) += count;
            } else {
                let new_stone = stone * 2024;
                *next_stone_counts.entry(new_stone).or_insert(0) += count;
            }
        }

        stone_counts = next_stone_counts;
    }

    // Sum up the counts of all stones
    stone_counts.values().sum()
}
