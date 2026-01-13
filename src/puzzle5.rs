use std::collections::HashSet;

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-5.txt").expect("file");
    let tunnels = data.trim().chars().collect::<Vec<_>>();

    let mut total_distance = 0;
    let mut index = 0;
    let mut visited = HashSet::new();
    while index < tunnels.len() {
        let cur_char = tunnels[index];
        visited.insert(cur_char);
        for (idx, ch) in tunnels.iter().enumerate() {
            if idx == index {
                continue;
            }
            if *ch == cur_char {
                total_distance += index.abs_diff(idx);
                index = idx;
                break;
            }
        }
        index += 1;
    }
    println!("Puzzle 5, part 1 = {total_distance}");

    let mut part2 = String::from("");
    for ch in &tunnels {
        if visited.insert(*ch) {
            part2.push(*ch);
        }
    }
    println!("Puzzle 5, part 2 = {part2}");
}
