// Macro to execute this code with the input for the day.
// Basically it takes a number as argument, and `fn main() ...` as another parameter (TokenStream),
// then loads the file matching to the day and passes that to run its own main function,
// from where it calls whatever is passed in here.
// (Or at least something slightly similar is what's happening).

// Todo: Replace this number with the day you want to run!
#[aoc::main(2)]
fn main(input: &str) {
    let data = input.lines().collect();
    println!("Todo: do something with this data and print out the solution. Test output: {}", calculate_something(&data));
}

pub fn calculate_something(data: &Vec<&str>) -> u32 {
    data[0].parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;   // Making functions from outer module accessible

    // Todo: replace the number in this path
    const TEST_INPUT: &str = include_str!("../../inputs/day2/test.in");

    #[test]
    fn solution_part_1() {
        let input = TEST_INPUT.lines().collect();
        assert_eq!(12345, calculate_something(&input))
    }

    #[test]
    fn solution_part_2() {
        let input = TEST_INPUT.lines().collect();
        assert_eq!(98765, calculate_something(&input))
    }
}
