use std::fs;

fn extreme_coordinate<F>(cubes: &Vec<(i64, i64, i64)>, f: F) -> (i64, i64, i64)
where F: Fn(i64, i64) -> bool {
    cubes.iter().fold((0, 0, 0), |(x_acc, y_acc, z_acc), (x, y, z)| {
        let mut m = (x_acc, y_acc, z_acc);
        if f(*x, x_acc) {
            m.0 = *x;
        }
        if f(*y, y_acc) {
            m.1 = *y;
        }
        if f(*z, z_acc) {
            m.2 = *z;
        }
        m
    })
}

fn max_coordinate(cubes: &Vec<(i64, i64, i64)>) -> (i64, i64, i64) {
    extreme_coordinate(cubes, |v, acc| v > acc)
}

fn min_coordinate(cubes: &Vec<(i64, i64, i64)>) -> (i64, i64, i64) {
    extreme_coordinate(cubes, |v, acc| v < acc)
}

fn get_coordinates(text: &str) -> Vec<(i64, i64, i64)> {
    text.lines().enumerate().map(|(i, line)| {
        line.chars().enumerate().filter(|(_, c)| c == &'#').map(move |(j, _)| {
            (j as i64, i as i64, 0)
        })
    }).flatten().collect()
}

fn print_grid(cubes: &Vec<(i64, i64, i64)>) {
    let min = min_coordinate(cubes);
    let max = max_coordinate(cubes);
    for z in min.2..max.2+1 {
        println!("z={}", z);
        for y in min.1..max.1+1 {
            for x in min.0..max.0+1 {
                let mut exists = false;
                for coord in cubes {
                    if coord == &(x, y, z) {
                        exists = true;
                        break;
                    }
                }
                if exists {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn distance((x1, y1, z1): &(i64, i64, i64), (x2, y2, z2): &(i64, i64, i64)) -> i64 {
    *[(x1 - x2).abs(), (y1 - y2).abs(), (z1 - z2).abs()].iter().max().unwrap()
}

fn nr_neighbors(cubes: &Vec<(i64, i64, i64)>, coord: &(i64, i64, i64)) -> u32 {
    cubes.iter().fold(0, |acc, e| {
        if distance(coord, e) == 1 {
            acc + 1
        } else {
            acc
        }
    })
}

fn step(cubes: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64, i64)> {
    let mut next_grid = Vec::new();
    let max = max_coordinate(cubes);
    let min = min_coordinate(cubes);
    for x in min.0-1..max.0+2 {
        for y in min.1-1..max.1+2 {
            for z in min.2-1..max.2+2 {
                let pos = (x, y, z);
                let occupied = match cubes.iter().try_fold(false, |_, coord| {
                    if coord == &pos {
                        None
                    } else {
                        Some(false)
                    }
                }) {
                    Some(res) => res,
                    None => true,
                };
                let nr_neighbors = nr_neighbors(cubes, &pos);
                if occupied && (nr_neighbors == 2 || nr_neighbors == 3) {
                    next_grid.push(pos);
                } else if !occupied && nr_neighbors == 3 {
                    next_grid.push(pos);
                }
            }
        }
    }
    next_grid
}

pub fn simulate(filename: &str) -> usize {
    let text = fs::read_to_string(filename).unwrap();
    let mut coords = get_coordinates(&text);
    for _ in 0..6 {
        coords = step(&coords);
    }
    coords.len()
}

fn extreme_coordinate4<F>(cubes: &Vec<(i64, i64, i64, i64)>, f: F) -> (i64, i64, i64, i64)
where F: Fn(i64, i64) -> bool {
    cubes.iter().fold((0, 0, 0, 0), |(x_acc, y_acc, z_acc, w_acc), (x, y, z, w)| {
        let mut m = (x_acc, y_acc, z_acc, w_acc);
        if f(*x, x_acc) {
            m.0 = *x;
        }
        if f(*y, y_acc) {
            m.1 = *y;
        }
        if f(*z, z_acc) {
            m.2 = *z;
        }
        if f(*w, w_acc) {
            m.3 = *w;
        }
        m
    })
}

fn max_coordinate4(cubes: &Vec<(i64, i64, i64, i64)>) -> (i64, i64, i64, i64) {
    extreme_coordinate4(cubes, |v, acc| v > acc)
}

fn min_coordinate4(cubes: &Vec<(i64, i64, i64, i64)>) -> (i64, i64, i64, i64) {
    extreme_coordinate4(cubes, |v, acc| v < acc)
}

fn get_coordinates4(text: &str) -> Vec<(i64, i64, i64, i64)> {
    text.lines().enumerate().map(|(i, line)| {
        line.chars().enumerate().filter(|(_, c)| c == &'#').map(move |(j, _)| {
            (j as i64, i as i64, 0, 0)
        })
    }).flatten().collect()
}

fn print_grid4(cubes: &Vec<(i64, i64, i64, i64)>) {
    let min = min_coordinate4(cubes);
    let max = max_coordinate4(cubes);
    for w in min.3..max.3+1 {
        for z in min.2..max.2 + 1 {
            println!("z={}, w={}", z, w);
            for y in min.1..max.1 + 1 {
                for x in min.0..max.0 + 1 {
                    let mut exists = false;
                    for coord in cubes {
                        if coord == &(x, y, z, w) {
                            exists = true;
                            break;
                        }
                    }
                    if exists {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }
}

fn distance4((x1, y1, z1, w1): &(i64, i64, i64, i64), (x2, y2, z2, w2): &(i64, i64, i64, i64)) -> i64 {
    *[(x1 - x2).abs(), (y1 - y2).abs(), (z1 - z2).abs(), (w1 - w2).abs()].iter().max().unwrap()
}

fn nr_neighbors4(cubes: &Vec<(i64, i64, i64, i64)>, coord: &(i64, i64, i64, i64)) -> u32 {
    cubes.iter().fold(0, |acc, e| {
        if distance4(coord, e) == 1 {
            acc + 1
        } else {
            acc
        }
    })
}

fn step4(cubes: &Vec<(i64, i64, i64, i64)>) -> Vec<(i64, i64, i64, i64)> {
    let mut next_grid = Vec::new();
    let max = max_coordinate4(cubes);
    let min = min_coordinate4(cubes);
    for x in min.0-1..max.0+2 {
        for y in min.1-1..max.1+2 {
            for z in min.2-1..max.2+2 {
                for w in min.3-1..max.3+2 {
                    let pos = (x, y, z, w);
                    let occupied = match cubes.iter().try_fold(false, |_, coord| {
                        if coord == &pos {
                            None
                        } else {
                            Some(false)
                        }
                    }) {
                        Some(res) => res,
                        None => true,
                    };
                    let nr_neighbors = nr_neighbors4(cubes, &pos);
                    if occupied && (nr_neighbors == 2 || nr_neighbors == 3) {
                        next_grid.push(pos);
                    } else if !occupied && nr_neighbors == 3 {
                        next_grid.push(pos);
                    }
                }
            }
        }
    }
    next_grid
}

pub fn simulate4(filename: &str) -> usize {
    let text = fs::read_to_string(filename).unwrap();
    let mut coords = get_coordinates4(&text);
    for _ in 0..6 {
        coords = step4(&coords);
    }
    coords.len()
}

#[cfg(test)]
mod tests {
    use crate::{get_coordinates, min_coordinate, max_coordinate, print_grid, nr_neighbors, step, simulate, simulate4};
    use std::fs;

    #[test]
    fn test_min_max() {
        let text = fs::read_to_string("data/example.txt").unwrap();
        let coords = get_coordinates(&text);
        assert_eq!((0, 0, 0), min_coordinate(&coords));
        assert_eq!((2, 2, 0), max_coordinate(&coords));
    }

    #[test]
    fn test_neighbors() {
        let text = fs::read_to_string("data/example.txt").unwrap();
        let coords = get_coordinates(&text);
        assert_eq!(3, nr_neighbors(&coords, &(0, 1, 0)));
    }

    #[test]
    fn test_step() {
        let text = fs::read_to_string("data/example.txt").unwrap();
        let coords = get_coordinates(&text);
        let next_grid = step(&coords);
        assert_eq!(2, nr_neighbors(&next_grid, &(0, 0, -1)));
    }

    #[test]
    fn test_simulate() {
        assert_eq!(112, simulate("data/example.txt"));
    }

    #[test]
    fn test_simulate4() {
        assert_eq!(848, simulate4("data/example.txt"));
    }

    #[test]
    fn test_simulate_task1() {
        assert_eq!(333, simulate("data/cubes.txt"));
    }

    #[test]
    fn test_simulate_task2() {
        assert_eq!(2676, simulate4("data/cubes.txt"));
    }
}
