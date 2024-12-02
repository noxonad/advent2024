use std::collections::{BinaryHeap, HashMap};
use std::fs;

fn get_distance(input_string: String) -> u32 {
    // Split into lines
    let input_lines = input_string.lines();

    let mut lnums: BinaryHeap<u32> = BinaryHeap::new();
    let mut rnums: BinaryHeap<u32> = BinaryHeap::new();

    for line in input_lines {
        let mut s = line.split_whitespace();

        lnums.push(s.next().unwrap().parse().unwrap());
        rnums.push(s.next().unwrap().parse().unwrap());
    }

    let mut distance: u32 = 0;

    loop {
        let lnum: Option<u32> = lnums.pop();
        let rnum: Option<u32> = rnums.pop();

        if lnum.is_some() && rnum.is_some() {
            let lnum: u32 = lnum.unwrap();
            let rnum: u32 = rnum.unwrap();

            distance += lnum.abs_diff(rnum);
        } else {
            break;
        }
    }

    distance
}

fn get_similarity(input_string: String) -> u32 {
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
