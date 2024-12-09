use std::fs;

/// [Triangular numbers](https://en.wikipedia.org/wiki/Triangular_number)
const EXTRA: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

pub fn compact_disk(disk: &[usize]) -> usize {
    // Start at the first free block and the last file.
    let mut free = 0;
    let mut file = disk.len() + disk.len() % 2;

    let mut available = 0;
    let mut needed = 0;

    let mut block = 0;
    let mut checksum = 0;

    while free < file {
        // Take as much space as possible from the current free block range.
        let size = needed.min(available);
        (checksum, block) = update(checksum, block, file, size);
        available -= size;
        needed -= size;

        // One or both of "available" and "free" could be zero.
        if needed == 0 {
            file -= 2;
            needed = disk[file];
        }

        // When moving to the next free block, add the checksum for the file we're skipping over.
        if available == 0 {
            let size = disk[free];
            (checksum, block) = update(checksum, block, free, size);
            available = disk[free + 1];
            free += 2;
        }
    }

    // Account for any remaining file blocks left over.
    (checksum, _) = update(checksum, block, file, needed);
    checksum
}

#[inline]
// Update the checksum and block number for the next file.
fn update(checksum: usize, block: usize, index: usize, size: usize) -> (usize, usize) {
    let id = index / 2;
    let extra = block * size + EXTRA[size];
    (checksum + id * extra, block + size)
}

fn main() {
    // Read the input file
    let input = fs::read_to_string("disk.txt").expect("Failed to read input file");

    // Parse the input into a vector of usize
    let disk: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid character in input") as usize)
        .collect();

    // Call part1 with the parsed disk map
    let result = compact_disk(&disk);

    println!("Filesystem checksum: {}", result);
}
