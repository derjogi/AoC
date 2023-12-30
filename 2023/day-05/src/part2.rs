use std::collections::HashSet;
use std::ops::{Deref, Range};
use itertools::Itertools;
use crate::custom_error::AocError;
use rayon::prelude::*;

#[derive(Clone)]
struct Data {
    mappings: Vec<Mapping>,
    seed_positions: Vec<usize>
}

#[derive(Clone)]
struct Mapping {
    from_name: Option<String>,
    to_name: Option<String>,
    dest_ranges: Vec<Range<usize>>,
    source_ranges: Vec<Range<usize>>,
    mapped_positions: Vec<usize>
}

impl Mapping {
    fn calculate_destination(&self, source_positions: &Vec<usize>, dest_positions: &mut Vec<usize>) {
        source_positions.iter()
            .for_each(|seed| {
                let mut pos:Option<usize> = None;
                let mut range_index: Option<usize> = None;

                for (index, range) in self.source_ranges.iter().enumerate() {
                    if range.contains(&seed) {
                        pos = Some(seed - range.start);
                        range_index = Some(index);
                        break;
                    };
                };

                if pos == None {
                    dest_positions.push(seed.clone());
                } else {
                    dest_positions.push(self.dest_ranges[range_index.unwrap()].start+pos.unwrap());
                }
            });
    }
}

#[derive(Clone, Copy)]
enum State {
    Seeds,
    Map_Init,
    Map_Coords,
    Map_Coords_Other
}

fn parse_data(string: &str) -> Data {
    let mut data = Data { /* Initialize data structure */ mappings: vec![], seed_positions: Default::default() };
    let mut state = State::Seeds;

    for line in string.lines() {
        match (state, line.trim()) {
            // Transition to the next section based on identifier lines
            (State::Seeds, line) if line.starts_with("seeds:") => {
                let split_seeds = line["seeds: ".len()..]
                    .split_whitespace()
                    .map(|seed| seed.parse().unwrap()).collect::<Vec<usize>>();

                let length = split_seeds.len();
                let mut ranges = vec![];
                for i in (0..length).step_by(2) {
                    let start = split_seeds[i];
                    let end = start+split_seeds[i + 1];
                    ranges.push(start..end);
                }

                data.seed_positions = ranges.into_par_iter().flat_map(|range| range).collect::<Vec<_>>();

                println!("Parsed initial seeds");
            },

            (_, line) if line.is_empty() => {
                state = State::Map_Init;
            }

            (State::Map_Init, line) => {
                let (from, to) = line.split_once(" ").unwrap().0.split_once("-to-").unwrap();
                data.mappings.push(Mapping {
                    from_name: Some(String::from(from)),
                    to_name: Some(String::from(to)),
                    dest_ranges: vec![],
                    source_ranges: vec![],
                    mapped_positions: Default::default(),
                });
                state = State::Map_Coords;
            },

            (State::Map_Coords, line) => {
                let map = data.mappings.last_mut().unwrap();
                let dest_source_lenght: Vec<usize> = line
                    .splitn(3, " ")
                    .map(|num| num.parse::<usize>().unwrap()).collect();
                map.dest_ranges.push(dest_source_lenght[0]..dest_source_lenght[0]+dest_source_lenght[2]);
                map.source_ranges.push(dest_source_lenght[1]..dest_source_lenght[1] + dest_source_lenght[2]);
            }

            _ => { panic!("Shouldn't be here") }
        }
    }

    data
}


#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {
    let mut data = parse_data(input);
    let length = data.mappings.len();
    let mut current_mapping = &mut data.mappings[0];
    let mut source_positions: Vec<usize> = data.seed_positions.clone();
    let mut dest_positions: Vec<usize> = Default::default();
    for i in 0..length-1 {
        current_mapping.calculate_destination(&source_positions, &mut dest_positions);
        source_positions = dest_positions.clone();
        dest_positions.clear();
        current_mapping = &mut data.mappings[i+1];
    }
    // Final result; get the smallest value from the set
    return Ok(source_positions.iter().sorted().collect_vec()[0].clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(46, process(input)?);
        Ok(())
    }

    #[test]
    fn real_process() -> miette::Result<()> {
        let input = include_str!("../input.txt");
        let result = process(input)?;
        assert!(result < 4141696360);
        println!("Result: {result}");
        Ok(())
    }
}
