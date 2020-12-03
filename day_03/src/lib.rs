use std::fs;

pub fn count_trees(filename: &str) -> u32 {
    let forest = fs::read_to_string(filename).unwrap();
    count_trees_for_slope(&forest)
}

fn count_trees_for_slope(forest: &String) -> u32 {
    let mut column = 0;
    let mut count = 0;
    for line in forest.lines() {
        let pos = &line[column..column + 1];
        if pos == "#" {
            count += 1;
        }
        column = (column + 3) % line.len();
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::count_trees;

    #[test]
    fn test_example() {
        assert_eq!(7, count_trees("data/example.txt"));
    }
}
