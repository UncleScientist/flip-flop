use std::collections::HashMap;

pub(crate) fn run() {
    // let data = std::fs::read_to_string("test.txt").expect("file");
    let data = std::fs::read_to_string("input/puzzle-8.txt").expect("file");
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
}
