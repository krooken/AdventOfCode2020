use std::fs;

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

fn get_commands(text: &str) -> Vec<Command> {
    let re = regex::Regex::new(r"(^E|N|W|S|F|R|L)(\d+)$").unwrap();
    text.lines().map(|line| {
        let cap = re.captures_iter(line).next().unwrap();
        let num: i64 = cap[2].parse().unwrap();
        match &cap[1] {
            "E" => Command::East(num),
            "N" => Command::North(num),
            "W" => Command::West(num),
            "S" => Command::South(num),
            "F" => Command::Forward(num),
            "R" => Command::Right(num),
            "L" => Command::Left(num),
            _ => panic!(),
        }
    }).collect()
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Command {
    East(i64),
    North(i64),
    West(i64),
    South(i64),
    Forward(i64),
    Right(i64),
    Left(i64),
}

pub fn sail_to_destination(filename: &str) -> i64 {
    let text = fs::read_to_string(filename).unwrap();
    let commands = get_commands(&text);
    let mut pos = Position::new();
    for command in commands.iter() {
        pos.sail(*command);
    }
    pos.manhattan()
}

#[cfg(test)]
mod tests {
    use crate::{Position, Command, get_commands, sail_to_destination};
    use std::fs;

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

    #[test]
    fn test_get_commands() {
        let text = fs::read_to_string("data/example.txt").unwrap();
        let commands = get_commands(&text);
        assert_eq!(Command::Forward(10), commands[0]);
        assert_eq!(Command::Right(90), commands[3]);
    }

    #[test]
    fn test_destination() {
        assert_eq!(25, sail_to_destination("data/example.txt"));
    }
}
