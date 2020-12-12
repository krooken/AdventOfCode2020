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

    fn left(&mut self) {
        self.orientation = match &self.orientation {
            Orientation::East => Orientation::North,
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
        }
    }

    fn right(&mut self) {
        self.orientation = match &self.orientation {
            Orientation::East => Orientation::South,
            Orientation::North => Orientation::East,
            Orientation::West => Orientation::North,
            Orientation::South => Orientation::West,
        }
    }

    fn sail(&mut self, command: Command) {
        let translated_command = match command {
            Command::Forward(num) => {match self.orientation {
                Orientation::East => Command::East(num),
                Orientation::North => Command::North(num),
                Orientation::West => Command::West(num),
                Orientation::South => Command::South(num),
            }},
            _ => command,
        };
        match translated_command {
            Command::East(num) => self.east += num,
            Command::North(num) => self.north += num,
            Command::West(num) => self.east -= num,
            Command::South(num) => self.north -= num,
            Command::Right(num) => {
                for _ in 0..num/90 {
                    self.right();
                }
            },
            Command::Left(num) => {
                for _ in 0..num/90 {
                    self.left();
                }
            }
            _ => panic!(),
        }
    }
}

enum Command {
    East(i64),
    North(i64),
    West(i64),
    South(i64),
    Forward(i64),
    Right(i64),
    Left(i64),
}

#[cfg(test)]
mod tests {
    use crate::{Position, Command};

    #[test]
    fn test_manhattan() {
        let mut pos = Position::new();
        pos.north += 10;
        pos.east += -5;
        assert_eq!(15, pos.manhattan());
    }

    #[test]
    fn test_sail() {
        let mut pos = Position::new();
        pos.sail(Command::Forward(10));
        pos.sail(Command::North(3));
        pos.sail(Command::Forward(7));
        pos.sail(Command::Right(90));
        pos.sail(Command::Forward(11));
        assert_eq!(25, pos.manhattan());
    }
}
