use crate::utils::file_utils::read_lines;
use std::collections::HashSet;

pub fn day5() {
    let mut max_seat_id = 0;
    // maximum seat id with row from 0..127 and col from 0..7 and using the formula (row * 8) + col
    let mut seat_ids_set: HashSet<u32> = (0..1023).into_iter().collect();

    if let Ok(lines) = read_lines("./assets/day5.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let (row_str, column_str) = ip.split_at(7);

                let row_int = convert_binary_str_to_int(row_str, 'F', 'B');
                let column_int = convert_binary_str_to_int(column_str, 'L', 'R');

                let seat_id = (row_int * 8) + column_int;

                if max_seat_id < seat_id {
                    max_seat_id = seat_id;
                }

                seat_ids_set.remove(&seat_id);
            }
        }
    }

    println!("Maximum seat id: {}", max_seat_id);

    let my_seat_id = find_seat_id(&seat_ids_set, &max_seat_id);

    println!("My seat id: {}", my_seat_id);
}

fn convert_binary_str_to_int(string: &str, zero_as: char, one_as: char) -> u32 {
    let mut bin_num_str = String::from("");
    let string_chars = string.chars();

    for character in string_chars {
        if character == zero_as {
            bin_num_str.push('0');
        }

        if character == one_as {
            bin_num_str.push('1');
        }
    }

    u32::from_str_radix(&bin_num_str, 2).unwrap()
}

fn find_seat_id(seat_ids_set: &HashSet<u32>, max_seat_id: &u32) -> u32 {
    for id in seat_ids_set {
        // Based on the logic of not taking initial and last values. For mex seat, it was till row
        // 119. Same for col
        if id <= max_seat_id && id >= &80 {
            return id.clone();
        }
    }

    0
}
