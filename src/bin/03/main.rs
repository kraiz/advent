use std::fs;

fn run(map: &Vec<Vec<char>>, rule: (usize, usize)) -> usize {
    let mut pos = [0, 0];
    let mut trees = 0;
    loop {
        pos[0] = (pos[0] + rule.0) % 31;
        pos[1] += rule.1;
        if pos[1] >= map.len() {
            break;
        }
        if map[pos[1]][pos[0]] == '#' {
            trees += 1;
        }
    }
    trees
}

fn main() {
    let contents = fs::read_to_string("./src/bin/03/input.txt")
        .expect("Something went wrong reading the file");

    let map = contents
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let rules = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product = 1;
    for (i, rule) in rules.iter().enumerate() {
        let trees = run(&map, *rule);
        println!("Trees rule #{}: {}", i, trees);
        product *= trees;
    }

    println!("Product: {}", product);
}
