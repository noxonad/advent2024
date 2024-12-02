use std::fs;

const ADMISSIBLE_INCREMENT: u8 = 3;

fn is_line_safe(line: &str) -> bool {
    let mut nums: Vec<u8> = Vec::new();

    // Parse the numbers
    line.split_whitespace()
        .for_each(|n| nums.push(n.parse().unwrap()));

    // Decreasing
    let inc: Vec<bool> = nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(a, b)| b < a && a <= &(b + ADMISSIBLE_INCREMENT))
        .collect();

    // If it's decreasing, return and don't calculate the increasing
    if inc.iter().all(|&x| x) {
        return true;
    }

    // Increasing
    let dec: Vec<bool> = nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(a, b)| a < b && &(a + ADMISSIBLE_INCREMENT) >= b)
        .collect();

    dec.iter().all(|&x| x)
}

/// Fortunately, the first location The Historians want to search
/// isn't a long walk from the Chief Historian's office.
///
/// While the Red-Nosed Reindeer nuclear fusion/fission plant appears
/// to contain no sign of the Chief Historian, the engineers there run
/// up to you as soon as they see you. Apparently, they still talk about
/// the time Rudolph was saved through molecular synthesis from a single electron.
///
/// They're quick to add that - since you're already here - they'd really appreciate
/// your help analyzing some unusual data from the Red-Nosed reactor.
/// You turn to check if The Historians are waiting for you,
/// but they seem to have already divided into groups that are currently searching
/// every corner of the facility. You offer to help with the unusual data.
///
/// The unusual data (your puzzle input) consists of many reports, one report per line.
/// Each report is a list of numbers called levels that are separated by spaces. For example:
/// ````
/// 7 6 4 2 1
/// 1 2 7 8 9
/// 9 7 6 2 1
/// 1 3 2 4 5
/// 8 6 4 4 1
/// 1 3 6 7 9
/// ````
/// This example data contains six reports each containing five levels.
///
/// The engineers are trying to figure out which reports are safe.
/// The Red-Nosed reactor safety systems can only tolerate levels that are either
/// gradually increasing or gradually decreasing. So, a report only counts
/// as safe if both of the following are true:
///
/// The levels are either all increasing or all decreasing.
/// Any two adjacent levels differ by at least one and at most three.
/// In the example above, the reports can be found safe or unsafe by checking those rules:
///
/// `7 6 4 2 1`: Safe because the levels are all decreasing by `1` or `2`.
/// `1 2 7 8 9`: Unsafe because `2 7` is an increase of `5`.
/// `9 7 6 2 1`: Unsafe because `6 2` is a decrease of `4`.
/// `1 3 2 4 5`: Unsafe because `1 3` is increasing but `3 2` is decreasing.
/// `8 6 4 4 1`: Unsafe because `4 4` is neither an increase or a decrease.
/// `1 3 6 7 9`: Safe because the levels are all increasing by `1`, `2`, or `3`.
/// So, in this example, `2` reports are safe.
///
/// Analyze the unusual data from the engineers. How many reports are safe?
pub fn count_safe_levels(input_levels: String) -> u32 {
    let mut counter: u32 = 0;
    let input_lines = input_levels.lines();
    for line in input_lines {
        if is_line_safe(line) {
            counter += 1;
        }
    }
    counter
}

fn is_line_safe_with_dampener(line: &str) -> bool {
    let mut nums: Vec<u8> = Vec::new();

    // Parse the numbers
    line.split_whitespace()
        .for_each(|n| nums.push(n.parse().unwrap()));

    // Decreasing
    for (i, _) in nums.iter().enumerate() {
        let nums_cut: Vec<u8> = nums
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, value)| *value)
            .collect();
        let inc: Vec<bool> = nums_cut
            .iter()
            .zip(nums_cut.iter().skip(1))
            .map(|(a, b)| b < a && a <= &(b + ADMISSIBLE_INCREMENT))
            .collect();

        if inc.iter().all(|&x| x) {
            return true;
        }
    }

    // Increasing
    for (i, _) in nums.iter().enumerate() {
        let nums_cut: Vec<u8> = nums
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, value)| *value)
            .collect();
        let dec: Vec<bool> = nums_cut
            .iter()
            .zip(nums_cut.iter().skip(1))
            .map(|(a, b)| a < b && &(a + ADMISSIBLE_INCREMENT) >= b)
            .collect();

        if dec.iter().all(|&x| x) {
            return true;
        }
    }

    false
}

/// The engineers are surprised by the low number of safe reports
/// until they realize they forgot to tell you about the Problem Dampener.
///
/// The Problem Dampener is a reactor-mounted module that lets the
/// reactor safety systems tolerate a single bad level in what would
/// otherwise be a safe report. It's like the bad level never happened!
///
/// Now, the same rules apply as before, except if removing a single level
/// from an unsafe report would make it safe, the report instead counts as safe.
///
/// More of the above example's reports are now safe:
///
/// `7 6 4 2 1`: Safe without removing any level.
/// `1 2 7 8 9`: Unsafe regardless of which level is removed.
/// `9 7 6 2 1`: Unsafe regardless of which level is removed.
/// `1 3 2 4 5`: Safe by removing the second level, `3`.
/// `8 6 4 4 1`: Safe by removing the third level, `4`.
/// `1 3 6 7 9`: Safe without removing any level.
/// Thanks to the Problem Dampener, `4` reports are actually safe!
///
/// Update your analysis by handling situations where the
/// Problem Dampener can remove a single level from unsafe reports.
/// How many reports are now safe?
pub fn count_safe_levels_with_dampener(input_levels: String) -> u32 {
    let mut counter: u32 = 0;
    let input_lines = input_levels.lines();
    for line in input_lines {
        if is_line_safe_with_dampener(line) {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let input: String =
        fs::read_to_string("src/input.txt").expect("Could not read the puzzle input");
    println!("safe: {}", count_safe_levels(input.clone()));
    println!(
        "safe with dampener: {}",
        count_safe_levels_with_dampener(input.clone())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_part_one() {
        let input: String =
            fs::read_to_string("src/test_input.txt").expect("Could not read the puzzle input");
        let e = 2;
        assert_eq!(e, count_safe_levels(input))
    }

    #[test]
    fn given_test_part_two() {
        let input: String =
            fs::read_to_string("src/test_input.txt").expect("Could not read the puzzle input");
        let e = 4;
        assert_eq!(e, count_safe_levels_with_dampener(input))
    }
}
