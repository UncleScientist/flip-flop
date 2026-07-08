use std::fmt::Display;

pub fn run() {
    // let data = std::fs::read_to_string("test.txt").expect("file");
    let data = std::fs::read_to_string("input/puzzle-3.txt").expect("file");
    let passwords = data.lines().map(Password::new).collect::<Vec<_>>();

    let max = passwords
        .iter()
        .map(|pw| (pw, pw.p1score()))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;
    println!("Puzzle 3, part 1 = {max}");

    let max = passwords
        .iter()
        .map(|pw| (pw, pw.p2score()))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;
    println!("Puzzle 3, part 2 = {max}");

    let mut max_sum = 0;
    for append in ('a'..='z').chain(('A'..='Z').chain('0'..='9')) {
        let sum = passwords
            .iter()
            .map(|pw| pw.with(append))
            .map(|pw| pw.p2score())
            .sum::<usize>();
        if sum > max_sum {
            max_sum = sum;
        }
    }
    println!("Puzzle 3, part 3 = {max_sum}");
}

#[derive(Debug)]
struct Password(String);

impl Password {
    fn new(password: &str) -> Self {
        Self(password.to_string())
    }

    fn first_bit(&self) -> usize {
        self.0.find(char::is_lowercase).is_some() as usize
            + self.0.find(char::is_uppercase).is_some() as usize
            + self.0.find(char::is_numeric).is_some() as usize
    }

    fn p1score(&self) -> usize {
        self.first_bit() * self.0.len()
    }

    fn p2score(&self) -> usize {
        let first = self.first_bit();

        let mut found7 = 0;
        for digit in self.0.matches(char::is_numeric) {
            if digit != "7" {
                found7 = 0;
                break;
            } else {
                found7 = 7;
            }
        }

        let mut i = self.0.chars();
        let mut cur_char = i.next().unwrap();
        let mut count = 1;
        let mut max_count = 0;
        for ch in i {
            if ch == cur_char {
                count += 1;
            } else {
                if count >= 3 {
                    max_count = max_count.max(count);
                }
                cur_char = ch;
                count = 1;
            }
        }
        if count >= 3 {
            max_count = max_count.max(count);
        }

        let color = self.0.find("red").is_some()
            || self.0.find("green").is_some()
            || self.0.find("blue").is_some();

        (first + found7 + max_count * max_count) * if color { 3 } else { 1 } * self.0.len()
    }

    fn with(&self, append: char) -> Self {
        Self(format!("{}{append}", self.0))
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
