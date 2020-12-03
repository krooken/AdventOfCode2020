use std::fs;

pub fn count_trees(filename: &str) -> u32 {
    let forest = fs::read_to_string(filename).unwrap();
    count_trees_for_slope(&forest, 1, 3)
}

fn count_trees_for_slope(forest: &str, rows: usize, columns: usize) -> u32 {
    let mut column = 0;
    let mut count = 0;
    for line in forest.lines().step_by(rows) {
        let pos = &line[column..column + 1];
        if pos == "#" {
            count += 1;
        }
        column = (column + columns) % line.len();
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{count_trees, count_trees_for_slope};
    use std::fs;

    #[test]
    fn test_example() {
        assert_eq!(7, count_trees("data/example.txt"));
    }

    #[test]
    fn test_1_and_1() {
        let forest = fs::read_to_string("data/example.txt").unwrap();
        assert_eq!(2, count_trees_for_slope(&forest, 1, 1));
    }

    #[test]
    fn test_1_and_3() {
        let forest = fs::read_to_string("data/example.txt").unwrap();
        assert_eq!(7, count_trees_for_slope(&forest, 1, 3));
    }

    #[test]
    fn test_1_and_5() {
        let forest = fs::read_to_string("data/example.txt").unwrap();
        assert_eq!(3, count_trees_for_slope(&forest, 1, 5));
    }

    #[test]
    fn test_1_and_7() {
        let forest = fs::read_to_string("data/example.txt").unwrap();
        assert_eq!(4, count_trees_for_slope(&forest, 1, 7));
    }

    #[test]
    fn test_2_and_1() {
        let forest = fs::read_to_string("data/example.txt").unwrap();
        assert_eq!(2, count_trees_for_slope(&forest, 2, 1));
    }
}
