use std::collections::HashMap;
use itertools::Itertools;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {
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

    let mut iterations = 0;
    let mut position = "AAA";
    let mut index = 0;
    loop {
        iterations += 1;
        position = network.get(position).unwrap().get(step_guide[index]).unwrap();
        if position == "ZZZ" {
            break;
        }
        if index == step_guide.len()-1 {
            index = 0;
        } else {
            index += 1;
        }
    }

    Ok(iterations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_step_process() -> miette::Result<()> {
        let input = include_str!("../test_2_steps.txt");
        assert_eq!(2, process(input)?);
        Ok(())
    }

    #[test]
    fn test_6_step_process() -> miette::Result<()> {
        let input = include_str!("../test_6_steps.txt");
        assert_eq!(6, process(input)?);
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
