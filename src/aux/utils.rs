
pub mod utils {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    pub fn parse_input(input: &str) -> Vec<&str> {
        let lines: Vec<&str> = input.split("\n").collect();
        lines
    }

    pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(Path::new("./src/").join(filename.as_ref())).expect("File not available?");
        let reader = BufReader::new(file);
        reader.lines()
            .map(|l| l.expect("Eh? Could not parse this line :-("))
            .collect()
    }
}
