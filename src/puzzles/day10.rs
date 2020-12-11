use crate::utils::file_utils::read_lines;

pub fn day10() {
    let mut all_numbers: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("./assets/day10.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                all_numbers.push(ip.parse().unwrap());
            }
        }
    }

    all_numbers.sort();

    let (one_jolt_differences, three_jolt_differences) = find_all_differences(&all_numbers);

    println!(
        "Multiplication of one and three jolt differences: {}",
        one_jolt_differences * three_jolt_differences
    );

    // last number is adapter which is 3 plus the largest number
    all_numbers.push(&all_numbers[all_numbers.len() - 1] + 3);
    let final_permutation = find_permutation(&all_numbers);

    println!(
        "Possible combinations of adapters are: {}",
        final_permutation
    );
}

fn find_all_differences(all_numbers: &[u32]) -> (u32, u32) {
    let mut one_jolt_differences = 0;
    let mut three_jolt_differences = 0;
    let mut prev_num = 0;

    for num in all_numbers {
        if num - prev_num == 1 {
            one_jolt_differences += 1;
        }

        if num - prev_num == 3 {
            three_jolt_differences += 1;
        }

        prev_num = num.clone();
    }

    // For calculating the difference with device which is always 3
    three_jolt_differences += 1;

    (one_jolt_differences, three_jolt_differences)
}

fn find_permutation(all_numbers: &[u32]) -> u64 {
    let mut final_permutation: u64 = 1;
    let mut contiguous_num_count = 1;
    let mut prev_number = 0;

    for num in all_numbers {
        if num - prev_number <= 2 {
            contiguous_num_count += 1;
        } else {
            let combinations = possible_permutations(contiguous_num_count);
            final_permutation = final_permutation * combinations;
            contiguous_num_count = 1;
        }

        prev_number = num.clone();
    }

    final_permutation
}

fn possible_permutations(range: u32) -> u64 {
    // tribonacci sequence
    if range == 1 || range == 2 {
        return 1;
    } else if range == 3 {
        return 2;
    }

    let mut first = 1;
    let mut second = 1;
    let mut third = 2;
    let mut sum = 0;

    let mut i = 4;

    while i <= range {
        sum = first + second + third;
        first = second;
        second = third;
        third = sum;

        i += 1;
    }

    sum
}
