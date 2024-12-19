use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: (usize, usize),
    direction: usize, // // 0: East, 1: South, 2: West, 3: North
    score: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(position: (usize, usize), direction: usize, score: usize) -> Self {
        Self {
            position,
            direction,
            score,
        }
    }
}

fn main() {
    let input = fs::read_to_string("maze.txt").unwrap();
    let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    // Find the positions of the end and start
    for (y, row) in maze.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == 'S' {
                start = (x, y);
            } else if tile == 'E' {
                end = (x, y);
            }
        }
    }

    // Directions = East, South, West, North
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    // Implement Priority queue for A* search
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, State::new(start, 0, 0))));

    // Set to track visited states
    let mut visited = HashSet::new();

    while let Some(Reverse((current_score, state))) = heap.pop() {
        // If we reach the end, print the score and break
        if state.position == end {
            println!("Score: {}", current_score);
            return;
        }

        // Skip if the state has been visited with a lower score
        if !visited.insert((state.position, state.direction)) {
            continue;
        }

        // Try moving forward first
        let (dx, dy) = directions[state.direction];
        let new_position = (
            (state.position.0 as isize + dx) as usize,
            (state.position.1 as isize + dy) as usize,
        );

        if maze[new_position.1][new_position.0] != '#' {
            heap.push(Reverse((
                current_score + 1,
                State::new(new_position, state.direction, state.score + 1),
            )));
        }

        // Try rotating clockwise and counter closewise
        for &rotation in &[-1, 1] {
            let new_direction = (state.direction as isize + rotation).rem_euclid(4) as usize;
            heap.push(Reverse((
                current_score + 1000,
                State::new(state.position, new_direction, state.score + 1000),
            )));
        }
    }

    println!("No solution found");
}
