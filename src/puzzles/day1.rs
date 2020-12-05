use crate::utils::file_utils::read_lines;
use std::collections::HashSet;

pub fn day1() {
    let mut vector_int: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("./assets/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                vector_int.push(ip.parse().unwrap());
            }
        }
    }

    let (first_num, second_num) = two_num_with_sum(&vector_int, 2020);

    println!(
        "First number: {}, Second number: {}, Multiplication: {}",
        first_num,
        second_num,
        first_num * second_num
    );

    let (first, second, third) = three_num_with_sum(&vector_int, 2020);

    println!(
        "First number: {}, second number: {}, third number: {}, multiplication: {}",
        first,
        second,
        third,
        first * second * third
    );
}

fn three_num_with_sum(vector_int: &[u32], sum: u32) -> (u32, u32, u32) {
    for int in vector_int {
        let difference = sum - int;
        let (first, second) = two_num_with_sum(&vector_int, difference);

        if first != 0 && second != 0 {
            return (int.clone(), first, second);
        }
    }

    (0, 0, 0)
}

fn two_num_with_sum(vector_int: &[u32], sum: u32) -> (u32, u32) {
    let mut encountered_numbers: HashSet<u32> = HashSet::new();

    for int in vector_int {
        if &sum > int {
            let difference = sum - int;

            if encountered_numbers.contains(&difference) {
                return (int.clone(), difference);
            } else {
                encountered_numbers.insert(int.clone());
            }
        }
    }

    (0, 0)
}
