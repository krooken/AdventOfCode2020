use std::cmp::{max, min};
use std::fs;

#[derive(Copy, Clone)]
enum Occupied {
    Free,
    Taken,
    Floor,
}

fn find_neigbors(seating: &Vec<Vec<Occupied>>, i: usize, j: usize) -> u32 {
    let mut nr_neighbors = 0;
    let i_32 = i as i32;
    let i_low = max(0, i_32-1) as usize;
    let i_high = min(i_32+2, seating.len() as i32) as usize;
    let j_32 = j as i32;
    let j_low = max(0, j_32-1) as usize;
    let j_high = min(j_32+2, seating[i].len() as i32) as usize;
    for k in i_low..i_high {
        for l in j_low..j_high {
            if !(k == i && l == j) {
                if let Occupied::Taken = seating[k][l] {
                    nr_neighbors += 1;
                }
            }
        }
    };
    nr_neighbors
}

fn update_by_neighbors(seating: &Vec<Vec<Occupied>>, i: usize, j: usize) -> (Occupied, bool) {
    let nr_neighbors = find_neigbors(seating, i, j);
    match seating[i][j] {
        Occupied::Free => {
            if nr_neighbors == 0 {
                (Occupied::Taken, true)
            } else {
                (Occupied::Free, false)
            }
        },
        Occupied::Taken => {
            if nr_neighbors >= 4 {
                (Occupied::Free, true)
            } else {
                (Occupied::Taken, false)
            }
        },
        Occupied::Floor => (Occupied::Floor, false),
    }
}

fn simulation_step<F>(seating: &Vec<Vec<Occupied>>, f: F) -> (Vec<Vec<Occupied>>, bool)
where F: Fn(&Vec<Vec<Occupied>>, usize, usize) -> (Occupied, bool) {
    let mut next_step = Vec::new();
    let mut updated = false;
    for i in 0..seating.len() {
        next_step.push(Vec::new());
        for j in 0..seating[i].len() {
            let (occ, up) = f(seating, i, j);
            updated |= up;
            next_step[i].push(occ);
        }
    }
    (next_step, updated)
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

fn simulation_step_with_neighbors(seating: &Vec<Vec<Occupied>>) -> (Vec<Vec<Occupied>>, bool) {
    simulation_step(&seating, |b, i, j| {
            update_by_neighbors(b, i, j)
        })
}

pub fn simulate(filename: &str) -> u32 {
    let mut board = read_seating(filename);
    let mut updated = true;
    while updated {
        let (next, up) = simulation_step_with_neighbors(&board);
        board = next;
        updated = up;
    }
    count_occupied(&board)
}

#[cfg(test)]
mod tests {
    use crate::{read_seating, count_occupied, simulate, simulation_step_with_neighbors};

    #[test]
    fn test_count_and_read() {
        let seating = read_seating("data/example.txt");
        assert_eq!(0, count_occupied(&seating));
    }

    #[test]
    fn test_simulation_step1() {
        let seating = read_seating("data/example.txt");
        let (next_step, updated) = simulation_step_with_neighbors(&seating);
        assert_eq!(71, count_occupied(&next_step));
        assert!(updated);
    }

    #[test]
    fn test_simulation_step2() {
        let mut next_step = read_seating("data/example.txt");
        for _ in 0..2 {
            let (next, updated) = simulation_step_with_neighbors(&next_step);
            next_step = next;
            assert!(updated);
        }
        assert_eq!(20, count_occupied(&next_step));
    }

    #[test]
    fn test_simulation_step5() {
        let mut next_step = read_seating("data/example.txt");
        for _ in 0..5 {
            let (next, updated) = simulation_step_with_neighbors(&next_step);
            next_step = next;
            assert!(updated);
        }
        assert_eq!(37, count_occupied(&next_step));
    }

    #[test]
    fn test_simulate() {
        assert_eq!(37, simulate("data/example.txt"));
    }
}
