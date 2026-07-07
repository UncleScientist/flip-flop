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
    label: Label,
}

impl Color {
    fn label(&self) -> Label {
        self.label
    }

    fn generate_label(red: i32, green: i32, blue: i32) -> Label {
        if red == green || red == blue || green == blue {
            return Label::Special;
        }

        if red > green && red > blue {
            Label::Red
        } else if green > red && green > blue {
            Label::Green
        } else if blue > red && blue > green {
            Label::Blue
        } else {
            panic!("color cannot be labelled: {red}, {green}, {blue}");
        }
    }

    fn cost(&self) -> i32 {
        self.label.cost()
    }
}

impl FromStr for Color {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut nums = line.split(',');
        let red = nums.next().unwrap().parse().unwrap();
        let green = nums.next().unwrap().parse().unwrap();
        let blue = nums.next().unwrap().parse().unwrap();
        Ok(Self {
            red,
            green,
            blue,
            label: Self::generate_label(red, green, blue),
        })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.red, self.green, self.blue)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
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
