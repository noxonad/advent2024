use std::fs;

use std::collections::{BTreeSet, HashMap};

fn get_rules(rules: Vec<&str>) -> HashMap<u8, BTreeSet<u8>> {
    let mut res: HashMap<u8, BTreeSet<u8>> = HashMap::new();

    for rule in rules {
        let mut srule = rule.split("|");
        let before: u8 = srule.next().unwrap().parse().unwrap();
        let after: u8 = srule.next().unwrap().parse().unwrap();

        res.entry(before).or_default();
        res.get_mut(&before).unwrap().insert(after);
    }

    res
}

fn get_changes_breaking_rules_by_index(
    changes: &[u8],
    rules: &HashMap<u8, BTreeSet<u8>>,
) -> Option<(usize, usize)> {
    for (i, v) in changes.iter().enumerate() {
        for (j, u) in changes.iter().skip(i + 1).enumerate() {
            if rules.get(u).unwrap_or(&BTreeSet::new()).contains(v) {
                return Some((i, j + i + 1));
            }
        }
    }
    None
}

/// Satisfied with their search on Ceres,
/// the squadron of scholars suggests subsequently
/// scanning the stationery stacks of sub-basement 17.
///
/// The North Pole printing department is busier than ever
/// this close to Christmas, and while The Historians continue
/// their search of this historically significant facility,
/// an Elf operating a very familiar printer beckons you over.
///
/// The Elf must recognize you, because they waste no time explaining
/// that the new sleigh launch safety manual updates won't print correctly.
/// Failure to update the safety manuals would be dire indeed, so you offer your services.
///
/// Safety protocols clearly indicate that new pages for the safety manuals
/// must be printed in a very specific order. The notation X|Y
/// means that if both page number X and page number Y are to be produced
/// as part of an update, page number X must be printed at some point before page number Y.
///
/// The Elf has for you both the page ordering rules and the pages
/// to produce in each update (your puzzle input), but can't figure out
/// whether each update has the pages in the right order.
///
/// For example:
///
/// ````
/// 47|53
/// 97|13
/// 97|61
/// 97|47
/// 75|29
/// 61|13
/// 75|53
/// 29|13
/// 97|29
/// 53|29
/// 61|53
/// 97|53
/// 61|29
/// 47|13
/// 75|47
/// 97|75
/// 47|61
/// 75|61
/// 47|29
/// 75|13
/// 53|13
///
/// 75,47,61,53,29
/// 97,61,53,29,13
/// 75,29,13
/// 75,97,47,61,53
/// 61,13,29
/// 97,13,75,29,47
/// ````
///
/// The first section specifies the page ordering rules, one per line.
/// The first rule, `47|53`, means that if an update includes both page number `47`
/// and page number `53`, then page number `47` must be printed at some point
/// before page number `53`. (`47` doesn't necessarily need to be immediately before `53`;
/// other pages are allowed to be between them.)
///
/// The second section specifies the page numbers of each update.
/// Because most safety manuals are different, the pages needed in the updates
/// are different too. The first update, `75,47,61,53,29`, means that
/// the update consists of page numbers `75`, `47`, `61`, `53`, and `29`.
///
/// To get the printers going as soon as possible, start by identifying
/// which updates are already in the right order.
///
/// In the above example, the first update (`75,47,61,53,29`) is in the right order:
///
/// - `75` is correctly first because there are rules that put
///    each other page after it: `75|47`, `75|61`, `75|53`, and `75|29`.
/// - `47` is correctly second because `75` must be before it
///    (`75|47`) and every other page must be after it according to `47|61`, `47|53`, and `47|29`.
/// - `61` is correctly in the middle because `75` and `47` are before it
///    (`75|61` and `47|61`) and `53` and `29` are after it (`61|53` and `61|29`).
/// - `53` is correctly fourth because it is before page number `29` (`53|29`).
/// - `29` is the only page left and so is correctly last.
///
/// Because the first update does not include some page numbers,
/// the ordering rules involving those missing page numbers are ignored.
///
/// The second and third updates are also in the correct order according to the rules.
/// Like the first update, they also do not include every page number,
/// and so only some of the ordering rules apply - within each update,
/// the ordering rules that involve missing page numbers are not used.
///
/// The fourth update, `75,97,47,61,53`, is not in the correct order:
/// it would print `75` before `97`, which violates the rule `97|75`.
///
/// The fifth update, `61,13,29`, is also not in the correct order, since it breaks the rule 29|13.
///
/// The last update, `97,13,75,29,47`, is not in the correct order due to breaking several rules.
///
/// For some reason, the Elves also need to know the middle page number
/// of each update being printed. Because you are currently only printing
/// the correctly-ordered updates, you will need to find the middle page number
/// of each correctly-ordered update. In the above example, the correctly-ordered updates are:
///
/// ````
/// 75,47,61,53,29
/// 97,61,53,29,13
/// 75,29,13
/// ````
///
/// These have middle page numbers of 61, 53, and 29 respectively.
/// Adding these page numbers together gives 143.
///
/// Of course, you'll need to be careful: the actual list of page ordering rules
/// is bigger and more complicated than the above example.
///
/// Determine which updates are already in the correct order.
/// What do you get if you add up the middle page number from those correctly-ordered updates?
fn get_sum_middle_pages(input: &str) -> u32 {
    let rules_string: Vec<&str> = input
        .lines()
        .take_while(|line| !line.is_empty()) // Take lines until the first empty line
        .collect();
    let changes_string: Vec<&str> = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .collect();

    let rules = get_rules(rules_string);
    let changes: Vec<Vec<u8>> = changes_string
        .iter()
        .map(|line| {
            line.split(",")
                .map(|el| el.parse().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();

    changes
        .iter()
        .map(|el| {
            if get_changes_breaking_rules_by_index(el, &rules).is_none() {
                el[el.len() / 2]
            } else {
                0
            }
        })
        .collect::<Vec<u8>>()
        .iter()
        .map(|&el| el as u32)
        .sum::<u32>()
}

fn correct_using_rules(changes: &[u8], rules: &HashMap<u8, BTreeSet<u8>>) -> Vec<u8> {
    let mut res = Vec::from(changes);

    loop {
        let c: Option<(usize, usize)> = get_changes_breaking_rules_by_index(&res, rules);
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
        res.swap(c.0, c.1);
    }

    res
}

/// While the Elves get to work printing the correctly-ordered updates,
/// you have a little time to fix the rest of them.
///
/// For each of the incorrectly-ordered updates, use the page ordering rules
/// to put the page numbers in the right order. For the above example, here are
/// the three incorrectly-ordered updates and their correct orderings:
///
/// - `75,97,47,61,53` becomes `97,75,47,61,53`.
/// - `61,13,29` becomes `61,29,13`.
/// - `97,13,75,29,47` becomes `97,75,47,29,13`.
///
/// After taking only the incorrectly-ordered updates and ordering them correctly,
/// their middle page numbers are 47, 29, and 47. Adding these together produces 123.
///
/// Find the updates which are not in the correct order. What do you get if you
/// add up the middle page numbers after correctly ordering just those updates?
fn get_sum_middle_pages_incorrect_but_ordered(input: &str) -> u32 {
    let rules_string: Vec<&str> = input
        .lines()
        .take_while(|line| !line.is_empty()) // Take lines until the first empty line
        .collect();
    let changes_string: Vec<&str> = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .collect();

    let rules = get_rules(rules_string);
    let changes: Vec<Vec<u8>> = changes_string
        .iter()
        .map(|line| {
            line.split(",")
                .map(|el| el.parse().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();

    let is_correct = changes
        .iter()
        .map(|el| get_changes_breaking_rules_by_index(el, &rules).is_none())
        .collect::<Vec<bool>>();

    changes
        .iter()
        .enumerate()
        .map(|(i, _)| {
            if !is_correct.get(i).unwrap() {
                correct_using_rules(&changes[i], &rules)[changes[i].len() / 2] as u32
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input: String =
        fs::read_to_string("input.txt").expect("Could not read from the input file");

    println!("Sum of correct: {}", get_sum_middle_pages(&input));
    println!(
        "Sum of incorrect: {}",
        get_sum_middle_pages_incorrect_but_ordered(&input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_part_one() {
        let input: String =
            fs::read_to_string("test_input.txt").expect("Could not read from the file");
        let e = 143;
        assert_eq!(e, get_sum_middle_pages(&input));
    }

    #[test]
    fn given_test_part_two() {
        let input: String =
            fs::read_to_string("test_input.txt").expect("Could not read from the file");
        let e = 123;
        assert_eq!(e, get_sum_middle_pages_incorrect_but_ordered(&input))
    }
}
