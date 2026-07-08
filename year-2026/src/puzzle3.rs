use std::fmt::Display;

pub fn run() {
    // let data = std::fs::read_to_string("test.txt").expect("file");
    let data = std::fs::read_to_string("input/puzzle-3.txt").expect("file");
    let passwords = data.lines().map(Password::new).collect::<Vec<_>>();
    let max = passwords
        .iter()
        .map(|pw| (pw, pw.score()))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;
    println!("Puzzle 3, part 1 = {max}");
}

#[derive(Debug)]
struct Password(String);

impl Password {
    fn new(password: &str) -> Self {
        Self(password.to_string())
    }

    fn score(&self) -> usize {
        let lc = self.0.find(char::is_lowercase).is_some() as usize;
        let uc = self.0.find(char::is_uppercase).is_some() as usize;
        let nm = self.0.find(char::is_numeric).is_some() as usize;
        (lc + uc + nm) * self.0.len()
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
