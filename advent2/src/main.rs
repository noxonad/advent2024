use std::{cmp::Ordering, fs};

const ADMISSIBLE_INCREMENT: u8 = 3;

fn is_line_safe(line: &str) -> bool {
    let mut is_increasing: Option<bool> = None;
    let mut current_num: Option<u8> = None;

    for num in line.split_whitespace() {
        // For the first number
        if current_num.is_none() {
            current_num = Some(num.parse().unwrap());
            continue;
        }

        // For the second number, to get whether it's increasing or decreasing
        let num: u8 = num.parse().unwrap();
        if is_increasing.is_none() {
            match current_num.unwrap().cmp(&num) {
                Ordering::Greater => is_increasing = Some(false),
                Ordering::Less => is_increasing = Some(true),
                Ordering::Equal => return false,
            }
        }

        if current_num.unwrap() == num {
            return false;
        }

        if current_num.unwrap().abs_diff(num) > ADMISSIBLE_INCREMENT {
            return false;
        }
        if is_increasing.unwrap() && current_num.unwrap() > num
            || !is_increasing.unwrap() && current_num.unwrap() < num
        {
            return false;
        }

        current_num = Some(num);
    }

    true
}

fn count_safe_levels(input_levels: String) -> u32 {
    let mut counter: u32 = 0;
    let input_lines = input_levels.lines();
    for line in input_lines {
        if is_line_safe(line) {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let input: String =
        fs::read_to_string("src/input.txt").expect("Could not read the puzzle input");
    println!("{}", count_safe_levels(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test() {
        let input: String =
            fs::read_to_string("src/test_input.txt").expect("Could not read the puzzle input");
        let e = 2;
        assert_eq!(e, count_safe_levels(input))
    }
}
