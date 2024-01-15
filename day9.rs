use std::io;
use std::io::BufRead;

#[derive(Debug, Clone)]
enum Direction {
    Forward,
    Backward,
}

impl Direction {
    fn predict(&self, list: &[i32], prev_predicition: i32) -> i32 {
        match self {
            Direction::Forward => list.last().unwrap() + prev_predicition,
            Direction::Backward => list.first().unwrap() - prev_predicition,
        }
    }
}

fn differences(input_list: &[i32]) -> Vec<i32> {
    input_list
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect()
}

fn predict(line: &[i32], direction: Direction) -> i32 {
    let diffs = differences(line);

    if line.iter().all(|&x| x == 0) {
        0
    } else {

        direction.predict(line, predict(&diffs, direction.clone()))
    }
}   

fn main() {
    println!("Please enter multiple lines of numbers (press Ctrl+D when finished):");

    let stdin = io::stdin();
    let mut res = 0;

    for line in stdin.lock().lines() {
        let input = line.expect("Failed to read line");

        if input.trim().is_empty() {
            break; // Exit the loop if an empty line is encountered
        }

        let parsed_numbers: Vec<i32> = input
            .split_whitespace()
            .map(|num| num.trim().parse::<i32>().expect("Invalid input"))
            .collect();

        res += predict(&parsed_numbers, Direction::Forward);
    }

    println!("{:?}", res);
}


