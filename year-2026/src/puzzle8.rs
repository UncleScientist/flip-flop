use std::collections::HashMap;

pub(crate) fn run() {
    // let data = std::fs::read_to_string("test.txt").expect("file");
    let data = std::fs::read_to_string("input/puzzle-8.txt").expect("file");

    // Part 1 - Simple evolution

    let mut map = HashMap::new();
    for line in data.lines() {
        let (from, to) = line.split_once(' ').unwrap();
        let from = from.chars().next().unwrap();
        map.entry(from)
            .or_insert_with(|| to.split_whitespace().collect::<String>());
    }

    let mut stoats = "AB".to_string();
    for _generation in 0..7 {
        let mut next_stoats = String::new();
        for stoat in stoats.chars() {
            let children = &map[&stoat];
            next_stoats.push_str(children);
        }

        stoats = next_stoats;
    }
    println!("Puzzle 8, part 1: {}", stoats.len());

    // Part 2 - pairwise evolution
    let mut map = HashMap::new();
    for line in data.lines() {
        let rule = line
            .split_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect::<Vec<_>>();

        map.entry((rule[0], rule[1]))
            .or_insert_with(|| rule[2..].iter().collect::<String>());
        map.entry((rule[1], rule[0]))
            .or_insert_with(|| rule[2..].iter().collect::<String>());
    }

    let mut stoats = "AB".to_string();
    for _generation in 0..7 {
        let mut next_stoats = String::new();
        let slice = stoats.chars().collect::<Vec<_>>();
        next_stoats.push(slice[0]);
        for stoat_pair in slice.windows(2) {
            if let Some(children) = map.get(&(stoat_pair[0], stoat_pair[1])) {
                next_stoats.push_str(children);
                next_stoats.push(stoat_pair[1]);
            } else {
                panic!("can't find stoat pair for {stoat_pair:?}");
            }
        }

        stoats = next_stoats;
    }
    println!("Puzzle 8, part 2: {}", stoats.len());
}
