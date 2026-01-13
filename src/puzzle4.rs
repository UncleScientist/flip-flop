use std::{convert::Infallible, str::FromStr};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-4.txt").expect("file");
    let coords: Vec<Coord> = data.lines().map(|line| line.parse().unwrap()).collect();

    println!(
        "Puzzle 4, part 1 = {}",
        coords
            .iter()
            .fold((0, Coord::zero()), |(dist, pos), cur_pos| {
                // println!("{dist}, {pos:?}");
                (dist + cur_pos.manhattan_distance_to(&pos), *cur_pos)
            })
            .0
    );
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn _xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    fn manhattan_distance_to(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl FromStr for Coord {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (left, right) = line.split_once(',').unwrap();
        Ok(Self {
            x: left.parse().unwrap(),
            y: right.parse().unwrap(),
        })
    }
}
