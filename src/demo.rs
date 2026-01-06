pub fn run() {
    let part1 = std::fs::read_to_string("input/demo-1.txt")
        .expect("missing file")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .sum::<usize>();
    println!("Demo: part 1 = {part1}");
}
