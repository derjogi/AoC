use std::collections::HashSet;
use itertools::Itertools;
use quote::__private::push_pound;

#[aoc::main(3)]
fn main(input: &str) {
    println!("Items in both rucksack-compartments: {}", calculate_rucksack_items(input));
    println!("Sum of badges: {}", calculate_badge(input));
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn calculate_rucksack_items(data: &str) -> u32 {
    let result = data.lines()
        .map(|l| {
            let mut chars_in_both_compartments = HashSet::new();
            let half = l.len() / 2;
            let l1 = &l[..half];
            let l2 = &l[half..];
            for x in l1.chars() {
                if l2.contains(x) {
                    if x.is_lowercase() {
                        let position = 1 + ALPHABET.find(x).unwrap() as u8;
                        chars_in_both_compartments.insert(position);
                    } else {
                        let lowercase = x.to_lowercase().exactly_one().unwrap();
                        let position = (1 + ALPHABET.find(lowercase).unwrap() + ALPHABET.len()) as u8;
                        chars_in_both_compartments.insert(position);
                    }
                }
            }
            let in_rucksack = chars_in_both_compartments.iter().map(|&i| i as u32).sum::<u32>();
            println!("{in_rucksack}");
            in_rucksack
        })
        .sum();
    result
}

fn calculate_badge(data: &str) -> u32 {
    let lines: Vec<&str> = data.lines().collect();
    let groups = lines.chunks(3);
    groups.map(|group| {
        let mut chars_in_all_rucksacks = HashSet::new();
        for x in group[0].chars() {
            if group[1].contains(x) && group[2].contains(x) {
                if x.is_lowercase() {
                    let position = 1 + ALPHABET.find(x).unwrap() as u8;
                    chars_in_all_rucksacks.insert(position);
                } else {
                    let lowercase = x.to_lowercase().exactly_one().unwrap();
                    let position = (1 + ALPHABET.find(lowercase).unwrap() + ALPHABET.len()) as u8;
                    chars_in_all_rucksacks.insert(position);
                }
            }
        }
        let badge_in_group = chars_in_all_rucksacks.iter().map(|&i| i as u32).sum::<u32>();
        println!("{}", badge_in_group);
        badge_in_group
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;   // Making functions from outer module accessible

    const TEST_INPUT: &str = include_str!("../../inputs/day3/test.in");

    #[test]
    fn solution_part_1() {
        assert_eq!(157, calculate_rucksack_items(TEST_INPUT))
    }

    #[test]
    fn solution_part_2() {
        assert_eq!(70, calculate_badge(TEST_INPUT))
    }
}
