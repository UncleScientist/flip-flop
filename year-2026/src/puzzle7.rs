use std::{collections::VecDeque, convert::Infallible, ops::AddAssign, str::FromStr};

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
    println!("Puzzle 7, part 2 = {}", game.solve_part_2());
    println!("Puzzle 7, part 3 = {}", game.solve_part_3());
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

    fn solve_part_2(&self) -> usize {
        let mut snake = VecDeque::from([Point::default()]);
        let mut head_loc = Point::default();
        let mut sushi_eaten = 0;

        'out: for movement in &self.instructions {
            head_loc += *movement;

            if head_loc == self.sushi[sushi_eaten] {
                sushi_eaten += 1;
            } else {
                snake.pop_back();
            }

            for pos in &snake {
                if *pos == head_loc {
                    break 'out;
                }
            }

            snake.push_front(head_loc);
        }

        snake.len() + 1
    }

    fn solve_part_3(&self) -> usize {
        let mut snake = VecDeque::from([Point::default()]);
        let mut head_loc = Point::default();
        let mut sushi_eaten = 0;
        let mut self_eat_count = 0;

        for movement in &self.instructions {
            head_loc += *movement;

            if sushi_eaten < self.sushi.len() && head_loc == self.sushi[sushi_eaten] {
                sushi_eaten += 1;
            } else {
                snake.pop_back();
            }

            for (idx, pos) in snake.iter().enumerate() {
                if *pos == head_loc {
                    self_eat_count += 1;
                    snake.truncate(idx - 1);
                    break;
                }
            }

            snake.push_front(head_loc);
        }

        snake.len() * self_eat_count
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
