use std::cmp::min;
use itertools::Itertools;
use syn::__private::bool;

#[aoc::main(4)]
fn main(input: &str) {
    println!("Part1 output: {}", get_number_completely_contains(input));
    println!("Part2 output: {}", get_number_overlapping(input));
}

fn get_number_completely_contains(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let array_of_pairs = get_line_as_array_of_pairs(line);
            completely_contains(array_of_pairs)
        }).filter(|b| *b)
        .count()
}

fn get_number_overlapping(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let array_of_pairs = get_line_as_array_of_pairs(line);
            overlaps(array_of_pairs)
        }).filter(|b| *b)
        .count()
}

fn completely_contains(pair:Vec<[u8;2]>) -> bool {
    let one_contains_the_other = pair[0].surrounds(&pair[1])
        || pair[1].surrounds(&pair[0]);
    if one_contains_the_other {
        println!("Pair {:?} contains each other.", pair)
    }
    one_contains_the_other
}

fn overlaps(pair:Vec<[u8;2]>) -> bool {
    let overlapping = pair[0].overlaps(&pair[1]);
    if overlapping {
        println!("Pair {:?} overlap each other.", pair)
    }
    overlapping
}

fn get_line_as_array_of_pairs(line: &str) -> Vec<[u8; 2]> {
    line.split(",")
        .map(|range| {
            let min_max = range.split_once("-").unwrap();
            let min_max_array = [min_max.0.parse::<u8>().unwrap(), min_max.1.parse::<u8>().unwrap()];
            min_max_array
        })
        .collect::<Vec<[u8; 2]>>()
}


trait ContainableSortedRange {
    fn surrounds(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl ContainableSortedRange for [u8;2] {
    fn surrounds(&self, other:&Self) -> bool {
        // Return true if one range is completely inside another
        self[0] <= other[0] && self[1] >= other[1]
    }

    fn overlaps(&self, other: &Self) -> bool {
        // Return true if one range is partly (or completely inside another
        self.surrounds(other)
            || (self[0] >= other[0] && self[0] <= other[1])
            || (self[1] >= other[0] && self[1] <= other[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;   // Making functions from outer module accessible

    const TEST_INPUT: &str = include_str!("../../inputs/day4/test.in");

    #[test]
    fn solution_part_1() {
        let i = get_number_completely_contains(TEST_INPUT);
        println!("Found {i} pairs where one fully contains the other!");
        assert_eq!(2, i)
    }

    #[test]
    fn solution_part_2() {
        let i = get_number_overlapping(TEST_INPUT);
        println!("Found {i} pairs where one overlaps the other!");
        assert_eq!(4, i)
    }
}
