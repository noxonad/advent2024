use regex::Regex;
use std::fs;

/// "Our computers are having issues,
/// so I have no idea if we have any Chief Historians
/// in stock! You're welcome to check the warehouse,
/// though," says the mildly flustered shopkeeper at the
/// North Pole Toboggan Rental Shop. The Historians head out to take a look.
///
/// The shopkeeper turns to you.
/// "Any chance you can see why our computers are having issues again?"
///
/// The computer appears to be trying to run a program,
/// but its memory (your puzzle input) is corrupted.
/// All of the instructions have been jumbled up!
///
/// It seems like the goal of the program is just to multiply some numbers.
/// It does that with instructions like `mul(X,Y)`, where `X` and `Y`
/// are each 1-3 digit numbers. For instance, `mul(44,46)`
/// multiplies `44` by `46` to get a result of `2024`.
/// Similarly, `mul(123,4)` would multiply `123` by `4`.
///
/// However, because the program's memory has been corrupted,
/// there are also many invalid characters that should be ignored,
/// even if they look like part of a mul instruction.
/// Sequences like `mul(4*`, `mul(6,9!`, `?(12,34)`, or `mul ( 2 , 4 )` do nothing.
///
/// For example, consider the following section of corrupted memory:
///
/// `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`
/// Only the four highlighted sections are real mul instructions.
/// Adding up the result of each instruction produces `161` `(2*4 + 5*5 + 11*8 + 8*5)`.
///
/// Scan the corrupted memory for uncorrupted mul instructions.
/// What do you get if you add up all of the results of the multiplications?
pub fn do_sum_mul(input: String) -> u32 {
    // Create the regex rule of mul
    // mul(123,123)
    let r = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();

    // Initialize the vector storing numbers
    let mut muls: Vec<(u32, u32)> = Vec::new();

    // Apply the regex and store the numbers in the vector
    r.captures_iter(input.as_str())
        .map(|e| e.extract())
        .for_each(|(_, [n1, n2])| muls.push((n1.parse().unwrap(), n2.parse().unwrap())));

    // Return the sum of multiplications
    muls.iter()
        .map(|(n1, n2)| n1 * n2)
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
}

/// As you scan through the corrupted memory, you notice that
/// some of the conditional statements are also still intact.
/// If you handle some of the uncorrupted conditional statements
/// in the program, you might be able to get an even more accurate result.
///
/// There are two new instructions you'll need to handle:
///
/// The do() instruction enables future mul instructions.
/// The don't() instruction disables future mul instructions.
/// Only the most recent do() or don't() instruction applies.
/// At the beginning of the program, mul instructions are enabled.
///
/// For example:
///
/// `xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`
/// This corrupted memory is similar to the example from before, but this time
/// the `mul(5,5)` and `mul(11,8)` instructions are disabled because there is
/// a `don't()` instruction before them. The other mul instructions function normally,
/// including the one at the end that gets re-enabled by a `do()` instruction.
///
/// This time, the sum of the results is `48` `(2*4 + 8*5)`.
///
/// Handle the new instructions; what do you get if you add up all
/// of the results of just the enabled multiplications?
pub fn do_sum_mul_with_do(input: String) -> u32 {
    // Split by `don't()`
    let split_input_dont: Vec<&str> = input.split("don't()").collect();

    // Initialize a vector for `do()` split
    let mut split_input_do: Vec<&str> = Vec::new();

    // Add the first element until `don't()` to the vector
    split_input_do.push(split_input_dont.first().unwrap());

    // Add the elements after `do()` and before the next `don't()` to the vector
    for s in split_input_dont.iter().skip(1) {
        s.split("do()").skip(1).for_each(|e| split_input_do.push(e));
    }

    // Sum of multiply of the substring of input
    do_sum_mul(split_input_do.concat())
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Could not read the puzzle input");
    println!("Sum: {}", do_sum_mul(input.clone()));
    println!("Sum with do: {}", do_sum_mul_with_do(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_part_one() {
        let input: String =
            fs::read_to_string("test_input.txt").expect("Could not read the puzzle input");
        let e: u32 = 161;
        assert_eq!(e, do_sum_mul(input))
    }

    #[test]
    fn given_test_part_two() {
        let input: String =
            fs::read_to_string("test_input.txt").expect("Could not read the puzzle input");
        let e: u32 = 48;
        assert_eq!(e, do_sum_mul_with_do(input))
    }
}
