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

fn find_pair_with_sum(sum: u32, values: &Vec<u32>) -> (u32, u32) {
    for i in 0..values.len() {
        for j in i+1..values.len() {
            if values[i] + values[j] == sum {
                return (values[i], values[j]);
            }
        }
    }
    panic!("Didn't find the sum");
}


#[cfg(test)]
mod tests {
    use crate::{get_expenses, find_pair_with_sum};

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

    #[test]
    fn two_elements() {
        assert_eq!((2,3), find_pair_with_sum(5, &vec![2,3]));
    }

    #[test]
    fn ten_elements() {
        assert_eq!((7,3), find_pair_with_sum(10, &vec![1, 7, 4, 2, 5, 11, 44, 1, 3, 0]));
    }
}
