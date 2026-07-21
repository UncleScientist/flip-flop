use std::{collections::HashMap, convert::Infallible, str::FromStr};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-11.txt").expect("missing input");
    // let data = std::fs::read_to_string("test.txt").expect("missing input");
    let dna_sets = data.split("\n\n").collect::<Vec<_>>();

    let dna_sets = dna_sets
        .into_iter()
        .map(|set| set.parse::<DnaSet>().unwrap())
        .collect::<Vec<_>>();

    let mut biomass = 0;
    println!("{}", dna_sets.len());
    for (i, set) in dna_sets.into_iter().enumerate() {
        let mut tree = Tree::new(set);
        for year in 0..100 {
            tree.grow();

            let energy_required = tree.energy_required();
            let energy_produced = tree.energy_produced();

            if year >= 4 && energy_required > energy_produced {
                break;
            }
        }
        println!("{i:3} = {}", tree.mass());
        biomass += tree.mass();
    }

    println!("Puzzle 11, part 1 = {biomass}");
}

#[derive(Debug)]
struct Tree {
    rules: DnaSet,
    trunk: HashMap<(isize, isize), Option<usize>>,
    height: isize,
    leftmost: isize,
    rightmost: isize,
}

impl Tree {
    fn new(rules: DnaSet) -> Self {
        Self {
            rules,
            trunk: HashMap::from([((0, 0), Some(0))]),
            height: 1,
            leftmost: 0,
            rightmost: 0,
        }
    }

    fn energy_required(&self) -> usize {
        self.trunk.len() * 3
    }

    fn mass(&self) -> usize {
        self.trunk.len()
    }

    fn grow(&mut self) {
        let mut new_trunk: HashMap<(isize, isize), Option<usize>> = self
            .trunk
            .iter()
            .filter(|(_, x)| x.is_none())
            .map(|(p, s)| (*p, *s))
            .collect();

        for (pos, segment) in self.trunk.drain() {
            if let Some(sprout) = segment {
                for (id, dir) in [
                    self.rules.dna[sprout].left,
                    self.rules.dna[sprout].above,
                    self.rules.dna[sprout].right,
                ]
                .iter()
                .zip(DIRS.iter())
                {
                    if let Some(next) = id {
                        let newpos = match dir {
                            Dir::Left => (pos.0, pos.1 - 1),
                            Dir::Above => (pos.0 + 1, pos.1),
                            Dir::Right => (pos.0, pos.1 + 1),
                        };

                        let entry = new_trunk.get(&newpos);
                        if entry.is_none() {
                            new_trunk.insert(newpos, *id);
                        } else if let Some(sprout) = entry
                            && let Some(s) = sprout
                            && s < next
                        {
                            new_trunk.insert(newpos, *id);
                        }
                        self.height = self.height.max(newpos.0);
                        self.leftmost = self.leftmost.min(newpos.1);
                        self.rightmost = self.rightmost.max(newpos.1);
                    }
                }
            }
            new_trunk.insert(pos, None);
        }

        self.trunk = new_trunk;
    }

    fn energy_produced(&self) -> usize {
        let mut energy = 0;
        for (pos, _) in self.trunk.iter().filter(|(_, x)| x.is_none()) {
            let height = 10.min(pos.0 + 1);
            let mut multiplier = 3;
            let mut look = pos.0 + 1;
            while multiplier > 0 && look <= self.height {
                if let Some(item) = self.trunk.get(&(look, pos.1))
                    && item.is_none()
                {
                    multiplier -= 1;
                }
                look += 1;
            }
            energy += height * multiplier;
        }

        energy as usize
    }

    fn _print(&self) {
        for row in 0..=self.height {
            for col in self.leftmost..=self.rightmost {
                if let Some(segment) = self.trunk.get(&(row, col)) {
                    if segment.is_none() {
                        print!("#");
                    } else {
                        print!("@");
                    }
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
struct DnaSet {
    dna: Vec<DnaID>,
}

impl FromStr for DnaSet {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let num_to_index = |num: &str| num.parse::<usize>().ok();

        let mut dna = Vec::new();

        let (top, bottom) = line.split_once('\n').unwrap();
        let tops = top.split_whitespace().map(num_to_index).collect::<Vec<_>>();
        let bottoms = bottom
            .split_whitespace()
            .map(num_to_index)
            .collect::<Vec<_>>();

        for (index, (top, sides)) in tops.iter().zip(bottoms.chunks(3)).enumerate() {
            assert_eq!(Some(index), sides[1]);

            dna.push(DnaID {
                left: sides[0],
                above: *top,
                right: sides[2],
            });
        }

        Ok(Self { dna })
    }
}

#[derive(Default, Debug)]
struct DnaID {
    left: Option<usize>,
    above: Option<usize>,
    right: Option<usize>,
}

const DIRS: [Dir; 3] = [Dir::Left, Dir::Above, Dir::Right];

#[derive(Debug)]
enum Dir {
    Left,
    Above,
    Right,
}
