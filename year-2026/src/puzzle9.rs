use std::collections::{HashSet, VecDeque};

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
