use std::fs;
use std::thread;
use std::time::Duration;

const IS_VISUAL_ON: bool = false;

fn make_step(map: &mut [Vec<char>]) -> bool {
    let mut i_change: Option<usize> = None;
    let mut j_change: Option<usize> = None;
    let mut change_char: Option<char> = None;

    for (i, line) in map.iter_mut().enumerate() {
        for (j, c) in line.iter_mut().enumerate() {
            match c {
                'v' => {
                    i_change = i.checked_add(1);
                    j_change = Some(j);
                    change_char = Some('v');
                }
                '^' => {
                    i_change = i.checked_sub(1);
                    j_change = Some(j);
                    change_char = Some('^');
                }
                '<' => {
                    i_change = Some(i);
                    j_change = j.checked_sub(1);
                    change_char = Some('<');
                }
                '>' => {
                    i_change = Some(i);
                    j_change = j.checked_add(1);
                    change_char = Some('>');
                }
                _ => continue, // Should not get here
            }
            *c = 'X';
        }
    }

    if i_change.is_none() || j_change.is_none() || change_char.is_none() {
        return false;
    }

    if map
        .get(i_change.unwrap())
        .unwrap_or(&vec![])
        .get(j_change.unwrap())
        .unwrap_or(&' ')
        == &'#'
    {
        match change_char {
            Some('v') => {
                i_change = i_change.unwrap().checked_sub(1);
                j_change = j_change.unwrap().checked_sub(1);
                change_char = Some('<');
            }
            Some('^') => {
                i_change = i_change.unwrap().checked_add(1);
                j_change = j_change.unwrap().checked_add(1);
                change_char = Some('>');
            }
            Some('<') => {
                i_change = i_change.unwrap().checked_sub(1);
                j_change = j_change.unwrap().checked_add(1);
                change_char = Some('^');
            }
            Some('>') => {
                i_change = i_change.unwrap().checked_add(1);
                j_change = j_change.unwrap().checked_sub(1);
                change_char = Some('v');
            }
            _ => return false,
        }
    }

    if i_change.is_none() || j_change.is_none() || change_char.is_none() {
        return false;
    }

    if i_change.unwrap() < map.len() && j_change.unwrap() < map[i_change.unwrap()].len() {
        map[i_change.unwrap()][j_change.unwrap()] = change_char.unwrap();
    }

    true
}

/// The Historians use their fancy device again, this time to whisk you all away
/// to the North Pole prototype suit manufacturing lab... in the year 1518!
/// It turns out that having direct access to history is very convenient for a group of historians.
///
/// You still have to be careful of time paradoxes, and so it will be important to avoid anyone
/// from 1518 while The Historians search for the Chief. Unfortunately,
/// a single guard is patrolling this part of the lab.
///
/// Maybe you can work out where the guard will go ahead of time so that The Historians can search safely?
///
/// You start by making a map (your puzzle input) of the situation. For example:
///
///````
/// ....#.....
/// .........#
/// ..........
/// ..#.......
/// .......#..
/// ..........
/// .#..^.....
/// ........#.
/// #.........
/// ......#...
/// ````
///
/// The map shows the current position of the guard with `^` (to indicate the guard is currently facing up from the perspective of the map).
/// Any obstructions - crates, desks, alchemical reactors, etc. - are shown as `#`.
///
/// Lab guards in 1518 follow a very strict patrol protocol which involves repeatedly following these steps:
///
/// If there is something directly in front of you, turn right 90 degrees.
/// Otherwise, take a step forward.
/// Following the above protocol, the guard moves up several times until she reaches an obstacle (in this case, a pile of failed suit prototypes):
///
/// ````
/// ....#.....
/// ....^....#
/// ..........
/// ..#.......
/// .......#..
/// ..........
/// .#........
/// ........#.
/// #.........
/// ......#...
/// ````
///
/// Because there is now an obstacle in front of the guard, she turns right before continuing straight in her new facing direction:
///
/// ````
/// ....#.....
/// ........>#
/// ..........
/// ..#.......
/// .......#..
/// ..........
/// .#........
/// ........#.
/// #.........
/// ......#...
/// ````
///
/// Reaching another obstacle (a spool of several very long polymers), she turns right again and continues downward:
///
/// ````
/// ....#.....
/// .........#
/// ..........
/// ..#.......
/// .......#..
/// ..........
/// .#......v.
/// ........#.
/// #.........
/// ......#...
/// ````
///
/// This process continues for a while, but the guard eventually leaves the mapped area (after walking past a tank of universal solvent):
///
/// ````
/// ....#.....
/// .........#
/// ..........
/// ..#.......
/// .......#..
/// ..........
/// .#........
/// ........#.
/// #.........
/// ......#v..
/// ````
/// By predicting the guard's route, you can determine which specific positions in the lab will be in the patrol path.
/// Including the guard's starting position, the positions visited by the guard before leaving the area are marked with an `X`:
///
/// ````
/// ....#.....
/// ....XXXXX#
/// ....X...X.
/// ..#.X...X.
/// ..XXXXX#X.
/// ..X.X.X.X.
/// .#XXXXXXX.
/// .XXXXXXX#.
/// #XXXXXXX..
/// ......#X..
/// ````
/// In this example, the guard will visit 41 distinct positions on your map.
///
/// Predict the path of the guard. How many distinct positions will the guard visit before leaving the mapped area?
fn number_of_distinct_steps(input: &str) -> u32 {
    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    while make_step(&mut map) {
        if IS_VISUAL_ON {
            print!("\x1b[2J\x1b[H");
            map.iter().for_each(|line| {
                println!(
                    "{:?}",
                    line.iter()
                        .map(|&c| String::from(c) + " ")
                        .collect::<String>()
                )
            });
            thread::sleep(Duration::from_millis(50));
        }
    }

    (map.iter().flatten().filter(|&&c| c == 'X').count()) as u32
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Could not open the input file");
    println!(
        "Nunmber of distinct steps: {}",
        number_of_distinct_steps(&input),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_part_one() {
        let input: String =
            fs::read_to_string("test_input.txt").expect("Could not open the test input file");
        let e: u32 = 41;
        assert_eq!(e, number_of_distinct_steps(&input));
    }
}
