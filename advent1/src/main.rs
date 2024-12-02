use std::collections::{BinaryHeap, HashMap};
use std::fs;

/// Upon pouring into the office, everyone confirms that the Chief Historian
/// is indeed nowhere to be found. Instead, the Elves discover an assortment
/// of notes and lists of historically significant locations! This seems to be
/// the planning the Chief Historian was doing before he left. Perhaps these notes
/// can be used to determine which locations to search?
///
/// Throughout the Chief's office, the historically significant locations are listed
/// not by name but by a unique number called the location ID. To make sure they don't
/// miss anything, The Historians split into two groups, each searching the office
/// and trying to create their own complete list of location IDs.
///
/// There's just one problem: by holding the two lists up side by side (your puzzle input),
/// it quickly becomes clear that the lists aren't very similar.
/// Maybe you can help The Historians reconcile their lists?
///
/// Maybe the lists are only off by a small amount! To find out,
/// pair up the numbers and measure how far apart they are. Pair up the
/// smallest number in the left list with the smallest number in the right list,
/// then the second-smallest left number with the second-smallest right number, and so on.
///
/// Within each pair, figure out how far apart the two numbers are;
/// you'll need to add up all of those distances. For example,
/// if you pair up a `3` from the left list with a `7` from the right list,
/// the distance apart is `4`; if you pair up a `9` with a `3`, the distance apart is `6`.
///
/// In the example list above, the pairs and distances would be as follows:
/// - The smallest number in the left list is `1`,
///   and the smallest number in the right list is `3`. The distance between them is `2`.
/// - The second-smallest number in the left list is `2`, and the second-smallest number
///   in the right list is another `3`. The distance between them is `1`.
/// - The third-smallest number in both lists is `3`, so the distance between them is `0`.
/// - The next numbers to pair up are `3` and `4`, a distance of `1`.
/// - The fifth-smallest numbers in each list are `3` and `5`, a distance of `2`.
/// - Finally, the largest number in the left list is `4`, while the largest number
///   in the right list is `9`; these are a distance `5` apart.
///
/// To find the total distance between the left list and the right list,
/// add up the distances between all of the pairs you found.
/// In the example above, this is `2 + 1 + 0 + 1 + 2 + 5`, a total distance of `11`!
pub fn get_distance(input_string: String) -> u32 {
    // Split into lines
    let input_lines = input_string.lines();

    // Creates binary heaps for left and right columns
    let mut lnums: BinaryHeap<u32> = BinaryHeap::new();
    let mut rnums: BinaryHeap<u32> = BinaryHeap::new();

    // Inserts into both heaps, complexity is O(n*log(n)) where n is the line number
    for line in input_lines {
        let mut s = line.split_whitespace();

        lnums.push(s.next().unwrap().parse().unwrap());
        rnums.push(s.next().unwrap().parse().unwrap());
    }

    // Collect heap into a vector, complexity is O(n)
    let lnums = lnums.into_sorted_vec();
    let rnums = rnums.into_sorted_vec();

    // Calculate the distance, complexity is O(n)
    lnums.iter().zip(&rnums).map(|(l, r)| l.abs_diff(*r)).sum()
}

/// Your analysis only confirmed what everyone feared: the two lists of location IDs are indeed very different.
///
/// Or are they?
///
/// The Historians can't agree on which group made the mistakes or how to read
/// most of the Chief's handwriting, but in the commotion you notice an interesting detail:
/// a lot of location IDs appear in both lists! Maybe the other numbers aren't
/// location IDs at all but rather misinterpreted handwriting.
///
/// This time, you'll need to figure out exactly how often each number from the left
/// list appears in the right list. Calculate a total similarity score by adding up
/// each number in the left list after multiplying it by the number of times that
/// number appears in the right list.
///
/// For these example lists, here is the process of finding the similarity score:
/// - The first number in the left list is `3`. It appears in the right list three times,
///   so the similarity score increases by `3 * 3 = 9`.
/// - The second number in the left list is `4`. It appears in the right list once,
///   so the similarity score increases by `4 * 1 = 4`.
/// - The third number in the left list is `2`. It does not appear in the right list,
///   so the similarity score does not increase `(2 * 0 = 0)`.
/// - The fourth number, `1`, also does not appear in the right list.
/// - The fifth number, `3`, appears in the right list three times; the similarity score increases by `9`.
/// - The last number, `3`, appears in the right list three times; the similarity score again increases by `9`.
///
/// So, for these example lists, the similarity score at the end of this process is `31` (`9 + 4 + 0 + 0 + 9 + 9`).
///
/// Once again consider your left and right lists. What is their similarity score?
pub fn get_similarity(input_string: String) -> u32 {
    let input_lines = input_string.lines();

    let mut lmap: HashMap<u32, u8> = HashMap::new();
    let mut rmap: HashMap<u32, u8> = HashMap::new();

    // Map the numbers to their count in the input
    for line in input_lines {
        let mut s = line.split_whitespace();

        let lnum: u32 = s.next().unwrap().parse().unwrap();
        let rnum: u32 = s.next().unwrap().parse().unwrap();

        lmap.entry(lnum).and_modify(|e| *e += 1).or_insert(1);
        rmap.entry(rnum).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut similarity: u32 = 0;
    for (k, v) in lmap.iter() {
        similarity += k * (*v as u32) * (*rmap.get(k).unwrap_or(&0) as u32);
    }

    similarity
}

fn main() {
    // Read from input file
    let input = fs::read_to_string("src/input.txt").expect("Could not read the puzzle input");
    println!("distance: {}", get_distance(input.clone()));
    println!("similarity: {}", get_similarity(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_part_one() {
        let input =
            fs::read_to_string("src/test_input.txt").expect("Could not read the puzzle input");
        let e = 11;
        assert_eq!(e, get_distance(input))
    }

    #[test]
    fn given_test_part_two() {
        let input: String =
            fs::read_to_string("src/test_input.txt").expect("Could not read the puzzle input");
        let e: u32 = 31;
        assert_eq!(e, get_similarity(input))
    }
}
