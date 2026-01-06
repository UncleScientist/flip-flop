pub fn run() {
    let numbers = std::fs::read_to_string("input/demo.txt")
        .expect("missing file")
        .lines()
        .map(|line| line.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    let sum = numbers.iter().sum::<f64>();
    println!("Demo: part 1 = {sum}");

    // let numbers = [11, 22, 30, 34, 48, 8, 57, 57, 69, 69, 69];
    println!("Demo: part 2 = {}", (sum / numbers.len() as f64).round());
}
