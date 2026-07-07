pub fn run() {
    let data = std::fs::read_to_string("input/puzzle-1.txt").expect("missing file");

    let temps = data
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let heat_up_time = temps
        .iter()
        .map(|temp| 60usize.saturating_sub(*temp))
        .sum::<usize>();
    println!("Puzzle 1, part 1 = {heat_up_time}");

    let cool_down_time = temps
        .iter()
        .map(|temp| temp.saturating_sub(60))
        .sum::<usize>()
        * 5;
    println!("Puzzle 1, part 2 = {}", heat_up_time + cool_down_time);

    let half = temps.len() / 2;
    let preferred_time = temps[0..half]
        .iter()
        .zip(temps[half..].iter())
        .map(|(actual, preferred)| {
            preferred.saturating_sub(*actual) + actual.saturating_sub(*preferred) * 5
        })
        .sum::<usize>();
    println!("Puzzle 1, part 3 = {preferred_time}");
}
