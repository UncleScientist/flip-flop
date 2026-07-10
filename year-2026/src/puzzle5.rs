use std::collections::HashSet;

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-5.txt").expect("missing file");
    // let data = std::fs::read_to_string("test.txt").expect("missing file");

    let lines = data.lines().collect::<Vec<_>>();

    let mut streets: Vec<Vec<char>> = Vec::new();
    for line in lines.iter() {
        streets.push(line.chars().collect());
    }

    let baseline_visited = find_visited(&streets);
    println!("Puzzle 5, part 1 = {}", baseline_visited.len());

    let last_row = streets.len() - 1;
    let last_col = streets[0].len() - 1;

    let mut max_visited = baseline_visited.len();

    for rotate in &baseline_visited {
        if rotate.0 == 0 || rotate.0 == last_row || rotate.1 == 0 || rotate.1 == last_col {
            continue;
        }
        let mut permute = streets.clone();
        for _ in 0..3 {
            permute[rotate.0][rotate.1] = match permute[rotate.0][rotate.1] {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("compiler bug"),
            };
            let visited = find_visited(&permute).len();
            max_visited = max_visited.max(visited);
        }
    }
    println!("Puzzle 5, part 2 = {max_visited}");
}

fn find_visited(streets: &[Vec<char>]) -> HashSet<(usize, usize)> {
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

    visited
}
