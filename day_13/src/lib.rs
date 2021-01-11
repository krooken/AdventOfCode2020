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

fn least_common_multiple(a: u64, b: u64) -> u64 {
    (a/gcd(a,b)) * b
}

fn find_earliest_for_offset(start: u64, step: (u64, u64), offset: u64) -> u64 {
    let mut a = start;
    let mut b = 0u64;
    while a + offset != b {
        if b < a + offset {
            b += step.1;
        } else {
            a += step.0;
        }
    }
    if a - start > least_common_multiple(step.0, step.1) {
        panic!();
    }
    a
}

fn find_earliest_departure(departures: &Vec<(u64, u64)>) -> u64 {
    let mut departure_a = departures[0].0;
    let mut step = departure_a;
    for (departure_b, offset_b) in departures.iter().skip(1) {
        departure_a = find_earliest_for_offset(departure_a, (step, *departure_b), *offset_b);
        step = least_common_multiple(step, *departure_b);
    }
    departure_a
}

pub fn calculate_earliest_departure(filename: &str) -> u64 {
    let text = fs::read_to_string(filename).unwrap();
    let mut rows = text.lines();
    rows.next().unwrap();
    let mut timetable: Vec<(u64, u64)> = Vec::new();
    for (offset, time) in rows.next().unwrap().split(",").enumerate() {
        match time.parse() {
            Ok(num) => timetable.push((num, offset as u64)),
            _ => (),
        }
    }
    find_earliest_departure(&timetable)
}

#[cfg(test)]
mod tests {
    use crate::{wait_in_minutes, shortest_wait, calculate_product, gcd, least_common_multiple, find_earliest_for_offset, find_earliest_departure, calculate_earliest_departure};

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

    #[test]
    fn test_lcm_1() {
        assert_eq!(12, least_common_multiple(3, 4));
    }

    #[test]
    fn test_lcm_2() {
        assert_eq!(15, least_common_multiple(5, 15));
    }

    #[test]
    fn test_lcm_3() {
        assert_eq!(30, least_common_multiple(6, 10));
    }

    #[test]
    fn test_find_earliest_for_offset_1() {
        assert_eq!(2, find_earliest_for_offset(0, (1, 3), 1));
    }

    #[test]
    fn test_find_earliest_for_offset_2() {
        assert_eq!(1, find_earliest_for_offset(0, (1, 3), 2));
    }

    #[test]
    fn test_find_earliest_for_offset_3() {
        assert_eq!(0, find_earliest_for_offset(0, (1, 3), 3));
    }

    #[test]
    fn test_find_earliest_for_offset_4() {
        assert_eq!(9, find_earliest_for_offset(0, (3, 5), 1));
    }

    #[test]
    fn test_find_earliest_for_offset_5() {
        assert_eq!(54, find_earliest_for_offset(9, (15, 7), 2));
    }

    #[test]
    fn test_find_earliest_departure_1() {
        let v = vec![(3, 0), (5, 1), (7, 2)];
        assert_eq!(54, find_earliest_departure(&v));
    }

    #[test]
    fn test_find_earliest_departure_2() {
        let v = vec![(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)];
        assert_eq!(1_068_781, find_earliest_departure(&v));
    }

    #[test]
    fn test_find_earliest_departure_3() {
        let v = vec![(17, 0), (13, 2), (19, 3)];
        assert_eq!(3417, find_earliest_departure(&v));
    }

    #[test]
    fn test_find_earliest_departure_4() {
        let v = vec![(67, 0), (7, 1), (59, 2), (61, 3)];
        assert_eq!(754_018, find_earliest_departure(&v));
    }

    #[test]
    fn test_find_earliest_departure_5() {
        let v = vec![(67, 0), (7, 2), (59, 3), (61, 4)];
        assert_eq!(779_210, find_earliest_departure(&v));
    }

    #[test]
    fn test_find_earliest_departure_6() {
        let v = vec![(67, 0), (7, 1), (59, 3), (61, 4)];
        assert_eq!(1_261_476, find_earliest_departure(&v));
    }

    #[test]
    fn test_find_earliest_departure_7() {
        let v = vec![(1789, 0), (37, 1), (47, 2), (1889, 3)];
        assert_eq!(1_202_161_486, find_earliest_departure(&v));
    }

    #[test]
    fn test_calculate_earliest_departure() {
        assert_eq!(1_068_781, calculate_earliest_departure("data/example.txt"));
    }
}
