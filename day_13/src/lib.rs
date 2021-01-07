fn wait_in_minutes(bus_id: u32, departure_time: u32) -> u32 {
    (bus_id - departure_time % bus_id) % bus_id
}

fn shortest_wait(timetable: &Vec<u32>, departure_time: u32) -> u32 {
    timetable.iter().fold(timetable[0], |acc, e| {
        let wait = wait_in_minutes(*e, departure_time);
        if wait < acc {wait} else {acc}
    })
}

#[cfg(test)]
mod tests {
    use crate::{wait_in_minutes, shortest_wait};

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
        assert_eq!(5, shortest_wait(&vec![7, 13, 59, 31, 19], 939));
    }
}
