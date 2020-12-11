use std::cmp::{max, min};
use std::fs;

#[derive(Copy, Clone)]
enum Occupied {
    Free,
    Taken,
    Floor,
}

fn simulation_step(seating: &Vec<Vec<Occupied>>) -> Vec<Vec<Occupied>> {
    let mut next_step = Vec::new();
    for i in 0..seating.len() {
        next_step.push(Vec::new());
        for j in 0..seating[i].len() {
            let mut nr_neighbors = 0;
            for k in max(0, (i as i32)-1) as usize..min(i as i32+2, seating.len() as i32) as usize {
                for l in max(0, (j as i32)-1) as usize..min(j as i32+2, seating[i].len() as i32) as usize {
                    if !(k == i && l == j) {
                        if let Occupied::Taken = seating[k][l] {
                            nr_neighbors += 1;
                        }
                    }
                }
            }
            match seating[i][j] {
                Occupied::Free => {
                    if nr_neighbors == 0 {
                        next_step[i].push(Occupied::Taken);
                    } else {
                        next_step[i].push(Occupied::Free);
                    }
                },
                Occupied::Taken => {
                    if nr_neighbors >= 4 {
                        next_step[i].push(Occupied::Free);
                    } else {
                        next_step[i].push(Occupied::Taken);
                    }
                },
                Occupied::Floor => next_step[i].push(Occupied::Floor),
            }
        }
    }
    next_step
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
    use crate::{read_seating, count_occupied, simulation_step};

    #[test]
    fn test_count_and_read() {
        let seating = read_seating("data/example.txt");
        assert_eq!(0, count_occupied(&seating));
    }

    #[test]
    fn test_simulation_step1() {
        let seating = read_seating("data/example.txt");
        let next_step = simulation_step(&seating);
        assert_eq!(71, count_occupied(&next_step));
    }

    #[test]
    fn test_simulation_step2() {
        let mut next_step = read_seating("data/example.txt");
        for _ in 0..2 {
            next_step = simulation_step(&next_step);
        }
        assert_eq!(20, count_occupied(&next_step));
    }

    #[test]
    fn test_simulation_step5() {
        let mut next_step = read_seating("data/example.txt");
        for _ in 0..5 {
            next_step = simulation_step(&next_step);
        }
        assert_eq!(37, count_occupied(&next_step));
    }
}
