use std::collections::HashMap;
use std::ops::Deref;
use itertools::Itertools;
use lcmx::lcmx;
use num::BigInt;
use num::integer::lcm;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<BigInt, AocError> {
    let mut lines = input.lines();
    let step_guide = lines.next().unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect_vec();

    let mut network: HashMap<&str, Vec<&str>> = Default::default();
    lines.for_each(|line| {
        if line.is_empty() {
            return;
        }
        if let Some((key, values)) = line.split_once(" = ") {
            if let Some((left, right)) = values
                .strip_prefix("(")
                .and_then(|s| s.strip_suffix(")"))
                .and_then(|s| s.split_once(", "))
            {
                network.insert(key, vec![left, right]);
            } else {
                println!("Parsing error in line {line}")
            };

        } else {
            println!("Errr...")
        }
    });
    network.insert("0", vec!["0", "0"]);    // For loops that we know are already closed.

    // Todo: new approach:
    // for each position, there is probably a 'circularity' where it ... might? ... end up at the same starting position
    // after x iterations (and then it would repeat).
    // That circularity (if it indeed exists?) might be of different length across all 6 circles;
    // and calculating when in each it would have the z
    // ('lowest common multiple' (LCM), i.e. 6 & 8 -> 24)
    // This _only_ works IF there is actually some circularity.
    // To check, we'll have to record every key that each one is iterating over along with the position (iteration number),
    // and then see whether this matches. Note that it might take several iterations over index to fill one circle.

    // get all starting positions: all that end with 'A':
    let mut positions: Vec<Vec<&str>> = vec![];
    let initial_positions = network.iter()
        .filter_map(|(key, val)| if key.ends_with("A") { Some(*key) } else { None })
        .collect_vec();
    positions.push(initial_positions);

    let mut iterations = 0;
    let mut index = 0;
    let mut ghosts_to_check_on_next_iteration: Vec<usize> = vec![];
    let mut loop_lengths: Vec<usize> = vec![];
    loop {
        iterations += 1;
        let mut new_positions = positions.last().unwrap().iter()
            .map(|position| network.get(position).unwrap()
                .get(step_guide[index]).unwrap().deref())
            .collect_vec();
        if !ghosts_to_check_on_next_iteration.is_empty() {
            ghosts_to_check_on_next_iteration.iter().for_each(|ghost| {
                // let first_position_after_z = new_positions[*ghost];
                // let mut start_pos = iterations-1 - step_guide.len();
                // while start_pos > 0 {
                //     if first_position_after_z == positions[start_pos][*ghost] {
                        println!("loop for Ghost {ghost} for {iterations} steps at index {index}");
                // let ghost_loop = positions.iter().map(|all_ghosts| all_ghosts[*ghost]).join(" -> ");
                // println!("{ghost_loop}");
                        loop_lengths.push(iterations-1);
                        // ok, we don't need to check this any more, so remove it from positions!
                        new_positions.remove(*ghost);
                        new_positions.insert(*ghost, "0");
                        // break;
                    // }
                    // start_pos = start_pos - step_guide.len();
                // }
            });

            ghosts_to_check_on_next_iteration.clear();
        }
        if loop_lengths.len() == positions[0].len() {
            break;
        }
        let ending_with_z = new_positions.iter().enumerate().filter(|(index, pos)| {
            if pos.ends_with("Z") {
                ghosts_to_check_on_next_iteration.push(*index);
                return true;
            }
            return false;
        }).count();
        if ending_with_z > 0 {
            println!("{:?} , ended with Z!", new_positions);
            if ending_with_z == positions.len() {
                println!("All positions had a Z 🎉");
                break;
            }
        }
        positions.push(new_positions);
        if index == step_guide.len()-1 {
            index = 0;
        } else {
            index += 1;
        }
    }

    if loop_lengths.len() == positions[0].len() {
        let vec1 = loop_lengths.iter().map(|&x| BigInt::from(x)).collect_vec();

        let mut result = BigInt::from(1);
        for x in vec1 {
            result = lcm(result, x);
        }
        Ok(result)
        // Ok(vec1.iter().fold(1u64, |acc, &num| lcm(acc, num)) as usize)
        // Ok(lcmx(&vec1).unwrap() as usize)
    } else {
        Ok(BigInt::from(iterations))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_step_process() -> miette::Result<()> {
        let input = include_str!("../test_2_steps.txt");
        assert_eq!(BigInt::from(2), process(input)?);
        Ok(())
    }

    #[test]
    fn test_6_step_process() -> miette::Result<()> {
        let input = include_str!("../test_6_steps.txt");
        assert_eq!(BigInt::from(6), process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2_process() -> miette::Result<()> {
        let input = include_str!("../test_part2.txt");
        assert_eq!(BigInt::from(6), process(input)?);
        Ok(())
    }

    #[test]
    fn real_process() -> miette::Result<()> {
        let input = include_str!("../input.txt");
        let result = process(input)?;
        println!("Result: {result}");
        Ok(())
    }
}
