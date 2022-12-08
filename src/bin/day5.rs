use itertools::Itertools;

#[aoc::main(5)]
fn main(input: &str) {
    println!("Results Part 1: {}", get_tops_of_final_stack(input, false));
    println!("Results Part 2: {}", get_tops_of_final_stack(input, true));
}

pub fn get_tops_of_final_stack(data: &str, move_all_at_once: bool) -> String {
    let (num_stacks, stack_setup, moves) = setup(data);
    // Get stacks with the top element at the last position
    let stacks: Vec<Vec<char>> = parse_stacks(stack_setup, num_stacks);
    let modified_stacks = modify_stacks(stacks, moves, move_all_at_once);
    get_top_of_stacks(modified_stacks)
}

fn setup(data: &str) -> (usize, Vec<&str>, Vec<&str>) {
    let mut found_stack_num_line = false;
    let mut num_stacks: usize = 0;
    let mut stack_setup = vec![];
    let mut moves = vec![];
    for line in data.lines() {
        if line.is_empty() { continue }
        if line.trim_start().starts_with("1") {
            found_stack_num_line = true;
            num_stacks = line.split_whitespace()
                .next_back()
                .unwrap()
                .parse()
                .unwrap();
            continue;
        }
        if !found_stack_num_line {
            stack_setup.push(line)
        } else {
            moves.push(line);
        }
    }
    (num_stacks, stack_setup, moves)
}

fn get_top_of_stacks(stacks: Vec<Vec<char>>) -> String {
    let mut result: String = String::from("");
    for stack in stacks {
        result.push(stack.get(stack.len()-1).unwrap().to_owned())
    }
    result
}

fn parse_stacks(stack_lines: Vec<&str>, number_of_stacks: usize) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(number_of_stacks);

    for x in 0..number_of_stacks {
        println!("Creating stack {}", x);
        stacks.push(vec![])
    }

    // Iterate over each line and parse the individual crates at each position.
    // Can't just split at whitespace, because there might be multiple empty ones, so we need to
    // actually do it by it's position in the line.
    let reversed = stack_lines.into_iter().rev().collect::<Vec<&str>>();
    for line in reversed {
        let mut iteration = 0;
        let mut char_position = 1;
        while char_position < line.len() {
            println!("For line {}, getting char at {} and putting it into stack #{}", line, char_position, iteration);
            let char_at = line.chars().nth(char_position).unwrap();
            if !char_at.is_whitespace() {
                let stack = stacks.get_mut(iteration).unwrap();
                stack.push(char_at.to_owned());
            }
            iteration += 1;
            char_position += 4
        }
    }

    stacks
}

use regex::Regex;

fn modify_stacks(mut stacks: Vec<Vec<char>>, instructions: Vec<&str>, move_all_at_once: bool) -> Vec<Vec<char>> {
    for line in instructions {
        // Regex approach:
        let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let captures = regex.captures(line).unwrap();
        let move_amount: usize = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from: usize = captures.get(2).unwrap().as_str().parse::<usize>().unwrap()-1;
        let to: usize = captures.get(3).unwrap().as_str().parse::<usize>().unwrap()-1;

        if move_all_at_once {
            println!("Moving {} crates from stack {}:{:?}) to {}:{:?}",
                     move_amount,
                     from, &stacks.get(from).unwrap(),
                     to, &stacks.get(to).unwrap());
            let mut moved_elems: Vec<char>;
            {
                let stack_from = stacks.get_mut(from).unwrap();
                let len = stack_from.len();
                moved_elems = stack_from.splice(len-(move_amount)..len, []).collect();
            }
            let stack_to = stacks.get_mut(to).unwrap();
            stack_to.append(&mut moved_elems)
        } else {
            for _ in 0..move_amount {
                // Soooo... for some stupid reason I cannot get two sub-vecs as mut at the same time.
                // I'll have to get one, get the last element, forget it again, then get the other and push it.
                // HOW annoying!!!
                println!("Moving crate from stack {}:{:?}) to {}:{:?}",
                         from, &stacks.get(from).unwrap(),
                         to, &stacks.get(to).unwrap());
                let elem;
                {
                    let stack_from = stacks.get_mut(from).unwrap();
                    elem = stack_from.remove(stack_from.len() - 1);
                }
                let stack_to = stacks.get_mut(to).unwrap();
                stack_to.push(elem)
            }
        }
    }
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;   // Making functions from outer module accessible

    const TEST_INPUT: &str = include_str!("../../inputs/day5/test.in");

    #[test]
    fn check_stacks_are_correct() {
        let (num_stacks, stack_setup, _moves) = setup(TEST_INPUT);
        let vecs = parse_stacks(stack_setup, num_stacks);
        for vec in &vecs {
            println!("Got vec {:?}", vec)
        }
        assert_eq!(&'Z', vecs.get(0).unwrap().get(0).unwrap());
        assert_eq!(&'N', vecs.get(0).unwrap().get(1).unwrap());
        assert_eq!(&'M', vecs.get(1).unwrap().get(0).unwrap());
        assert_eq!(&'D', vecs.get(1).unwrap().get(2).unwrap());
    }

    #[test]
    fn solution_part_1() {
        assert_eq!("CMZ", get_tops_of_final_stack(TEST_INPUT, false))
    }

    #[test]
    fn solution_part_2() {
        assert_eq!("MCD", get_tops_of_final_stack(TEST_INPUT, true));
    }
}
