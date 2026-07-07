use std::collections::{HashMap, HashSet};

pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-5.txt").expect("file");
    let tunnels = data.trim().chars().collect::<Vec<_>>();

    let mut tunnel_map: HashMap<(char, usize), usize> = HashMap::new();
    let mut locations = vec![None; 256];
    for (tunnel_idx, ch) in tunnels.iter().enumerate() {
        let loc_idx = (*ch as u8 - b'0') as usize;
        if let Some(pos) = locations[loc_idx] {
            tunnel_map.insert((*ch, tunnel_idx), pos);
            tunnel_map.insert((*ch, pos), tunnel_idx);
        } else {
            locations[loc_idx] = Some(tunnel_idx);
        }
    }

    let mut total_distance = 0;
    let mut index = 0;
    let mut visited = HashSet::new();
    while index < tunnels.len() {
        let cur_char = tunnels[index];
        visited.insert(cur_char);
        let Some(outlet) = tunnel_map.get(&(cur_char, index)) else {
            panic!("No tunnel found @ {cur_char},{index}");
        };
        total_distance += outlet.abs_diff(index);
        index = *outlet + 1;
    }
    println!("Puzzle 5, part 1 = {total_distance}");

    let mut part2 = String::from("");
    for ch in &tunnels {
        if visited.insert(*ch) {
            part2.push(*ch);
        }
    }
    println!("Puzzle 5, part 2 = {part2}");

    let mut total_distance = 0isize;
    let mut index = 0;
    let mut visited = HashSet::new();
    while index < tunnels.len() {
        let cur_char = tunnels[index];
        visited.insert(cur_char);
        let Some(outlet) = tunnel_map.get(&(cur_char, index)) else {
            panic!("No tunnel found @ {cur_char},{index}");
        };
        if cur_char.is_ascii_uppercase() {
            total_distance -= outlet.abs_diff(index) as isize;
        } else {
            total_distance += outlet.abs_diff(index) as isize;
        }
        index = *outlet + 1;
    }
    println!("Puzzle 5, part 3 = {total_distance}");
}
