use std::{convert::Infallible, str::FromStr};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-4.txt").expect("can't read file");
    // let data = std::fs::read_to_string("test.txt").expect("can't read file");

    let flower = Flowerstalk::new(data.lines().map(|line| line.parse::<Branch>().unwrap()));
    println!("Puzzle 4, part 1 = {}", flower.count_leaves_above(400));

    println!("Puzzle 4, part 2 = {}", flower.count_swaps());
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

    fn count_swaps(&self) -> usize {
        let pos = self
            .0
            .iter()
            .position(|branch| matches!(branch, Branch::Left | Branch::Right))
            .expect("Need at least one branch in the puzzle input");
        let mut cur_side = self.0[pos];
        let mut swaps = 0;

        for branch in &self.0[pos + 1..] {
            if !matches!(branch, Branch::Left | Branch::Right) {
                continue;
            }
            if *branch != cur_side {
                swaps += 1;
                cur_side = *branch;
            }
        }
        swaps
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
