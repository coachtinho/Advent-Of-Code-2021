use std::fs;

struct Submarine {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

impl Submarine {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

fn challenge2(directions: &[Vec<&str>]) -> usize {
    let mut sub = Submarine::new();

    for direction in directions {
        let command = direction.get(0).unwrap();
        let x = direction.get(1).unwrap().parse::<usize>().unwrap();

        match *command {
            "forward" => {
                sub.horizontal += x;
                sub.depth += sub.aim * x;
            }
            "down" => sub.aim += x,
            "up" => sub.aim -= x,
            _ => {}
        };
    }

    sub.horizontal * sub.depth
}

fn challenge1(directions: &[Vec<&str>]) -> usize {
    let mut sub = Submarine::new();

    for direction in directions {
        let command = direction.get(0).unwrap();
        let x = direction.get(1).unwrap().parse::<usize>().unwrap();

        match *command {
            "forward" => sub.horizontal += x,
            "down" => sub.depth += x,
            "up" => sub.depth -= x,
            _ => {}
        };
    }

    sub.horizontal * sub.depth
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed reading input");
    let directions: Vec<Vec<&str>> = input.lines().map(|s| s.split(" ").collect()).collect();

    // Challenge 1
    println!("Challenge 1: {}", challenge1(directions.as_slice()));

    // Challenge 2
    println!("Challenge 2: {}", challenge2(directions.as_slice()));
}
