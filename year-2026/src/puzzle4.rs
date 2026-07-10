use std::{convert::Infallible, str::FromStr};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-4.txt").expect("can't read file");

    let flower = Flowerstalk::new(data.lines().map(|line| line.parse::<Branch>().unwrap()));
    println!("Puzzle 4, part 1 = {}", flower.count_leaves_above(400));
}

#[derive(Debug)]
struct Flowerstalk(Vec<Branch>);

impl Flowerstalk {
    fn new(flower: impl Iterator<Item = Branch>) -> Self {
        Self(flower.collect())
    }

    fn count_leaves_above(&self, height: usize) -> usize {
        self.0[0..self.0.len() - height - 1]
            .iter()
            .filter(|block| matches!(block, Branch::Left | Branch::Right))
            .count()
    }
}

#[derive(Debug)]
enum Branch {
    Left,
    Right,
    Stem,
    Flower,
    Ground,
}

impl FromStr for Branch {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.contains("/") || line.contains("@") {
            Ok(Self::Flower)
        } else if line.contains("|-o") {
            Ok(Self::Right)
        } else if line.contains("o-|") {
            Ok(Self::Left)
        } else if line.contains(" |") {
            Ok(Self::Stem)
        } else {
            println!("-- {line} --");
            assert!(line.contains("#"));
            Ok(Self::Ground)
        }
    }
}
