use std::collections::{HashSet, VecDeque};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-6.txt").expect("file");
    // let data = std::fs::read_to_string("test.txt").expect("file");
    let mut gears: Vec<Vec<Component>> = Vec::new();

    let mut start = (0, 0);
    for line in data.lines() {
        let row = line.chars().map(Component::translate).collect::<Vec<_>>();
        if let Some(col) = row.iter().position(|x| x == &Component::Start) {
            start = (gears.len(), col);
        }
        gears.push(row);
    }

    let mut grid = Grid { start, gears };
    grid.rotate();
    println!("Puzzle 6, part 1 = {}", grid.calculate_light_value());
}

#[derive(Debug, PartialEq)]
enum Component {
    Start,
    Gear(Rotation),
    Light,
    Irrelevant,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Rotation {
    Still,
    Clockwise,
    CounterClockwise,
}

impl Component {
    fn translate(ch: char) -> Self {
        match ch {
            'S' => Self::Start,
            '#' => Self::Gear(Rotation::Still),
            '*' => Self::Light,
            _ => Self::Irrelevant,
        }
    }
}

struct Grid {
    start: (usize, usize),
    gears: Vec<Vec<Component>>,
}

impl Grid {
    fn rotate(&mut self) {
        let mut rotated = HashSet::new();
        let mut queue = VecDeque::from([(self.start, Rotation::CounterClockwise)]);

        self.gears[self.start.0][self.start.1] = Component::Gear(Rotation::CounterClockwise);

        while let Some((pos, rot)) = queue.pop_front() {
            let opposite = match rot {
                Rotation::Still => panic!("prev gear at {pos:?} didn't move"),
                Rotation::Clockwise => Rotation::CounterClockwise,
                Rotation::CounterClockwise => Rotation::Clockwise,
            };
            if rotated.insert(pos) {
                let mut check = Vec::new();
                if pos.0 > 0 {
                    check.push((pos.0 - 1, pos.1));
                }
                if pos.1 > 0 {
                    check.push((pos.0, pos.1 - 1));
                }
                if pos.0 < self.gears.len() - 1 {
                    check.push((pos.0 + 1, pos.1));
                }
                if pos.1 < self.gears[0].len() - 1 {
                    check.push((pos.0, pos.1 + 1));
                }

                for loc in check {
                    if matches!(self.gears[loc.0][loc.1], Component::Gear(_)) {
                        self.gears[loc.0][loc.1] = Component::Gear(opposite);
                        queue.push_back((loc, opposite));
                    }
                }
            }
        }
    }

    fn calculate_light_value(&self) -> u128 {
        let mut result = 0;

        for (r, row) in self.gears.iter().enumerate() {
            for (c, comp) in row.iter().enumerate() {
                if matches!(comp, Component::Light) {
                    let pos = (r, c);

                    let mut check = Vec::new();
                    if pos.0 > 0 {
                        check.push((pos.0 - 1, pos.1));
                    }
                    if pos.1 > 0 {
                        check.push((pos.0, pos.1 - 1));
                    }
                    if pos.0 < self.gears.len() - 1 {
                        check.push((pos.0 + 1, pos.1));
                    }
                    if pos.1 < self.gears[0].len() - 1 {
                        check.push((pos.0, pos.1 + 1));
                    }

                    for gear in check {
                        match self.gears[gear.0][gear.1] {
                            Component::Gear(Rotation::Clockwise) => result = (result << 1) | 1,
                            Component::Gear(Rotation::CounterClockwise) => result <<= 1,
                            _ => {}
                        }
                    }
                }
            }
        }

        result
    }

    fn _print(&self) {
        for row in &self.gears {
            for comp in row {
                print!(
                    "{}",
                    match comp {
                        Component::Start => 'S',
                        Component::Gear(rotation) => match rotation {
                            Rotation::Still => '#',
                            Rotation::Clockwise => 'R',
                            Rotation::CounterClockwise => 'L',
                        },
                        Component::Light => '*',
                        Component::Irrelevant => '.',
                    }
                );
            }
            println!();
        }
    }
}
