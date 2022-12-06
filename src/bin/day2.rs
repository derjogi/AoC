use std::collections::HashMap;
use std::iter::Map;
use std::ptr::eq;
use itertools::Itertools;

#[aoc::main(2)]
fn main(input: &str) {
    println!("Rock, Paper, Scissors");
    println!("Playing Rock, Paper, Scissors according to our interpretation would result in a score of {}", calculate_score(input));
    println!("Playing Rock, Paper, Scissors according to the elf's plan would result in a score of {}", calculate_hand_and_score(input));
}

// Convert to 1, 2 & 3
// Lost = 0
// Draw = 3
// Win  = 6
// Score = Shape + Result (1/2/3 + 0/3/6)

//      You  Elf
// Lost = a - b == -1 or 2 (Scissors - Rock)
// Draw = a - b == 0
// Win  = a - b == 1 or -2 (Rock - Scissors)

pub fn calculate_score(input: &str) -> i32 {

    let mut hand_map = HashMap::new();
    hand_map.insert("A", 1);
    hand_map.insert("X", 1);
    hand_map.insert("B", 2);
    hand_map.insert("Y", 2);
    hand_map.insert("C", 3);
    hand_map.insert("Z", 3);

    let mut sum = 0;
    for line in input.lines() {
        let (elf, you) = line.split_once(" ").unwrap();
        let my_hand = hand_map.get(you).unwrap();
        let diff = my_hand - hand_map.get(elf).unwrap();
        let score = match diff {
            -1 | 2 => 0,
            0 => 3,
            1 | -2 => 6,
            _ => -1
        };
        if score >= 0 {
            println!("Testing");
            println!("Sum: {sum}, Hand: {my_hand}, Score: {score}");
            sum += score + my_hand;
        } else {
            println!("Oh, there's something wrong! Didn't expect {}", score );
            return -1
        }
    }

    sum
}


// Part 2:
// Convert to 1, 2 & 3
// Lost = 0
// Draw = 3
// Win  = 6
// Score = Shape + Result (1/2/3 + 0/3/6)
// Rock/1 loses to Paper/2 loses to Scissors/3 loses to Rock/1
// --> Win: need a Hand +1 or 1 (if Scissors/3)
// --> Lose: need a Hand -1 or 3 if Rock/1
// --> Draw: same Hand

pub fn calculate_hand_and_score(input: &str) -> i32 {

    let mut hand_map = HashMap::new();
    hand_map.insert("A", 1);
    hand_map.insert("B", 2);
    hand_map.insert("C", 3);

    hand_map.insert("X", 0);
    hand_map.insert("Y", 3);
    hand_map.insert("Z", 6);

    let mut sum = 0;
    for line in input.lines() {
        let (elf, condition) = line.split_once(" ").unwrap();
        let lose_draw_win = hand_map.get(condition).unwrap();
        let elf_hand = hand_map.get(elf).unwrap().to_owned();
        let my_hand = match lose_draw_win {
            0 => if elf_hand == 1 { 3 } else { elf_hand - 1 },
            3 => elf_hand.to_owned(),
            6 => if elf_hand == 3 { 1 } else { elf_hand + 1},
            _ => -1
        };


        if my_hand < 0 {
            println!("Oh, there's something wrong! Didn't expect {}", my_hand );
            return my_hand
        }
        sum += my_hand + lose_draw_win;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;   // Making functions from outer module accessible

    const TEST_INPUT: &str = include_str!("../../inputs/day2/test.in");

    #[test]
    fn solution_part_1() {
        assert_eq!(15, calculate_score(TEST_INPUT))
    }

    #[test]
    fn solution_part_2() {
        assert_eq!(12, calculate_hand_and_score(TEST_INPUT))
    }
}
