pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-1.txt").expect("missing file");
    let count = data
        .lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<_>>()
                .chunks(2)
                .filter(|chunk| {
                    (chunk[0] == 'b' && chunk[1] == 'a')
                        || (chunk[0] == 'n' && (chunk[1] == 'a' || chunk[1] == 'e'))
                })
                .count()
        })
        .sum::<usize>();
    println!("Puzzle 1, part 1 = {count}");
}
