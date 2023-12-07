use crate::custom_error::AocError;

struct Rect {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
    val: usize
}

impl Rect {

    fn contains(&self, sym: &Symbol) -> bool {
        return self.contains_coords(sym.x, sym.y)
    }
    fn contains_coords(&self, x: usize, y: usize) -> bool {
        return self.left <= x && x <= self.right && self.top <= y && y <= self.bottom;
    }
}

struct Symbol {
    x: usize,
    y: usize
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {

    // Approach:
    // * Get all numbers together with its Rect (left = start-1, right = end+1, top = line# -1, bottom = line# +1)
    // * Get symbols: coordinates of all non-dots-non-digits
    // * For each number, check whether any symbol is within range

    // 1st: Box the input so that we don't need to worry about overflows etc
    // -> give it an extra line at top and bottom, and extra col left and right
    let lines_with_extra_cols:String = input.lines().map(|line| format!(".{line}.\n")).collect();
    let boxed_input = format!("\n{}\n", lines_with_extra_cols);

    let mut rects: Vec<Rect> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    boxed_input.lines().enumerate().for_each(|(index, line)| {
        let mut left:Option<usize> = None;
        let mut right:Option<usize> = None;
        let mut val_string = String::from("");
        line.char_indices().for_each(|(pos, c)| {
            match c {
                x if x.is_digit(10) => {
                    if left == None {
                        left = Some(pos);
                        right = Some(pos);
                        val_string.push(c);
                    } else {
                        right = Some(pos);
                        val_string.push(c);
                    }
                }
                x => {
                    if x != '.' {
                        symbols.push(Symbol { x: pos, y: index });
                    }
                    if left != None {
                        rects.push(Rect {
                            left: left.unwrap() - 1,
                            right: right.unwrap() + 1,
                            top: index - 1,
                            bottom: index + 1,
                            val: val_string.parse().unwrap()
                        });
                        left = None;
                        right = None;
                        val_string = String::from("");
                    }
                }
            }
        });
    });

    let result = rects.iter()
        .map(|rect| {
            if symbols.iter()
                .any(|symbol| rect.contains(symbol)) {
                rect.val
            } else {
                0
            }
        }).sum();

    Ok(result)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test.txt");
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}
