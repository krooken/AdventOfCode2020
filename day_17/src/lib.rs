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

#[cfg(test)]
mod tests {
    use crate::{get_coordinates, min_coordinate, max_coordinate, print_grid};
    use std::fs;

    #[test]
    fn test_min_max() {
        let text = fs::read_to_string("data/example.txt").unwrap();
        let coords = get_coordinates(&text);
        assert_eq!((0, 0, 0), min_coordinate(&coords));
        assert_eq!((2, 2, 0), max_coordinate(&coords));
    }
}
