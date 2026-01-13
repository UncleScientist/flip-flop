use std::{convert::Infallible, str::FromStr};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-6.txt").expect("file");
    let birdspeeds: Vec<Speed> = data.lines().map(|line| line.parse().unwrap()).collect();

    println!(
        "Puzzle 6, part 1 = {}",
        birdspeeds
            .iter()
            .map(|speed| ZERO.location_at_time(speed, 100))
            .filter(|bird| bird.within(250, 250, 750, 750))
            .count()
    );
}

#[derive(Debug)]
struct Speed {
    x: isize,
    y: isize,
}

impl FromStr for Speed {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (left, right) = line.split_once(',').unwrap();
        Ok(Self {
            x: left.parse().unwrap(),
            y: right.parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Loc {
    x: isize,
    y: isize,
}

const ZERO: Loc = Loc { x: 0, y: 0 };

impl Loc {
    fn location_at_time(&self, speed: &Speed, t: isize) -> Self {
        Self {
            x: (speed.x * t).rem_euclid(1000),
            y: (speed.y * t).rem_euclid(1000),
        }
    }

    fn within(&self, x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
        self.x >= x1 && self.x < x2 && self.y >= y1 && self.y < y2
    }
}
