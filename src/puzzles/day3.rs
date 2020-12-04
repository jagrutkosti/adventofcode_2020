use crate::utils::file_utils::read_lines;

pub fn day3() {
    // Answers to the questions
    // Right 1, down 1. 79
    // Right 3, down 1. 216
    // Right 5, down 1. 91
    // Right 7, down 1. 96
    // Right 1, down 2. 45
    let mut tree_count = 0;

    if let Ok(lines) = read_lines("./assets/day3.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut index = 0;
        let mut line_skip_counter = 0;

        for line in lines {
            if let Ok(ip) = line {
                if line_skip_counter % 2 == 0 {
                    let str_as_chars: Vec<char> = ip.chars().collect();
                    let length = str_as_chars.len();

                    if str_as_chars[index % length] == '#' {
                        tree_count += 1;
                    }

                    index = index + 1;
                }

                line_skip_counter += 1;
            }
        }
    }

    println!("Number of trees: {}", tree_count);
}
