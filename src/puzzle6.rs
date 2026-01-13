use std::{convert::Infallible, str::FromStr};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-6.txt").expect("file");
    let birdspeeds: Vec<Speed> = data.lines().map(|line| line.parse().unwrap()).collect();

    println!(
        "Puzzle 6, part 1 = {}",
        birdspeeds
            .iter()
            .map(|speed| ZERO.location_at_time(speed, 100))
            .filter(|bird| bird.in_frame())
            .count()
    );

    println!(
        "Puzzle 6, part 2 = {}",
        (1..=1000)
            .map(|time| birdspeeds
                .iter()
                .map(|speed| ZERO.location_at_time(speed, time * 3600))
                .filter(|bird| bird.in_frame())
                .count())
            .sum::<usize>()
    );

    println!(
        "Puzzle 6, part 3 = {}",
        (1..=1000)
            .map(|time| birdspeeds
                .iter()
                .map(|speed| ZERO.location_at_time(speed, time * 31556926))
                .filter(|bird| bird.in_frame())
                .count())
            .sum::<usize>()
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
struct Loc<const SIZE: isize> {
    x: isize,
    y: isize,
}

const ZERO: Loc<1000> = Loc { x: 0, y: 0 };

impl<const SIZE: isize> Loc<SIZE> {
    const UPPER_LEFT: isize = SIZE / 4;
    const LOWER_RIGHT: isize = SIZE * 3 / 4;

    fn location_at_time(&self, speed: &Speed, t: isize) -> Self {
        Self {
            x: (speed.x * t).rem_euclid(SIZE),
            y: (speed.y * t).rem_euclid(SIZE),
        }
    }

    fn in_frame(&self) -> bool {
        self.x >= Self::UPPER_LEFT
            && self.x < Self::LOWER_RIGHT
            && self.y >= Self::UPPER_LEFT
            && self.y < Self::LOWER_RIGHT
    }
}
