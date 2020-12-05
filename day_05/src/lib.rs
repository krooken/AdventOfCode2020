use std::fs;

struct BoardingPass {
    row: String,
    column: String,
    row_nr: Option<u32>,
    column_nr: Option<u32>,
}

impl BoardingPass {
    fn new(row: String, column: String) -> BoardingPass {
        BoardingPass {
            row,
            column,
            row_nr: None,
            column_nr: None,
        }
    }

    fn id(self) -> u32 {
        let row_nr = self.row_nr.unwrap();
        let column_nr = self.column_nr.unwrap();
        row_nr * 8 + column_nr
    }
}

fn read_boarding_pass(text: &str) -> BoardingPass {
    BoardingPass::new(text[..7].to_string(), text[7..].to_string())
}

fn parse_boarding_pass(pass: &mut BoardingPass) {
    let row_nr = generic_binary_parse(&pass.row, |byte| match byte {
        b'F' => 0,
        b'B' => 1,
        _ => panic!(),
    });
    pass.row_nr = Some(row_nr);
    let column_nr = generic_binary_parse(&pass.column, |byte| match byte {
        b'L' => 0,
        b'R' => 1,
        _ => panic!(),
    });
    pass.column_nr = Some(column_nr);
}

fn generic_binary_parse<F>(text: &str, f: F) -> u32
where
    F: Fn(&u8) -> u32,
{
    text.as_bytes()
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, byte)| acc + f(byte) * 2u32.pow(i as u32))
}

fn get_max(text: &str) -> u32 {
    text.lines()
        .map(|row| {
            let mut pass = read_boarding_pass(row);
            parse_boarding_pass(&mut pass);
            pass
        })
        .fold(0, |acc, pass| {
            let id = pass.id();
            if id > acc {
                id
            } else {
                acc
            }
        })
}

pub fn get_max_pass_id(filename: &str) -> u32 {
    let text = fs::read_to_string(filename).unwrap();
    get_max(&text)
}

pub fn get_missing_seat(filename: &str) -> u32 {
    let text = fs::read_to_string(filename).unwrap();
    let mut sorted_passes = text
        .lines()
        .map(|row| {
            let mut pass = read_boarding_pass(row);
            parse_boarding_pass(&mut pass);
            pass.id()
        })
        .collect::<Vec<u32>>();
    sorted_passes.sort_unstable();
    let mut last_id = *sorted_passes.first().unwrap();
    let mut missing_id = 0;
    for id in sorted_passes.into_iter().skip(1) {
        if id == last_id + 2 {
            missing_id = id - 1;
            break;
        }
        last_id = id;
    }
    missing_id
}

#[cfg(test)]
mod tests {
    use crate::{get_max_pass_id, parse_boarding_pass, read_boarding_pass};

    #[test]
    fn test_split() {
        let text = "FBFBBFFRLR";
        let pass = read_boarding_pass(text);
        assert_eq!("FBFBBFF".to_string(), pass.row);
        assert_eq!("RLR".to_string(), pass.column);
    }

    #[test]
    fn test_parse() {
        let text = "FBFBBFFRLR";
        let mut pass = read_boarding_pass(text);
        parse_boarding_pass(&mut pass);
        assert_eq!(Some(44), pass.row_nr);
        assert_eq!(Some(5), pass.column_nr);
    }

    #[test]
    fn test_max() {
        assert_eq!(820, get_max_pass_id("data/example.txt"));
    }
}
