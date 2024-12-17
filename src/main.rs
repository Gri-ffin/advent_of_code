use std::fs;

fn main() {
    let content = fs::read_to_string("register.txt").unwrap();

    let mut lines = content.lines();

    let reg_a: i32 = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let reg_b: i32 = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let reg_c: i32 = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    lines.next();

    let program: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut a = reg_a;
    let mut b = reg_b;
    let mut c = reg_c;

    let mut ip = 0;

    let mut output = Vec::new();

    while ip < program.len() {
        let opcode = program[ip] as usize;
        let operand = program[ip + 1];
        ip += 2;

        match opcode {
            0 => {
                // adv instruction: A = A / (2^operand)
                let divisor = 2_i32.pow(operand as u32);
                a /= divisor;
            }
            1 => {
                // bxl instruction: B = B ^ operand
                b = b ^ operand;
            }
            2 => {
                // bst instruction: B = operand % 8
                b = operand % 8;
            }
            3 => {
                // jnz instruction: Jump if A != 0
                if a != 0 {
                    ip = operand as usize;
                }
            }
            4 => {
                // bxc instruction: B = B ^ C (operand is ignored)
                b = b ^ c;
            }
            5 => {
                // out instruction: Output operand % 8
                output.push(operand % 8);
            }
            6 => {
                // bdv instruction: B = A / (2^operand)
                let divisor = 2_i32.pow(operand as u32);
                b = a / divisor;
            }
            7 => {
                // cdv instruction: C = A / (2^operand)
                let divisor = 2_i32.pow(operand as u32);
                c = a / divisor;
            }
            _ => panic!("Unknown opcode: {}", opcode),
        }
    }

    let output_str = output
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", output_str);
}
