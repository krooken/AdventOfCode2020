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

pub fn get_product(filename: &str) -> u32 {
    let (one, _, three) = get_nr_differences(filename);
    one * three
}

fn traverse_arrangements(adapters: &Vec<u32>, current: usize, known: &mut Vec<Option<u64>>) -> u64 {
    if current == adapters.len() - 1 {
        1
    } else {
        let mut sum = 0;
        let cur_jolt = adapters[current];
        for (i, next_jolt) in adapters.iter().enumerate().skip(current + 1).take(3) {
            let diff = next_jolt - cur_jolt;
            if diff > 0 && diff < 4 {
                if let Some(num) = known[i] {
                    sum += num;
                } else {
                    let res = traverse_arrangements(adapters, i, known);
                    known[i] = Some(res);
                    sum += res;
                }
            }
        }
        sum
    }
}

pub fn get_arrangements(filename: &str) -> u64 {
    let mut adapters: Vec<u32> = fs::read_to_string(filename).unwrap().lines().map(|e| e.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort();
    let i = adapters.len() - 1;
    adapters.push(adapters[i] + 3);
    let mut known: Vec<Option<u64>> = (0..adapters.len() as u64).map(|_| None).collect();
    traverse_arrangements(&adapters, 0, &mut known)
}

#[cfg(test)]
mod tests {
    use crate::{get_nr_differences, get_arrangements, get_product};

    #[test]
    fn test_nr_differences() {
        assert_eq!((22, 0, 10), get_nr_differences("data/example.txt"));
    }

    #[test]
    fn test_arrangements() {
        assert_eq!(19208, get_arrangements("data/example.txt"));
    }

    #[test]
    fn test_task1() {
        assert_eq!(2482, get_product("data/adapters.txt"));
    }

    #[test]
    fn test_task2() {
        assert_eq!(96_717_311_574_016, get_arrangements("data/adapters.txt"));
    }
}
