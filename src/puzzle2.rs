pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-2.txt").expect("missing file");

    let mut height = 0;
    let mut max_height = 0;
    for ch in data.lines().next().unwrap().chars() {
        match ch {
            '^' => height += 1,
            'v' => height -= 1,
            _ => panic!("illegal input: '{ch}'"),
        }
        max_height = max_height.max(height);
    }
    println!("Puzzle 2, part 1 = {max_height}");

    let mut height = 0;
    let mut max_height = 0;
    let mut cur_steepness = 1;
    let mut prev_direction = '.';
    for ch in data.lines().next().unwrap().chars() {
        match ch {
            '^' => {
                if prev_direction != '^' {
                    cur_steepness = 1;
                    prev_direction = ch;
                }
                height += cur_steepness;
                cur_steepness += 1;
            }
            'v' => {
                if prev_direction != 'v' {
                    cur_steepness = 1;
                    prev_direction = ch;
                }
                height -= cur_steepness;
                cur_steepness += 1;
            }
            _ => panic!("illegal input: '{ch}'"),
        }
        max_height = max_height.max(height);
    }
    println!("Puzzle 2, part 2 = {max_height}");

    let mut height = 0;
    let mut max_height = 0;
    let mut motion_count = 0;
    let mut prev_direction = Direction::Start;
    let mut fibcache = Fibber::new();
    for ch in data.lines().next().unwrap().chars() {
        // for ch in "^^^^^^^^^^^^vvvvvvvvv^".chars() {
        let cur_direction: Direction = ch.into();
        match (prev_direction, cur_direction) {
            (Direction::Start, _) => {
                motion_count = 1;
            }
            (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => {
                motion_count += 1;
            }
            (Direction::Up, Direction::Down) => {
                height += fibcache.get(motion_count);
                motion_count = 1;
            }
            (Direction::Down, Direction::Up) => {
                height -= fibcache.get(motion_count);
                motion_count = 1;
            }
            _ => panic!("invalid direction pair {prev_direction:?} -> {cur_direction:?}"),
        }
        prev_direction = cur_direction;
        max_height = max_height.max(height);
    }
    println!("Puzzle 2, part 3 = {max_height}");
}

struct Fibber {
    cache: Vec<i32>,
}

impl Fibber {
    fn new() -> Self {
        Self {
            cache: vec![0, 1, 1, 2, 3, 5],
        }
    }

    fn get(&mut self, value: i32) -> i32 {
        while self.cache.len() as i32 <= value {
            let l = self.cache.len();
            self.cache.push(self.cache[l - 1] + self.cache[l - 2]);
        }
        self.cache[value as usize]
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Start,
    Up,
    Down,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            _ => panic!("invalid direction '{value}'"),
        }
    }
}
