use std::{collections::HashMap, convert::Infallible, str::FromStr};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-11.txt").expect("missing input");
    //let data = std::fs::read_to_string("test.txt").expect("missing input");
    let dna_sets = data
        .split("\n\n")
        .into_iter()
        .map(|set| set.parse::<DnaSet>().unwrap())
        .collect::<Vec<_>>();

    println!("Puzzle 11, part 1 = {}", part1(&dna_sets));

    /*
    let data = std::fs::read_to_string("test2.txt").expect("missing input");
    let dna_sets = data
        .split("\n\n")
        .into_iter()
        .map(|set| set.parse::<DnaSet>().unwrap())
        .collect::<Vec<_>>();
    */
    println!("Puzzle 11, part 2 = {}", part2(&dna_sets));
}

fn part1(dna_sets: &[DnaSet]) -> usize {
    let mut biomass = 0;
    for set in dna_sets {
        let mut tree = Tree::new(set.clone(), TreeID(0));
        let mut forest = Forest::new(1);

        for year in 0..100 {
            tree.grow(&mut forest);

            let energy_required = tree.energy_required(&forest);
            let energy_produced = tree.energy_produced(&forest);

            if year >= 4 && energy_required > energy_produced {
                break;
            }
        }
        biomass += tree.mass(&forest);
    }

    biomass
}

fn part2(dna_sets: &[DnaSet]) -> usize {
    let mut forest = Forest::new(dna_sets.len());

    let mut trees = dna_sets
        .iter()
        .enumerate()
        .map(|(id, set)| Tree::new(set.clone(), TreeID(id)))
        .collect::<Vec<_>>();

    for year in 0..100 {
        for tree in trees.iter_mut().filter(|tree| !tree.dead) {
            tree.grow(&mut forest);
        }

        for tree in trees.iter_mut().filter(|tree| !tree.dead) {
            let energy_required = tree.energy_required(&forest);
            let energy_produced = tree.energy_produced(&forest);

            if year >= 4 && energy_required > energy_produced {
                tree.dead = true;
            }
        }
    }

    println!("{}", forest.mass());
    trees
        .iter()
        .filter(|tree| tree.dead)
        .map(|tree| tree.mass(&forest))
        .sum()
}

#[derive(Debug, Default)]
struct Forest {
    trees: HashMap<(isize, isize), Segment>,
    height: isize,
    leftmost: isize,
    rightmost: isize,
}

impl Forest {
    fn new(tree_count: usize) -> Self {
        let trees = (0..tree_count)
            .map(|tree| {
                (
                    (0isize, 10 * tree as isize),
                    Segment::Sprout(TreeID(tree), 0),
                )
            })
            .collect::<HashMap<_, _>>();
        Self {
            trees,
            height: 1,
            ..Self::default()
        }
    }

    fn get_segments(&self, id: TreeID) -> Self {
        let trees = self
            .trees
            .iter()
            .filter(|(_, seg)| match seg {
                Segment::Stem(_) => true,
                Segment::Sprout(tree_id, _) => *tree_id != id,
            })
            // .cloned()
            .map(|(pos, seg)| (*pos, *seg)) // why can't I clone?
            .collect();
        Self { trees, ..*self }
    }

    fn energy_required_for(&self, id: TreeID) -> usize {
        self.trees
            .iter()
            .filter(|(_, seg)| match seg {
                Segment::Stem(tree_id) => id == *tree_id,
                Segment::Sprout(tree_id, _) => id == *tree_id,
            })
            .count()
            * 3
    }

    fn energy_produced_by(&self, id: TreeID) -> usize {
        let this_tree = self.trees.iter().filter(|(_, seg)| match seg {
            Segment::Stem(tree_id) => id == *tree_id,
            _ => false,
        });
        let mut energy = 0;

        for (pos, _) in this_tree {
            let height = 10.min(pos.0 + 1);
            let mut multiplier = 3;
            let mut look = pos.0 + 1;
            while multiplier > 0 && look <= self.height {
                if let Some(item) = self.trees.get(&(look, pos.1))
                    && matches!(item, Segment::Stem(_))
                {
                    multiplier -= 1;
                }
                look += 1;
            }
            energy += height * multiplier;
        }

        energy as usize
    }

    fn find_segments_for(&self, id: TreeID) -> impl Iterator<Item = (&(isize, isize), &Segment)> {
        self.trees.iter().filter(move |(_, seg)| match seg {
            Segment::Stem(tree_id) => id == *tree_id,
            Segment::Sprout(tree_id, _) => id == *tree_id,
        })
    }

    fn _print(&self) {
        for row in 0..=self.height {
            for col in self.leftmost..=self.rightmost {
                if let Some(segment) = self.trees.get(&(row, col)) {
                    match segment {
                        Segment::Stem(_) => print!("#"),
                        Segment::Sprout(_, _) => print!("@"),
                    }
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn mass(&self) -> usize {
        self.trees.len()
    }

    fn mass_of(&self, id: TreeID) -> usize {
        self.find_segments_for(id).count()
    }
}

#[derive(Debug)]
struct Tree {
    rules: DnaSet,
    id: TreeID,
    dead: bool,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Segment {
    Stem(TreeID),
    Sprout(TreeID, usize),
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct TreeID(usize);

impl Tree {
    fn new(rules: DnaSet, id: TreeID) -> Self {
        Self {
            rules,
            id,
            dead: false,
        }
    }

    fn energy_required(&self, forest: &Forest) -> usize {
        forest.energy_required_for(self.id)
    }

    fn mass(&self, forest: &Forest) -> usize {
        forest.mass_of(self.id)
    }

    fn grow(&mut self, forest: &mut Forest) {
        let mut new_forest: Forest = forest.get_segments(self.id);

        for (pos, segment) in forest.find_segments_for(self.id) {
            if let Segment::Sprout(_, sprout) = segment {
                for (id, dir) in [
                    self.rules.dna[*sprout].left,
                    self.rules.dna[*sprout].above,
                    self.rules.dna[*sprout].right,
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

                        let entry = new_forest.trees.get(&newpos);
                        if entry.is_none() {
                            new_forest
                                .trees
                                .insert(newpos, Segment::Sprout(self.id, *next));
                        } else if let Some(Segment::Sprout(tree_id, sprout)) = entry
                            && *tree_id == self.id
                            && sprout < next
                        {
                            new_forest
                                .trees
                                .insert(newpos, Segment::Sprout(self.id, *next));
                        }
                        new_forest.height = new_forest.height.max(newpos.0);
                        new_forest.leftmost = new_forest.leftmost.min(newpos.1);
                        new_forest.rightmost = new_forest.rightmost.max(newpos.1);
                    }
                }
            }
            new_forest.trees.insert(*pos, Segment::Stem(self.id));
        }

        forest.trees = new_forest.trees;
        forest.height = new_forest.height;
        forest.leftmost = new_forest.leftmost;
        forest.rightmost = new_forest.rightmost;
    }

    fn energy_produced(&self, forest: &Forest) -> usize {
        forest.energy_produced_by(self.id)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Default, Debug, Clone)]
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
