use std::collections::HashMap;

pub fn run() {
    let numbers = std::fs::read_to_string("input/demo.txt")
        .expect("missing file")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let sum = numbers.iter().sum::<usize>();

    println!("Demo: part 1 = {sum}");
    println!(
        "Demo: part 2 = {}",
        (sum as f64 / numbers.len() as f64).round()
    );

    let mut num_counts = HashMap::<usize, usize>::new();
    let mut digit_counts = HashMap::<char, usize>::new();
    for num in &numbers {
        *num_counts.entry(*num).or_insert(0) += 1;
        for ch in format!("{num}").chars() {
            *digit_counts.entry(ch).or_insert(0) += 1;
        }
    }
    let mut num_tups = num_counts.iter().collect::<Vec<_>>();
    num_tups.sort_by(|a, b| b.1.cmp(a.1));
    let mut dig_tups = digit_counts.iter().collect::<Vec<_>>();
    dig_tups.sort_by(|a, b| a.1.cmp(b.1));
    println!("Demo: part 3 = {}{}", num_tups[0].0, dig_tups[0].0);
}
