// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Debug)]
pub struct Robot {
    position: (isize, isize),
    direction: Direction,
}

impl<'a> Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            position: self.position,
            direction: match self.direction {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            position: self.position,
            direction: match self.direction {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
        }
    }

    pub fn advance(self) -> Self {
        let new_position = match self.direction {
            Direction::North => (self.position.0, self.position.1 + 1),
            Direction::South => (self.position.0, self.position.1 - 1),
            Direction::East => (self.position.0 + 1, self.position.1),
            Direction::West => (self.position.0 - 1, self.position.1),
        };

        Robot {
            position: new_position,
            direction: self.direction,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for instr in instructions.chars() {
            robot = match instr {
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => panic!("Unsupported direction!"),
            };
        }
        robot

    }

    pub fn position(&self) -> (isize, isize) {
        self.position
    }

    pub fn direction(&'a self) -> &'a Direction {
        &self.direction
    }
}
   