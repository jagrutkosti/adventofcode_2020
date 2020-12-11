use crate::puzzles::day1::two_num_with_sum;
use crate::utils::file_utils::read_lines;

pub fn day9() {
    let mut all_numbers: Vec<u64> = Vec::new();

    if let Ok(lines) = read_lines("./assets/day9.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                all_numbers.push(ip.parse().unwrap());
            }
        }
    }

    let invalid_number = find_number_with_not_sum(&all_numbers);

    println!("The number with invalid sum: {}", invalid_number);

    let max_from_contiguous_sum = find_minmaxsum_in_contiguous_sum(&all_numbers, &invalid_number);

    println!(
        "The min-max sum in contiguous sum for invalid number: {}",
        max_from_contiguous_sum
    );
}

fn find_number_with_not_sum(all_numbers: &[u64]) -> u64 {
    let mut preamble: Vec<u64> = Vec::with_capacity(25);
    let mut invalid_number = 0;

    for number in all_numbers {
        if preamble.len() < 25 {
            preamble.push(number.clone());
        } else {
            let (first_num, second_num) = two_num_with_sum(&preamble, number.clone());

            if first_num == 0 && second_num == 0 {
                invalid_number = number.clone();
            }

            preamble.remove(0);
            preamble.push(number.clone());
        }
    }

    invalid_number
}

fn find_minmaxsum_in_contiguous_sum(all_numbers: &[u64], desired_sum: &u64) -> u64 {
    let mut sum: u64 = 0;
    let mut contiguous_numbers: Vec<u64> = Vec::new();
    let mut min_max_sum: u64 = 0;

    for number in all_numbers {
        sum += number;

        if &sum > desired_sum {
            while &sum >= desired_sum {
                sum = sum - contiguous_numbers.remove(0);
            }
        }

        contiguous_numbers.push(number.clone());

        if &sum == desired_sum {
            min_max_sum = find_min_max_sum(&contiguous_numbers);
            break;
        }
    }

    min_max_sum
}

fn find_min_max_sum(vector: &[u64]) -> u64 {
    let mut max = 0;
    let mut min = vector[0];

    for num in vector {
        if num > &max {
            max = num.clone();
        }

        if num < &min {
            min = num.clone();
        }
    }

    min + max
}
