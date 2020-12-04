use crate::utils::file_utils::read_lines;

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

    let (first, second, third) = sum_to_2020(&vector_int);

    println!(
        "First number: {}, second number: {}, third number: {}, multiplication: {}",
        first,
        second,
        third,
        first * second * third
    );
}

fn sum_to_2020(vector_int: &[u32]) -> (&u32, &u32, &u32) {
    let mut min = vector_int[0];
    let mut second_min = min;

    for val in vector_int {
        if val < &min {
            second_min = min;
            min = val.clone();
        }
    }

    // These values help to reduce the number of iterations
    let max = 2020 - min - second_min;
    let second_max = 2020 - min;

    for outer_val in vector_int {
        if outer_val <= &max {
            for inner_val in vector_int {
                if inner_val <= &second_max {
                    for most_inner_val in vector_int {
                        if outer_val + inner_val + most_inner_val == 2020 {
                            return (outer_val, inner_val, most_inner_val);
                        }
                    }
                }
            }
        }
    }

    // No 3 numbers found that sum to 2020
    (&0, &0, &0)
}
