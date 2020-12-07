use crate::utils::file_utils::read_lines;
use std::collections::HashSet;

pub fn day6() {
    let mut total_distinct_count = 0;

    if let Ok(lines) = read_lines("./assets/day6.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut previous_line = String::from("");
        let mut intersection_set: HashSet<_> = HashSet::new();

        for line in lines {
            if let Ok(ip) = line {
                if ip != "" {
                    if previous_line == "" {
                        intersection_set = get_intersection_set(&ip, &ip);
                    } else {
                        let local_intersection_set = get_intersection_set(&ip, &previous_line);
                        intersection_set = local_intersection_set
                            .intersection(&intersection_set)
                            .cloned()
                            .collect();
                    }

                    previous_line = ip;
                } else {
                    total_distinct_count = total_distinct_count + intersection_set.len();
                    previous_line = String::from("");
                    intersection_set = HashSet::new();
                }
            }
        }
    }

    println!("Total count: {}", total_distinct_count);
}

fn count_distinct_char(string: &str) -> u32 {
    let mut distinct_chars: HashSet<char> = HashSet::new();
    let str_as_chars = string.chars();

    for character in str_as_chars {
        distinct_chars.insert(character);
    }

    distinct_chars.len() as u32
}

fn get_intersection_set(first_str: &str, second_str: &str) -> HashSet<char> {
    let first_str_chars = first_str.chars();
    let second_str_chars = second_str.chars();

    let mut distinct_chars_first: HashSet<char> = HashSet::new();
    for character in first_str_chars {
        distinct_chars_first.insert(character);
    }

    let mut distinct_chars_second: HashSet<char> = HashSet::new();
    for character in second_str_chars {
        distinct_chars_second.insert(character);
    }

    distinct_chars_first
        .intersection(&distinct_chars_second)
        .cloned()
        .collect()
}
