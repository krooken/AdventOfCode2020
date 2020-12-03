use std::fs;

pub fn get_expenses(filename: &str) -> Vec<u32> {
    let text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    text.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn find_pair_with_sum(sum: u32, values: &Vec<u32>) -> (u32, u32) {
    for i in 0..values.len() {
        for j in i+1..values.len() {
            if values[i] + values[j] == sum {
                return (values[i], values[j]);
            }
        }
    }
    panic!("Didn't find the sum");
}

pub fn product_of_pair(values: (u32, u32)) -> u32 {
    values.0 * values.1
}

pub fn find_triple_with_sum(sum: u32, values: &Vec<u32>) -> (u32, u32, u32) {
    for i in 0..values.len() {
        for j in i+1..values.len() {
            for k in j+1..values.len() {
                if values[i] + values[j] + values[k] == sum {
                    return (values[i], values[j], values[k]);
                }
            }
        }
    }
    panic!("Didn't find the sum");
}

pub fn product_of_triple(values: (u32, u32, u32)) -> u32 {
    values.0 * values.1 * values.2
}


#[cfg(test)]
mod tests {
    use crate::{get_expenses, find_pair_with_sum, product_of_pair, find_triple_with_sum, product_of_triple};

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

    #[test]
    fn product_1() {
        assert_eq!(0, product_of_pair((0, 2)));
    }

    #[test]
    fn product_2() {
        assert_eq!(2, product_of_pair((1, 2)));
    }

    #[test]
    fn product_3() {
        assert_eq!(96, product_of_pair((12, 8)));
    }

    #[test]
    fn test_task1() {
        let values = get_expenses("data/ExpenseReport.txt");
        let pair = find_pair_with_sum(2020, &values);
        assert_eq!(793524, product_of_pair(pair));
    }

    #[test]
    fn test_task2() {
        let values = get_expenses("data/ExpenseReport.txt");
        let triple = find_triple_with_sum(2020, &values);
        assert_eq!(61515678, product_of_triple(triple));
    }
}
