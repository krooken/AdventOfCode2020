use std::fs;

fn get_expenses(filename: &str) -> Vec<u32> {
    let text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut expenses: Vec<u32> = Vec::new();
    for row in text.lines() {
        expenses.push(row.parse().expect(&format!("Parsing failed on {}", row)));
    }
    expenses
}


#[cfg(test)]
mod tests {
    use crate::get_expenses;

    #[test]
    fn first_element() {
        let vector = get_expenses("data/ExpenseReport.txt");
        assert_eq!(&1293, vector.first().unwrap());
    }

    #[test]
    fn last_element() {
        let vector = get_expenses("data/ExpenseReport.txt");
        assert_eq!(&1396, vector.last().unwrap());
    }
}
