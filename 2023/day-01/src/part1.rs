use nom::character::complete::char;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {

    // let spelled_out = 

    let output = input
        .lines()
        .map(|line| {
            let first_pos = line.char_indices().find(|c: &(usize, char)| c.1.is_ascii_digit());
            let last_pos = line.char_indices().rfind(|c: &(usize, char)| c.1.is_ascii_digit());
            // theoretically it might not find it, but in that case I think it's totally ok to panic:
            if let Some(first) = first_pos.unwrap().1.to_digit(10) {
                if let Some(second) = last_pos.unwrap().1.to_digit(10) {
                    10 * first + second
                } else { 0 }
            } else { 0 }
        })
        .sum::<u32>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
