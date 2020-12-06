use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/bin/01/input.txt")
        .expect("Something went wrong reading the file");
    let numbers: Vec<u32> = contents
        .lines()
        .map(|l| l.parse().expect("parse failed"))
        .collect();

    for i in &numbers {
        for j in &numbers {
            if i + j == 2020 {
                println!("{} + {} = 2020", i, j);
                println!("{} * {} = {}", i, j, i * j)
            }

            for k in &numbers {
                if i + j + k == 2020 {
                    println!("{} + {} + {} = 2020", i, j, k);
                    println!("{} * {} * {} = {}", i, j, k, i * j * k)
                }
            }
        }
    }
}
