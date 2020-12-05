struct BoardingPass {
    row: String,
    column: String,
    row_nr: Option<u32>,
    column_nr: Option<u32>,
}

impl BoardingPass {
    fn new(row: String, column: String) -> BoardingPass {
        BoardingPass{
            row,
            column,
            row_nr: None,
            column_nr: None,
        }
    }
}

fn read_boarding_pass(text: &str) -> BoardingPass {
    BoardingPass::new(text[..7].to_string(), text[7..].to_string())
}

#[cfg(test)]
mod tests {
    use crate::read_boarding_pass;

    #[test]
    fn test_split() {
        let text = "FBFBBFFRLR";
        let pass = read_boarding_pass(text);
        assert_eq!("FBFBBFF".to_string(), pass.row);
        assert_eq!("RLR".to_string(), pass.column);
    }
}
