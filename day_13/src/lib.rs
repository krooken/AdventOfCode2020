use std::fs;

fn wait_in_minutes(bus_id: u32, departure_time: u32) -> u32 {
    (bus_id - departure_time % bus_id) % bus_id
}

fn shortest_wait(timetable: &Vec<u32>, departure_time: u32) -> (u32, u32) {
    timetable.iter().fold((timetable[0], timetable[0]), |(acc, id), e| {
        let wait = wait_in_minutes(*e, departure_time);
        if wait < acc {(wait, *e)} else {(acc, id)}
    })
}

pub fn calculate_product(filename: &str) -> u32 {
    let text = fs::read_to_string(filename).unwrap();
    let mut rows = text.lines();
    let departure: u32 = rows.next().unwrap().parse().unwrap();
    let mut timetable: Vec<u32> = Vec::new();
    for time in rows.next().unwrap().split(",") {
        match time.parse() {
            Ok(num) => timetable.push(num),
            _ => (),
        }
    }
    let (wait, id) = shortest_wait(&timetable, departure);
    wait * id
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    if a < b {
        let tmp_a = a;
        a = b;
        b = tmp_a;
    }
    while b != 0 {
        let tmp_b = a % b;
        a = b;
        b = tmp_b;
    }
    a
}

#[cfg(test)]
mod tests {
    use crate::{wait_in_minutes, shortest_wait, calculate_product, gcd};

    #[test]
    fn test_wait_in_minutes_zero() {
        assert_eq!(0, wait_in_minutes(13, 13));
    }

    #[test]
    fn test_wait_in_minutes_1() {
        assert_eq!(1, wait_in_minutes(13, 12));
    }

    #[test]
    fn test_wait_in_minutes_2() {
        assert_eq!(1, wait_in_minutes(13, 25));
    }

    #[test]
    fn test_earliest_departure() {
        assert_eq!(5, shortest_wait(&vec![7, 13, 59, 31, 19], 939).0);
    }

    #[test]
    fn test_calculate_product() {
        assert_eq!(295, calculate_product("data/example.txt"));
    }

    #[test]
    fn test_gcd_1() {
        assert_eq!(2, gcd(2,4));
    }

    #[test]
    fn test_gcd_2() {
        assert_eq!(1, gcd(5,7));
    }
}
