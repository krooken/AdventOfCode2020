use std::fs;

fn get_nr_differences(filename: &str) -> (u32, u32, u32) {
    let mut adapters: Vec<u32> = fs::read_to_string(filename).unwrap().lines().map(|e| e.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort();
    let mut result = (0, 0, 0);
    let mut prev = &adapters[0];
    let clos = |diff: u32, result: &mut (u32, u32, u32)| {
        match diff {
            1 => result.0 += 1,
            2 => result.1 += 1,
            3 => result.2 += 1,
            _ => panic!(),
        }
    };
    for cur in adapters.iter().skip(1) {
        clos(cur - prev, &mut result);
        prev = cur;
    }
    adapters.push(adapters.last().unwrap() + 3);
    let last = adapters.len() - 1;
    clos(adapters[last] - adapters[last - 1], &mut result);
    result
}

#[cfg(test)]
mod tests {
    use crate::get_nr_differences;

    #[test]
    fn test_nr_differences() {
        assert_eq!((22, 0, 10), get_nr_differences("data/example.txt"));
    }
}
