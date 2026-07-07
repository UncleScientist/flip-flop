pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-2.txt").expect("missing file");
    let instructions = data.chars().map(Direction::get);
    let mut wall = Wall::default();

    wall.apply(instructions);
    println!("Puzzle 2, part 1 = {}", wall.part1());
}

#[derive(Debug)]
struct Wall {
    segments: [usize; 100],
}

impl Default for Wall {
    fn default() -> Self {
        Self::new()
    }
}

impl Wall {
    fn new() -> Self {
        Self {
            segments: [0usize; 100],
        }
    }

    fn apply(&mut self, instructions: impl Iterator<Item = Direction>) {
        let mut pos = 0;
        for dir in instructions {
            match dir {
                Direction::Left => pos = (pos + 99) % 100,
                Direction::Right => pos = (pos + 1) % 100,
            }
            self.segments[pos] += 1;
        }
    }

    fn part1(&self) -> usize {
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
