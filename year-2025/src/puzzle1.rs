pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-1.txt").expect("missing file");
    let count = get_count(data.lines(), banane);
    println!("Puzzle 1, part 1 = {}", count.iter().sum::<usize>());
    println!(
        "Puzzle 1, part 2 = {}",
        count
            .iter()
            .filter(|val| val.is_multiple_of(2))
            .sum::<usize>()
    );
    let count = get_count(data.lines(), banana);
    println!("Puzzle 1, part 3 = {}", count.iter().sum::<usize>());
}

fn banane(_chunk: &str) -> bool {
    true
}

fn banana(chunk: &str) -> bool {
    !chunk.contains("ne")
}

fn get_count(lines: std::str::Lines, filter: fn(&str) -> bool) -> Vec<usize> {
    lines
        .filter(|line| filter(line))
        .map(|line| line.chars().collect::<Vec<_>>().chunks(2).count())
        .collect::<Vec<_>>()
}
