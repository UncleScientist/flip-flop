use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    ops::{Add, AddAssign},
};

pub fn run() {
    // let data = std::fs::read_to_string("small-maze.txt").expect("file");
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
            println!("Puzzle 9, part 2 = {count}");
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

    let mut visited = HashSet::new();
    let mut queue =
        VecDeque::<(Point, PortalPair, usize)>::from([(start, PortalPair::default(), 0)]);

    while let Some((cur_loc, PortalPair { orange, blue }, count)) = queue.pop_front() {
        if cur_loc == end {
            println!("Puzzle 9, part 3 = {count}");
            break;
        }
        if visited.insert((cur_loc, PortalPair { orange, blue })) {
            let actions = DIRS
                .iter()
                .map(|d| Action::Walk(*d))
                .chain(DIRS.iter().map(|d| Action::ShootOrange(*d)))
                .chain(DIRS.iter().map(|d| Action::ShootBlue(*d)));

            for action in actions {
                let mut orange = orange;
                let mut blue = blue;

                let neighbor = match action {
                    Action::Walk(dir) => {
                        let check = cur_loc + dir;
                        if maze.contains(&check) {
                            if orange == Some((dir, cur_loc)) {
                                if let Some((_, blue_loc)) = blue {
                                    Some(blue_loc)
                                } else {
                                    None
                                }
                            } else if blue == Some((dir, cur_loc)) {
                                if let Some((_, orange_loc)) = orange {
                                    Some(orange_loc)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            orange = None;
                            blue = None;
                            Some(check)
                        }
                    }
                    Action::ShootOrange(dir) => {
                        let end = look(&maze, &cur_loc, &dir);
                        if orange != Some((dir, end)) {
                            orange = Some((dir, end));
                            Some(cur_loc)
                        } else {
                            None
                        }
                    }
                    Action::ShootBlue(dir) => {
                        let end = look(&maze, &cur_loc, &dir);
                        if blue != Some((dir, end)) {
                            blue = Some((dir, end));
                            Some(cur_loc)
                        } else {
                            None
                        }
                    }
                };

                if let Some(neighbor) = neighbor {
                    if visited.contains(&(neighbor, PortalPair { orange, blue })) {
                        continue;
                    }
                    if let Some(o) = orange
                        && let Some(b) = blue
                        && o.1 == b.1
                    {
                        continue;
                    }

                    if maze.contains(&neighbor) {
                        continue;
                    }
                    queue.push_back((neighbor, PortalPair { orange, blue }, count + 1));
                }
            }
        }
    }
}

#[derive(Debug, Default, Eq)]
struct PortalPair {
    orange: Option<(Direction, Point)>,
    blue: Option<(Direction, Point)>,
}

impl PartialEq for PortalPair {
    fn eq(&self, other: &Self) -> bool {
        let this_opoint = self.orange.map(|o| o.1);
        let this_bpoint = self.blue.map(|b| b.1);
        let that_opoint = other.orange.map(|o| o.1);
        let that_bpoint = other.blue.map(|b| b.1);

        (this_opoint == that_opoint && this_bpoint == that_bpoint)
            || (this_opoint == that_bpoint && this_bpoint == that_opoint)
    }
}

impl Hash for PortalPair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(orange) = self.orange {
            if let Some(blue) = self.blue {
                if (orange.1.0 < blue.1.0) || (orange.1.0 == blue.1.0) && (orange.1.1 < blue.1.1) {
                    orange.1.hash(state);
                    blue.1.hash(state);
                } else {
                    blue.1.hash(state);
                    orange.1.hash(state);
                }
            } else {
                orange.1.hash(state);
                self.blue.hash(state);
            }
        } else if let Some(blue) = self.blue {
            blue.1.hash(state);
            self.orange.hash(state);
        } else {
            self.orange.hash(state);
            self.blue.hash(state);
        }
    }
}

#[derive(Debug)]
enum Action {
    Walk(Direction),
    ShootOrange(Direction),
    ShootBlue(Direction),
}

fn look(maze: &HashSet<Point>, p: &Point, dir: &Direction) -> Point {
    let mut start = *p;
    while !maze.contains(&(start + *dir)) {
        start += *dir;
    }
    start
}

fn full_dist(maze: &HashSet<Point>, p: &Point) -> Vec<Point> {
    DIRS.iter()
        .map(|dir| look(maze, p, dir))
        .filter(|new_point| new_point != p)
        .collect()
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
