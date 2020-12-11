use crate::utils::file_utils::read_lines;
use std::collections::HashSet;

struct InstructionSet {
    name: String,
    value: i32,
}

pub fn day8() {
    let mut all_instructions: Vec<InstructionSet> = Vec::new();

    if let Ok(lines) = read_lines("./assets/day8.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let (instruction, mut value) = ip.split_at(3);

                value = value.trim().trim_start_matches("+");

                let instruction = InstructionSet {
                    name: instruction.to_string(),
                    value: value.parse::<i32>().unwrap(),
                };

                all_instructions.push(instruction);
            }
        }
    }

    let (accumulator_value, _is_loop, reached_lines) =
        find_infinite_loop_and_sum(&all_instructions);

    println!(
        "Accumulator value: {}, Read lines count: {}",
        accumulator_value,
        reached_lines.len()
    );

    let correct_accumulator_val =
        correct_infinite_loop_and_sum(&mut all_instructions, reached_lines);

    println!("After correction accumulator: {}", correct_accumulator_val);
}

fn find_infinite_loop_and_sum(all_instructions: &[InstructionSet]) -> (i32, bool, HashSet<i32>) {
    let mut accumulator: i32 = 0;
    let mut reached_lines: HashSet<i32> = HashSet::new();
    let mut iterator: usize = 0;

    while iterator < all_instructions.len() {
        let is_line_unique = reached_lines.insert(iterator as i32);

        if !is_line_unique {
            return (accumulator, true, reached_lines);
        }

        let current_item = &all_instructions[iterator];

        if current_item.name == "acc" {
            accumulator = accumulator + current_item.value;
            iterator = iterator + 1;
        } else if current_item.name == "jmp" {
            iterator = (iterator as i32 + current_item.value) as usize;
        } else {
            iterator = iterator + 1;
        }
    }

    (accumulator, false, reached_lines)
}

// Change only one 'jmp' to 'nop' OR only one 'nop' to 'jmp' and the loop should end
fn correct_infinite_loop_and_sum(
    all_instructions: &mut [InstructionSet],
    reached_lines: HashSet<i32>,
) -> i32 {
    for line in reached_lines {
        let name_line_item = all_instructions[line as usize].name.clone();
        let value_line_item = all_instructions[line as usize].value;

        if name_line_item == "jmp" {
            all_instructions[line as usize] = InstructionSet {
                name: String::from("nop"),
                value: value_line_item,
            };

            let (accumulator, is_loop, _executed_lines) =
                find_infinite_loop_and_sum(&all_instructions);

            if !is_loop {
                return accumulator;
            } else {
                all_instructions[line as usize] = InstructionSet {
                    name: String::from("jmp"),
                    value: value_line_item,
                };
            }
        } else if name_line_item == "nop" {
            all_instructions[line as usize] = InstructionSet {
                name: String::from("jmp"),
                value: value_line_item,
            };

            let (accumulator, is_loop, _executed_lines) =
                find_infinite_loop_and_sum(&all_instructions);

            if !is_loop {
                return accumulator;
            } else {
                all_instructions[line as usize] = InstructionSet {
                    name: String::from("nop"),
                    value: value_line_item,
                };
            }
        }
    }

    0
}
