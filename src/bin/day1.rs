
// Macro to execute this code with the input for day '1'
// Basically it takes '1' as argument, and `fn main() ...` as another parameter (TokenStream),
// then loads the file matching with `1` and passes that to run its own main function,
// from where it calls whatever is passed in here.
// (Or at least something slightly similar is what's happening).
#[aoc::main(1)]
fn main(input: &str) {
    let calories_for_elves = input.lines().collect();
    println!("Elf with the most calories has {} calories!", highest_calory_elf(&calories_for_elves));
    println!("The top three Elves have {} calories together.", top_three(&calories_for_elves));
}

pub fn highest_calory_elf(elf_calories: &Vec<&str>) -> u32 {
    let mut highest: u32 = 0;
    let mut sum: u32 = 0;
    for line in elf_calories {
        if line.is_empty() {
            highest = if highest > sum { highest } else { sum };
            sum = 0
        } else {
            sum = sum + line.parse::<u32>().expect("Oh, this wasn't a number");
        }
    }
    highest
}

pub fn top_three(elf_calories: &Vec<&str>) -> u32 {
    let mut highest_three: [u32; 3] = [0, 0, 0];
    highest_three.sort();
    let mut sum: u32 = 0;
    for line in elf_calories {
        if line.is_empty() {
            if highest_three[0] < sum {
                highest_three[0] = sum;
                highest_three.sort()
            }
            sum = 0
        } else {
            sum = sum + line.parse::<u32>().expect("Oh, this wasn't a number");
        }
    }
    if highest_three[0] < sum {
        highest_three[0] = sum;
        highest_three.sort()
    }
    highest_three.iter().sum()
}

// pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
//     let file = File::open(Path::new("./src/").join(filename.as_ref())).expect("File not available?");
//     let reader = BufReader::new(file);
//     reader.lines()
//         .map(|l| l.expect("Eh? Could not parse this line :-("))
//         .collect()
// }

#[cfg(test)]
mod tests {
    use super::*;   // Making functions from outer module accessible?

    // Putting this up here so that I don't forget to update that for each new day.
    const TEST_INPUT: &str = include_str!("../../inputs/day1/test.in");

    #[test]
    fn get_highest_calory_elf() {
        let input = TEST_INPUT.lines().collect();
        assert_eq!(24000, highest_calory_elf(&input))
    }

    #[test]
    fn get_top_three() {
        let input = TEST_INPUT.lines().collect();
        assert_eq!(45000, top_three(&input))
    }
}
