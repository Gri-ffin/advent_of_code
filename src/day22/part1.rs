use std::fs::File;
use std::io::{self, BufRead};

fn next_secret(mut secret: u32) -> u32 {
    // Step 1: Multiply by xor 64, mix, prune
    secret ^= secret.wrapping_mul(64);
    secret %= 16_777_216;

    // Step 2: Divide by xor 32 (integer division), mix, prune
    secret ^= secret / 32;
    secret %= 16_777_216;

    // Step 3: Multiply by xor 2048, mix, prune
    secret ^= secret.wrapping_mul(2048);
    secret %= 16_777_216;

    secret
}

fn compute_2000th_secret(initial_secret: u32) -> u32 {
    let mut secret = initial_secret;
    for _ in 0..2000 {
        secret = next_secret(secret);
    }
    secret
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut sum: u64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if let Ok(initial_secret) = line.trim().parse::<u32>() {
            // Compute
            sum += compute_2000th_secret(initial_secret) as u64;
        }
    }

    println!("Sum of the 2000th secret numbers: {}", sum);
}
