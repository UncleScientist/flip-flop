pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-2.txt").expect("missing file");
    let mut wall = Wall::new();

    wall.do_part_1(&data);
    println!("Puzzle 2, part 1 = {}", wall.part1_score());

    println!("Puzzle 2, part 1 = {}", wall.do_part_2(&data));

    wall.reset();
    wall.do_part_3(&data);
    println!("Puzzle 2, part 1 = {}", wall.part1_score());
}

#[derive(Debug)]
struct Wall {
    segments: [usize; 100],
}

impl Wall {
    fn new() -> Self {
        Self {
            segments: [0usize; 100],
        }
    }

    fn do_part_1(&mut self, data: &str) {
        let instructions = data.chars().map(Direction::get);
        let mut pos = 0;
        for dir in instructions {
            match dir {
                Direction::Left => pos = (pos + 99) % 100,
                Direction::Right => pos = (pos + 1) % 100,
            }
            self.segments[pos] += 1;
        }
    }

    fn part1_score(&self) -> usize {
        let Some((index, max_val)) = self
            .segments
            .iter()
            .enumerate()
            .rev()
            .max_by(|a, b| a.1.cmp(b.1))
        else {
            panic!("max not found");
        };
        (index + 1) * max_val
    }

    fn do_part_2(&mut self, instructions: &str) -> usize {
        let mut laser_pos = 0;
        let mut robot_pos = 0;

        let forward = instructions.chars().map(Direction::get);
        let reverse = instructions.chars().rev().map(Direction::get);

        let mut count = 0;
        for (laser, robot) in forward.zip(reverse) {
            match laser {
                Direction::Left => laser_pos = (laser_pos + 99) % 100,
                Direction::Right => laser_pos = (laser_pos + 1) % 100,
            }
            match robot {
                Direction::Left => robot_pos = (robot_pos + 99) % 100,
                Direction::Right => robot_pos = (robot_pos + 1) % 100,
            }
            count += (laser_pos == robot_pos) as usize;
        }

        count
    }

    fn reset(&mut self) {
        self.segments = [0; 100];
    }

    fn do_part_3(&mut self, data: &str) {
        let forward = data.chars().map(Direction::get);
        let reverse = data.chars().rev().map(Direction::get);

        let mut laser_pos = 0;
        for (forward_step, reverse_step) in forward.zip(reverse) {
            match forward_step {
                Direction::Left => laser_pos = (laser_pos + 99) % 100,
                Direction::Right => laser_pos = (laser_pos + 1) % 100,
            }
            match reverse_step {
                Direction::Left => laser_pos = (laser_pos + 1) % 100,
                Direction::Right => laser_pos = (laser_pos + 99) % 100,
            }
            self.segments[laser_pos] += 1;
        }
    }
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn get(which: char) -> Self {
        match which {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("invalid direction '{which}'"),
        }
    }
}
