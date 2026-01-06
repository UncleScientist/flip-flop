pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-2.txt").expect("missing file");

    let mut height = 0;
    let mut max_height = 0;
    for ch in data.lines().next().unwrap().chars() {
        match ch {
            '^' => height += 1,
            'v' => height -= 1,
            _ => panic!("illegal input: '{ch}'"),
        }
        max_height = max_height.max(height);
    }
    println!("Puzzle 2, part 1 = {max_height}");

    let mut height = 0;
    let mut max_height = 0;
    let mut cur_steepness = 1;
    let mut prev_direction = '.';
    for ch in data.lines().next().unwrap().chars() {
        match ch {
            '^' => {
                if prev_direction != '^' {
                    cur_steepness = 1;
                    prev_direction = ch;
                }
                height += cur_steepness;
                cur_steepness += 1;
            }
            'v' => {
                if prev_direction != 'v' {
                    cur_steepness = 1;
                    prev_direction = ch;
                }
                height -= cur_steepness;
                cur_steepness += 1;
            }
            _ => panic!("illegal input: '{ch}'"),
        }
        max_height = max_height.max(height);
    }
    println!("Puzzle 2, part 1 = {max_height}");
}
