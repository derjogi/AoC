use std::collections::HashMap;
use std::ops::{Deref, Range};
use std::ptr::eq;
use itertools::Itertools;
use crate::custom_error::AocError;

#[derive(Clone)]
struct Data {
    seeds:Option<Vec<usize>>,
    mappings: Vec<Mapping>,
    // The initial seed, and the value of it after each transition
    seed_transitions: HashMap<usize, Vec<usize>>
}

#[derive(Clone)]
struct Mapping {
    from_name: Option<String>,
    to_name: Option<String>,
    dest_ranges: Vec<Range<usize>>,
    source_ranges: Vec<Range<usize>>,
}

#[derive(Clone, Copy)]
enum State {
    Seeds,
    Map_Init,
    Map_Coords,
    Map_Coords_Other
}

fn parse_data(string: &str) -> Data {
    let mut data = Data { /* Initialize data structure */ seeds: None, mappings: vec![], seed_transitions: HashMap::new() };
    let mut state = State::Seeds;

    for line in string.lines() {
        match (state, line.trim()) {
            // Transition to the next section based on identifier lines
            (State::Seeds, line) if line.starts_with("seeds:") => {
                let seeds:Vec<usize> = line["seeds: ".len()..]
                    .split_whitespace()
                    .map(|num| {
                        let seed = num.parse::<usize>().unwrap();
                        data.seed_transitions.insert(seed.clone(), vec![seed.clone()]);
                        seed
                    })
                    .collect();
                data.seeds = Some(seeds);
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
    let mut map_id = "seed";
    while true {
        let current_mapping = data.mappings.iter()
            .find(|map| {
                map_id == map.from_name.clone().unwrap().as_str()
            });
        if current_mapping.is_none() {
            return Ok(data.seed_transitions.iter().map(|(_, transitions)| transitions.last().unwrap()).min().cloned().unwrap())
        } else {
            let mapping = current_mapping.unwrap();
            if let Some(destination) = &mapping.to_name {
                map_id = destination;
            }
            let source_ranges = &mapping.source_ranges;
            let dest_ranges = &mapping.dest_ranges;

            data.seed_transitions.iter_mut()
                .for_each(|(init_seed, later_seeds)| {
                    let mut pos:Option<usize> = None;
                    let mut range_index: Option<usize> = None;
                    let last = later_seeds.last().cloned().unwrap();
                    for (index, range) in source_ranges.iter().enumerate() {
                        if range.contains(&last) {
                            pos = Some(&last - range.start);
                            range_index = Some(index);
                            break;
                        };
                    };

                    if pos == None {
                        later_seeds.push(last.clone())
                    } else {
                        later_seeds.push(dest_ranges[range_index.unwrap()].start+pos.unwrap())
                    }
                })
        }
    }
    let result: usize = data.seed_transitions.iter().map(|(seed, transitions)| transitions.last().unwrap()).sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(35, process(input)?);
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
