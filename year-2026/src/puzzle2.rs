pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-2.txt").expect("missing file");
    let instructions = data
        .chars()
        .map(|ch| Direction::get(ch))
        .collect::<Vec<_>>();
    let mut wall = Wall::default();

    wall.apply(&instructions);

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

    fn apply(&mut self, instructions: &[Direction]) {
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
        let mut max = 0;
        let mut pos = 0;
        for (idx, val) in self.segments.iter().enumerate() {
            if *val > max {
                max = *val;
                pos = idx + 1;
            }
        }
        pos * max
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
