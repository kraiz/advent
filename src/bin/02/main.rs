use recap::Recap;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"(?x)^(?P<min>\d+)-(?P<max>\d+)\s(?P<char>\w):\s(?P<pw>\w+)$"#)]
struct Line {
    min: usize,
    max: usize,
    char: char,
    pw: String,
}

fn main() {
    let contents = fs::read_to_string("./src/bin/02/input.txt")
        .expect("Something went wrong reading the file");
    let mut valid1 = 0;
    let mut valid2 = 0;

    for line in contents.lines().map(|l| l.parse::<Line>().unwrap()) {
        // part 1
        let amount = line.pw.chars().filter(|c| *c == line.char).count();
        if amount >= line.min && amount <= line.max {
            valid1 += 1
        }
        // part 2
        if line
            .pw
            .chars()
            .nth(line.min - 1)
            .filter(|c| *c == line.char)
            .xor(
                line.pw
                    .chars()
                    .nth(line.max - 1)
                    .filter(|c| *c == line.char),
            )
            .is_some()
        {
            valid2 += 1
        }
    }
    println!("Valid part 1: {}", valid1);
    println!("Valid part 2: {}", valid2);
}
