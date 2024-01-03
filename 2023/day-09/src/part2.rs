use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<isize, AocError> {
    let result = input.lines().map(|line| {
        let nums: Vec<isize> = line.split_whitespace().filter_map(|num| num.parse().ok()).collect();
        let mut first_nums = vec![nums[0].clone()];
        let mut derived = nums.clone();
        while !derived.iter().all(|&num| num == 0) {
            let mut temp = vec![];
            for i in 1..derived.len() {
                let i1 = derived[i] - derived[i - 1];
                temp.push(i1);
            }
            derived = temp;
            first_nums.push(derived[0].clone());
        }

        let mut final_nums: Vec<isize> = vec![0];
        for i in 0..first_nums.len() {
            final_nums.push(first_nums[first_nums.len()-1 - i] - final_nums.last().unwrap());
        }
        final_nums.last().unwrap().clone()
    }).sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(2, process(input)?);
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
