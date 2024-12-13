use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct ClawMachine {
    dx_a: i32,
    dy_a: i32,
    dx_b: i32,
    dy_b: i32,
    prize_x: i32,
    prize_y: i32,
}

fn parse_button_line(line: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let movements: Vec<&str> = parts[1].split(',').collect();
    if movements.len() != 2 {
        return None;
    }
    let mut dx = None;
    let mut dy = None;
    for movement in movements {
        let movement = movement.trim();
        if movement.starts_with("X") {
            let value = movement[1..].parse::<i32>().ok()?;
            dx = Some(value);
        } else if movement.starts_with("Y") {
            let value = movement[1..].parse::<i32>().ok()?;
            dy = Some(value);
        }
    }
    if let (Some(dx_val), Some(dy_val)) = (dx, dy) {
        Some((dx_val, dy_val))
    } else {
        None
    }
}

fn parse_prize_line(line: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let movements: Vec<&str> = parts[1].split(',').collect();
    if movements.len() != 2 {
        return None;
    }
    let mut prize_x = None;
    let mut prize_y = None;
    for movement in movements {
        let movement = movement.trim();
        if movement.starts_with("X=") {
            let value = movement[2..].parse::<i32>().ok()?;
            prize_x = Some(value);
        } else if movement.starts_with("Y=") {
            let value = movement[2..].parse::<i32>().ok()?;
            prize_y = Some(value);
        }
    }
    if let (Some(x), Some(y)) = (prize_x, prize_y) {
        Some((x, y))
    } else {
        None
    }
}

fn read_claw_machines<P: AsRef<Path>>(filename: P) -> io::Result<Vec<ClawMachine>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut machines = Vec::new();
    let mut current_machine_lines: Vec<String> = Vec::new(); // Specify the type explicitly

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            if current_machine_lines.len() == 3 {
                let button_a = parse_button_line(&current_machine_lines[0]);
                let button_b = parse_button_line(&current_machine_lines[1]);
                let prize = parse_prize_line(&current_machine_lines[2]);

                if let (Some((dx_a, dy_a)), Some((dx_b, dy_b)), Some((prize_x, prize_y))) =
                    (button_a, button_b, prize)
                {
                    machines.push(ClawMachine {
                        dx_a,
                        dy_a,
                        dx_b,
                        dy_b,
                        prize_x,
                        prize_y,
                    });
                }
            }
            current_machine_lines.clear();
        } else {
            current_machine_lines.push(line);
        }
    }

    // Handle the last machine if there's no trailing newline
    if current_machine_lines.len() == 3 {
        let button_a = parse_button_line(&current_machine_lines[0]);
        let button_b = parse_button_line(&current_machine_lines[1]);
        let prize = parse_prize_line(&current_machine_lines[2]);

        if let (Some((dx_a, dy_a)), Some((dx_b, dy_b)), Some((prize_x, prize_y))) =
            (button_a, button_b, prize)
        {
            machines.push(ClawMachine {
                dx_a,
                dy_a,
                dx_b,
                dy_b,
                prize_x,
                prize_y,
            });
        }
    }

    Ok(machines)
}

fn solve_machine(machine: &ClawMachine) -> Option<i32> {
    let mut min_tokens: Option<i32> = None;

    for x_a in 0..=100 {
        // Calculate the required presses for button B in X direction
        let x_b_numerator = machine.prize_x - x_a * machine.dx_a;
        if machine.dx_b == 0 {
            if x_b_numerator != 0 {
                continue; // No solution for this X
            }
            // If dx_b is 0 and x_b_numerator is 0, iterate x_b for the Y equation
            for x_b in 0..=100 {
                let y_total = x_a * machine.dy_a + x_b * machine.dy_b;
                if y_total == machine.prize_y {
                    let cost = x_a * 3 + x_b;
                    min_tokens = Some(min_tokens.map_or(cost, |min| min.min(cost)));
                }
            }
            continue;
        }

        if x_b_numerator % machine.dx_b != 0 {
            continue; // No valid x_b for this x_a
        }
        let x_b = x_b_numerator / machine.dx_b;

        // Check if x_b is within the range
        if x_b < 0 || x_b > 100 {
            continue;
        }

        // Validate the Y direction
        let y_total = x_a * machine.dy_a + x_b * machine.dy_b;
        if y_total == machine.prize_y {
            let cost = x_a * 3 + x_b;
            min_tokens = Some(min_tokens.map_or(cost, |min| min.min(cost)));
        }
    }

    min_tokens
}

fn main() {
    let filename = "prizes.txt";

    let machines = match read_claw_machines(filename) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            return;
        }
    };

    let mut total_tokens = 0;
    let mut total_prizes = 0;

    for (index, machine) in machines.iter().enumerate() {
        if let Some(tokens) = solve_machine(machine) {
            println!("Machine {}: Can win with {} tokens.", index + 1, tokens);
            total_tokens += tokens;
            total_prizes += 1;
        } else {
            println!("Machine {}: Cannot win the prize.", index + 1);
        }
    }

    println!("\nTotal prizes that can be won: {}", total_prizes);
    println!("Minimum total tokens required: {}", total_tokens);
}
