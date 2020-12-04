use crate::utils::file_utils::read_lines;
use regex::Regex;

pub fn day4() {
    let mut valid_passport_count = 0;
    let mut concatenated_lines = String::from("");

    if let Ok(lines) = read_lines("./assets/day4.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip != "" {
                    concatenated_lines.push_str(&ip);
                    concatenated_lines.push_str(" ");
                } else {
                    let result = is_valid_passport_2(&concatenated_lines.trim());

                    if result {
                        valid_passport_count += 1;
                    }

                    concatenated_lines = String::from("");
                }
            }
        }
    }

    println!("Number of valid passports: {}", valid_passport_count);
}

#[derive(Default)]
struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
}

impl Passport {
    fn new() -> Self {
        Default::default()
    }

    fn is_truthy(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}

fn is_valid_passport(passport_string: &str) -> bool {
    let all_fields: Vec<&str> = passport_string.split(" ").collect();
    let mut passport_fields = Passport::new();

    for field in all_fields {
        let key_value: Vec<&str> = field.split(":").collect();

        match key_value[0] {
            "byr" => passport_fields.byr = true,
            "iyr" => passport_fields.iyr = true,
            "eyr" => passport_fields.eyr = true,
            "hgt" => passport_fields.hgt = true,
            "hcl" => passport_fields.hcl = true,
            "ecl" => passport_fields.ecl = true,
            "pid" => passport_fields.pid = true,
            &_ => print!(""),
        }
    }

    if passport_fields.is_truthy() {
        true
    } else {
        false
    }
}

fn is_valid_passport_2(passport_string: &str) -> bool {
    let all_fields: Vec<&str> = passport_string.split(" ").collect();
    let mut passport_fields = Passport::new();

    for field in all_fields {
        let key_value: Vec<&str> = field.split(":").collect();

        match key_value[0] {
            "byr" => {
                if is_int_range(key_value[1], 1920, 2002) {
                    passport_fields.byr = true
                }
            }
            "iyr" => {
                if is_int_range(key_value[1], 2010, 2020) {
                    passport_fields.iyr = true
                }
            }
            "eyr" => {
                if is_int_range(key_value[1], 2020, 2030) {
                    passport_fields.eyr = true
                }
            }
            "hgt" => {
                if is_valid_height(key_value[1]) {
                    passport_fields.hgt = true
                }
            }
            "hcl" => {
                if is_valid_hair_color(key_value[1]) {
                    passport_fields.hcl = true
                }
            }
            "ecl" => {
                if is_valid_eye_color(key_value[1]) {
                    passport_fields.ecl = true
                }
            }
            "pid" => {
                if is_valid_passport_number(key_value[1]) {
                    passport_fields.pid = true
                }
            }
            &_ => print!(""),
        }
    }

    if passport_fields.is_truthy() {
        true
    } else {
        false
    }
}

fn is_int_range(string: &str, min: u32, max: u32) -> bool {
    let str_as_int: u32 = string.parse().unwrap();

    min <= str_as_int && str_as_int <= max
}

fn is_valid_height(string: &str) -> bool {
    let (height, unit) = string.split_at(string.len() - 2);

    if unit == "cm" {
        is_int_range(height, 150, 193)
    } else if unit == "in" {
        is_int_range(height, 59, 76)
    } else {
        false
    }
}

fn is_valid_hair_color(string: &str) -> bool {
    let (hash, color) = string.split_at(1);

    if hash != "#" {
        return false;
    }

    if color.len() != 6 {
        return false;
    }

    Regex::new(r"[0-9abcdef]").unwrap().is_match(color)
}

fn is_valid_eye_color(string: &str) -> bool {
    match string {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return true,
        &_ => return false,
    }
}

fn is_valid_passport_number(string: &str) -> bool {
    if string.len() != 9 {
        return false;
    }

    let pass_num = string.parse::<u32>();

    if pass_num.is_err() {
        false
    } else {
        true
    }
}
