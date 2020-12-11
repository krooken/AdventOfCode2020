use std::fs;

#[derive(Copy, Clone)]
enum Occupied {
    Free,
    Taken,
    Floor,
}

fn count_occupied(seating: &Vec<Vec<Occupied>>) -> u32 {
    seating.iter().fold(0, |acc, vec| {
        vec.iter().fold(acc, |inner_acc, e| {
            match e {
                Occupied::Taken => inner_acc + 1,
                _ => inner_acc,
            }
        })
    })
}

fn read_seating(filename: &str) -> Vec<Vec<Occupied>> {
    let text = fs::read_to_string(filename).unwrap();
    text.lines().map(|line| {
        let mut l = Vec::new();
        for i in 0..line.len() {
            match &line[i..i+1] {
                "L" => l.push(Occupied::Free),
                "#" => l.push(Occupied::Taken),
                "." => l.push(Occupied::Floor),
                _ => panic!(),
            }
        }
        l
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::{read_seating, count_occupied};

    #[test]
    fn test_count_and_read() {
        let seating = read_seating("data/example.txt");
        assert_eq!(0, count_occupied(&seating));
    }
}
