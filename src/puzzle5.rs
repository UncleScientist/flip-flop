pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-5.txt").expect("file");
    let tunnels = data.trim().chars().collect::<Vec<_>>();

    let mut total_distance = 0;
    let mut index = 0;
    while index < tunnels.len() {
        let cur_char = tunnels[index];
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
}
