use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, AddAssign},
};

pub fn run() {
    // let data = std::fs::read_to_string("test.txt").expect("file");
    let data = std::fs::read_to_string("input/puzzle-9.txt").expect("file");

    let mut maze = HashSet::<Point>::new();
    let mut start = Point(0, 0);
    let mut end = Point(0, 0);

    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    start = Point(row, col);
                }
                'E' => {
                    end = Point(row, col);
                }
                '.' => {}
                '#' => {
                    maze.insert(Point(row, col));
                }
                _ => panic!("bug in your code"),
            }
        }
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((p, count)) = queue.pop_front() {
        if p == end {
            println!("Puzzle 9, part 1 = {count}");
            break;
        }
        if visited.insert(p) {
            for neighbor in p.neighbors() {
                if maze.contains(&neighbor) {
                    continue;
                }
                if !visited.contains(&neighbor) {
                    queue.push_back((neighbor, count + 1));
                }
            }
        }
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((p, count)) = queue.pop_front() {
        if p == end {
            println!("Puzzle 9, part 1 = {count}");
            break;
        }
        if visited.insert(p) {
            for neighbor in p.neighbors().into_iter().chain(full_dist(&maze, &p)) {
                if maze.contains(&neighbor) {
                    continue;
                }
                if !visited.contains(&neighbor) {
                    queue.push_back((neighbor, count + 1));
                }
            }
        }
    }
}

fn full_dist(maze: &HashSet<Point>, p: &Point) -> Vec<Point> {
    let mut result = Vec::new();

    for dir in &DIRS {
        let mut start = *p;
        while !maze.contains(&(start + *dir)) {
            start += *dir;
        }
        if start != *p {
            result.push(start);
        }
    }

    result
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point(usize, usize);

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        vec![
            Point(self.0 - 1, self.1),
            Point(self.0 + 1, self.1),
            Point(self.0, self.1 - 1),
            Point(self.0, self.1 + 1),
        ]
    }
}

const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        *self = match rhs {
            Direction::Up => Point(self.0 - 1, self.1),
            Direction::Down => Point(self.0 + 1, self.1),
            Direction::Left => Point(self.0, self.1 - 1),
            Direction::Right => Point(self.0, self.1 + 1),
        };
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Point(self.0 - 1, self.1),
            Direction::Down => Point(self.0 + 1, self.1),
            Direction::Left => Point(self.0, self.1 - 1),
            Direction::Right => Point(self.0, self.1 + 1),
        }
    }
}
