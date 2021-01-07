fn wait_in_minutes(bus_id: u32, departure_time: u32) -> u32 {
    (bus_id - departure_time % bus_id) % bus_id
}

#[cfg(test)]
mod tests {
    use crate::wait_in_minutes;

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
}
