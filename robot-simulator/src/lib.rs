// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
pub struct Robot {
    direction: Direction,
    position: (i32, i32),
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            direction: d,
            position: (x, y),
        }
    }

    pub fn turn_right(self) -> Self {
        use Direction::*;
        let mut robot = Robot::new(self.position.0, self.position.1, self.direction);
        match self.direction {
            North => robot.direction = East,
            East => robot.direction = South,
            South => robot.direction = West,
            West => robot.direction = North,
        }
        robot
    }

    pub fn turn_left(self) -> Self {
        use Direction::*;
        let mut robot = Robot::new(self.position.0, self.position.1, self.direction);
        match self.direction {
            North => robot.direction = West,
            South => robot.direction = East,
            East => robot.direction = North,
            West => robot.direction = South,
        }
        robot
    }

    pub fn advance(self) -> Self {
        use Direction::*;
        let mut robot = Robot::new(self.position.0, self.position.1, self.direction);
        match self.direction {
            North => robot.position.1 += 1,
            South => robot.position.1 -= 1,
            East => robot.position.0 += 1,
            West => robot.position.0 -= 1,
        }
        robot
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self.clone();
        instructions.chars().for_each(|c| match c {
            'A' => robot = robot.advance(),
            'L' => robot = robot.turn_left(),
            'R' => robot = robot.turn_right(),
            _ => panic!(),
        });
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
