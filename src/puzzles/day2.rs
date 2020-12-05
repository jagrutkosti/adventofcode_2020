use crate::utils::file_utils::read_lines;

pub fn day2() {
    let mut valid_passwords_count = 0;

    if let Ok(lines) = read_lines("./assets/day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let sub_str: Vec<&str> = ip.split(" ").collect();
                let occurences: Vec<&str> = sub_str[0].split("-").collect();

                // Should handle exception here!!
                let min_occurence = occurences[0].parse().unwrap();
                let max_occurence = occurences[1].parse().unwrap();
                let search_char: Vec<&str> = sub_str[1].split(":").collect();
                // let result = is_valid_password(
                //     &min_occurence,
                //     &max_occurence,
                //     &search_char[0].chars().next().unwrap(),
                //     sub_str[2],
                // );
                let result = is_valid_password_2(
                    &min_occurence,
                    &max_occurence,
                    &search_char[0].chars().next().unwrap(),
                    sub_str[2],
                );

                if result {
                    valid_passwords_count += 1;
                }
            }
        }
    }

    println!("Number of valid passwords: {}", valid_passwords_count);
}

fn is_valid_password(
    min_occurence: &u32,
    max_occurence: &u32,
    search_char: &char,
    string: &str,
) -> bool {
    let str_as_chars = string.chars();
    let mut count = 0;

    for c in str_as_chars {
        if &c == search_char {
            count += 1;
        }
    }

    if min_occurence <= &count && &count <= max_occurence {
        true
    } else {
        false
    }
}

fn is_valid_password_2(
    first_position: &u32,
    second_position: &u32,
    search_char: &char,
    string: &str,
) -> bool {
    let str_as_chars: Vec<char> = string.chars().collect();
    let mut count = 0;
    let second: usize = (second_position - 1) as usize;
    let first: usize = (first_position - 1) as usize;

    if &str_as_chars[first] == search_char {
        count += 1;
    }

    if &str_as_chars[second] == search_char {
        count += 1;
    }

    if count == 1 {
        true
    } else {
        false
    }
}
