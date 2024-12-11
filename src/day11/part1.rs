fn main() {
    let input = "6 11 33023 4134 564 0 8922422 688775";

    let initial_stones: Vec<u64> = input
        .split_whitespace()
        .map(|num| num.parse::<u64>().expect("Invalid number"))
        .collect();

    let blinks = 25;

    let final_count = evolve_stones(initial_stones, blinks);

    println!("After {} blinks, there are {} stones.", blinks, final_count);
}

// evolve stones over a given number of blinks based on the rules given in the problem
fn evolve_stones(mut stones: Vec<u64>, blinks: usize) -> usize {
    for _ in 0..blinks {
        let mut next_stones = Vec::new();
        for stone in stones {
            // if the stone is 0 we push 1
            if stone == 0 {
                next_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                // otherwise we split the number in half and push the two halves
                let digits = stone.to_string();
                let mid = digits.len() / 2;
                let left: u64 = digits[0..mid].parse().unwrap();
                let right: u64 = digits[mid..].parse().unwrap();
                next_stones.push(left);
                next_stones.push(right);
            } else {
                // otherwise we just multiply by 2024
                next_stones.push(stone * 2024);
            }
        }
        // replace old stones with new stones vec
        stones = next_stones;
    }
    stones.len()
}
