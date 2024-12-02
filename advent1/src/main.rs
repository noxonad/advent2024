use std::collections::BinaryHeap;
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

fn main() {
    // Read from input file
    let input = fs::read_to_string("src/input.txt").expect("Could not read the puzzle input");
    println!("distance: {}", get_distance(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test() {
        let input =
            fs::read_to_string("src/test_input.txt").expect("Could not read the puzzle input");
        let e = 11;
        assert_eq!(e, get_distance(input))
    }
}
