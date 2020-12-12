enum Orientation {
    East,
    North,
    West,
    South,
}

struct Position {
    north: i64,
    east: i64,
    orientation: Orientation,
}

impl Position {
    fn new() -> Position {
        Position {
            north: 0,
            east: 0,
            orientation: Orientation::East,
        }
    }

    fn manhattan(&self) -> i64 {
        self.east.abs() + self.north.abs()
    }
}

#[cfg(test)]
mod tests {
    use crate::Position;

    #[test]
    fn test_manhattan() {
        let mut pos = Position::new();
        pos.north += 10;
        pos.east += -5;
        assert_eq!(15, pos.manhattan());
    }
}
