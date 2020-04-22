// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(PartialEq, Debug, Clone,  Copy)]
pub struct Point {
    x: i32,
    y: i32
}

#[derive(PartialEq, Debug, Clone,  Copy)]
pub struct Robot {
    start: Point,
    current_position: Point,
    current_direction: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            start: Point {x: x, y: y},
            current_direction: d,
            current_position: Point {x:x, y:y}
        }
    }

    pub fn turn_right(self) -> Self {
        let direction = match self.current_direction {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South
        };
        Robot {
            start: self.start,
            current_position: self.current_position,
            current_direction: direction
        }
    }

    pub fn turn_left(self) -> Self {
        let direction = match self.current_direction {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North
        };
        Robot {
            start: self.start,
            current_position: self.current_position,
            current_direction: direction
        }
    }

    pub fn advance(self) -> Self {
        let new_position:(i32, i32) = match self.current_direction {
            Direction::North => (self.current_position.x, self.current_position.y + 1),
            Direction::South => (self.current_position.x, self.current_position.y - 1),
            Direction::East => (self.current_position.x + 1, self.current_position.y),
            Direction::West => (self.current_position.x - 1, self.current_position.y)
        };

        Robot {
            start: self.start,
            current_position: Point {
                x: new_position.0,
                y: new_position.1
            },
            current_direction: self.current_direction
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut temp:Robot = self.clone();

        let _ignored_input = instructions.chars().map(|x| {
            temp = match x {
                'R' => temp.turn_right(),
                'L' => temp.turn_left(),
                'A' => temp.advance(),
                _ => temp
            }
        }).collect::<Vec<_>>();

        temp 
    }

    pub fn position(&self) -> (i32, i32) {
        (self.current_position.x, self.current_position.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.current_direction
    }
}
