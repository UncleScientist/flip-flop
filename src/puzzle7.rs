use std::{collections::HashMap, convert::Infallible, str::FromStr};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-7.txt").expect("file");
    let grids: Vec<Grid> = data.lines().map(|line| line.parse().unwrap()).collect();

    println!(
        "Puzzle 7, part 1 = {}",
        grids
            .iter()
            .map(|grid| {
                let mut cache = GridCache::default();
                cache.find(grid, &(0, 0, 0))
            })
            .sum::<usize>()
    );

    let grid3d: Vec<Grid> = grids
        .iter()
        .map(|grid| Grid {
            depth: grid.width,
            ..*grid
        })
        .collect();

    println!(
        "Puzzle 7, part 2 = {}",
        grid3d
            .iter()
            .map(|grid| {
                let mut cache = GridCache::default();
                cache.find(grid, &(0, 0, 0))
            })
            .sum::<usize>()
    );

    let multigrids: Vec<MultiGrid> = data.lines().map(|line| line.parse().unwrap()).collect();
    println!(
        "Puzzle 7, part 3 = {}",
        multigrids
            .iter()
            .map(|grid| {
                let mut cache = MultiCache::default();
                cache.find(grid, &vec![0usize; grid.dimensions])
            })
            .sum::<usize>()
    );
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    depth: usize,
}

#[derive(Default)]
struct GridCache {
    cache: HashMap<(usize, usize, usize), usize>,
}

impl GridCache {
    fn find(&mut self, grid: &Grid, pos: &(usize, usize, usize)) -> usize {
        if pos.0 + 1 == grid.width && pos.1 + 1 == grid.height && pos.2 + 1 == grid.depth {
            return 1;
        }

        if let Some(answer) = self.cache.get(pos) {
            return *answer;
        }

        let mut count = 0;

        if pos.0 + 1 < grid.width {
            count += self.find(grid, &(pos.0 + 1, pos.1, pos.2));
        }

        if pos.1 + 1 < grid.height {
            count += self.find(grid, &(pos.0, pos.1 + 1, pos.2));
        }

        if pos.2 + 1 < grid.depth {
            count += self.find(grid, &(pos.0, pos.1, pos.2 + 1));
        }

        self.cache.insert(*pos, count);
        count
    }
}

// ..........
// ...xyc....
// ...za.....
// ...b......
// ..........

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (left, right) = line.split_once(' ').unwrap();
        Ok(Self {
            width: left.parse().unwrap(),
            height: right.parse().unwrap(),
            depth: 1,
        })
    }
}

#[derive(Debug)]
struct MultiGrid {
    dimensions: usize,
    size: usize,
}

impl FromStr for MultiGrid {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (left, right) = line.split_once(' ').unwrap();
        Ok(Self {
            dimensions: left.parse().unwrap(),
            size: right.parse().unwrap(),
        })
    }
}

#[derive(Default)]
struct MultiCache {
    cache: HashMap<Vec<usize>, usize>,
}

impl MultiCache {
    fn find(&mut self, grid: &MultiGrid, pos: &[usize]) -> usize {
        if pos.iter().all(|p| p + 1 == grid.size) {
            return 1;
        }

        if let Some(answer) = self.cache.get(pos) {
            return *answer;
        }

        let mut count = 0;

        for i in 0..pos.len() {
            if pos[i] + 1 < grid.size {
                let mut next = pos.to_vec();
                next[i] += 1;
                count += self.find(grid, &next);
            }
        }

        self.cache.insert(pos.to_vec(), count);
        count
    }
}
