use std::collections::HashSet;

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-5.txt").expect("missing file");

    let lines = data.lines().collect::<Vec<_>>();

    let mut streets: Vec<Vec<char>> = Vec::new();
    for line in lines.iter() {
        streets.push(line.chars().collect());
    }

    println!("Puzzle 5, part 1 = {}", count_visited(&streets));
}

fn count_visited(streets: &[Vec<char>]) -> usize {
    let mut pos = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(pos);

    loop {
        let next = streets[pos.0][pos.1];
        pos = match next {
            '>' => (pos.0, pos.1 + 1),
            '<' => (pos.0, pos.1 - 1),
            'v' => (pos.0 + 1, pos.1),
            '^' => (pos.0 - 1, pos.1),
            _ => panic!("invalid char {next} in street"),
        };
        if !visited.insert(pos) {
            break;
        }
    }

    visited.len()
}
