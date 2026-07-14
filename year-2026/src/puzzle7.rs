use std::{convert::Infallible, ops::AddAssign, str::FromStr};

pub fn run() {
    // let data = std::fs::read_to_string("test.txt").expect("file");
    let data = std::fs::read_to_string("input/puzzle-7.txt").expect("file");

    let mut iter = data.lines();
    let movement_instructions = iter.next().unwrap();
    let instructions = movement_instructions
        .chars()
        .map(Movement::get)
        .collect::<Vec<_>>();

    iter.next(); // skip blank line

    let sushi = iter
        .map(|p| p.parse::<Point>().unwrap())
        .collect::<Vec<_>>();

    let game = SnakeGame {
        instructions,
        sushi,
    };
    println!("Puzzle 7, part 1 = {}", game.solve_part_1());
}

#[derive(Debug)]
struct SnakeGame {
    instructions: Vec<Movement>,
    sushi: Vec<Point>,
}

impl SnakeGame {
    fn solve_part_1(&self) -> usize {
        let mut sushi_eaten = 0;

        let mut snake = Point::default();

        let half = self.instructions.len() / 2;
        for movement in self.instructions.iter().take(half) {
            snake += *movement;
            if snake == self.sushi[sushi_eaten] {
                sushi_eaten += 1;
            }
        }

        sushi_eaten
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl AddAssign<Movement> for Point {
    fn add_assign(&mut self, rhs: Movement) {
        *self = match rhs {
            Movement::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
            Movement::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Movement::Up => Point {
                x: self.x,
                y: self.y + 1,
            },
            Movement::Down => Point {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Movement {
    Right,
    Left,
    Up,
    Down,
}

impl Movement {
    fn get(ch: char) -> Self {
        match ch {
            '>' => Self::Right,
            '<' => Self::Left,
            '^' => Self::Up,
            'v' => Self::Down,
            _ => panic!("Invalid input {ch}"),
        }
    }
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (left, right) = line.split_once(',').unwrap();
        Ok(Self {
            x: left.parse().unwrap(),
            y: right.parse().unwrap(),
        })
    }
}
