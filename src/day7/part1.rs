use itertools::Itertools;
use std::fs;

fn parse_file(file: &str) -> Vec<(usize, Vec<usize>)> {
    let content = fs::read_to_string(file).unwrap();
    content
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let target = parts[0].parse::<usize>().unwrap();
            let numbers: Vec<usize> = parts[1]
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            (target, numbers)
        })
        .collect()
}

// Takes a Vector of operations ex: [+, *] or [*, *] ... and a Vector of numbers ex: [1, 2, 3] and returns the result
fn evaluate_expression(numbers: &[usize], operators: &[char]) -> usize {
    let mut result = numbers[0];
    // Iterate over the operators and apply the operation to the result based on the operation
    for (i, &operation) in operators.iter().enumerate() {
        match operation {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            _ => unreachable!(),
        }
    }
    result
}

// checks if is a valid equation
fn is_valid_equation(numbers: &[usize], target: usize) -> bool {
    // Number of operations is always one less than the number of numbers
    let operations_count = numbers.len() - 1;

    for operators in (0..operations_count)
        .map(|_| ['+', '*'].iter())
        // Generate all possible combinations of operators
        .multi_cartesian_product()
    {
        if evaluate_expression(
            numbers,
            &operators.iter().cloned().copied().collect::<Vec<_>>(),
        ) == target
        {
            return true;
        }
    }
    false
}

fn main() {
    let equations = parse_file("equations.txt");
    let mut total = 0;
    for (target, numbers) in equations {
        if is_valid_equation(&numbers, target) {
            total += target;
        }
    }
    println!("Total calibration result: {}", total);
}
