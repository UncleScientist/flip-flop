use std::{collections::HashMap, convert::Infallible, fmt::Display, str::FromStr};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-3.txt").expect("missing file");
    let colors: Vec<Color> = data.lines().map(|line| line.parse().unwrap()).collect();

    let mut counts = HashMap::<Color, usize>::new();
    for c in &colors {
        *counts.entry(c.clone()).or_insert(0) += 1;
    }
    let mut count_list = counts.iter().collect::<Vec<_>>();
    count_list.sort_by(|a, b| b.1.cmp(a.1));
    println!("Puzzle 3, part 1 = {}", count_list[0].0);

    println!(
        "Puzzle 3, part 2 = {}",
        colors
            .iter()
            .filter(|color| color.label() == Label::Green)
            .count()
    );

    println!(
        "Puzzle 3, part 3 = {}",
        colors.iter().map(|color| color.cost()).sum::<i32>()
    );
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Color {
    red: i32,
    green: i32,
    blue: i32,
}

impl Color {
    fn label(&self) -> Label {
        if self.red == self.green || self.red == self.blue || self.green == self.blue {
            return Label::Special;
        }

        if self.red > self.green && self.red > self.blue {
            Label::Red
        } else if self.green > self.red && self.green > self.blue {
            Label::Green
        } else if self.blue > self.red && self.blue > self.green {
            Label::Blue
        } else {
            panic!("color cannot be labelled: {self}");
        }
    }

    fn cost(&self) -> i32 {
        self.label().cost()
    }
}

impl FromStr for Color {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut nums = line.split(',');
        Ok(Self {
            red: nums.next().unwrap().parse().unwrap(),
            green: nums.next().unwrap().parse().unwrap(),
            blue: nums.next().unwrap().parse().unwrap(),
        })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.red, self.green, self.blue)
    }
}

#[derive(PartialEq, Eq)]
enum Label {
    Red,
    Green,
    Blue,
    Special,
}

impl Label {
    fn cost(&self) -> i32 {
        match self {
            Label::Red => 5,
            Label::Green => 2,
            Label::Blue => 4,
            Label::Special => 10,
        }
    }
}
