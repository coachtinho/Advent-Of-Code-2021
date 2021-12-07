use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl From<&str> for Line {
    fn from(line: &str) -> Self {
        let mut pair = line.split(" -> ");
        let mut start = pair
            .next()
            .unwrap()
            .split(",")
            .map(|n| n.parse::<usize>().unwrap());
        let mut end = pair
            .next()
            .unwrap()
            .split(",")
            .map(|n| n.parse::<usize>().unwrap());

        let x1 = start.next().unwrap();
        let y1 = start.next().unwrap();
        let x2 = end.next().unwrap();
        let y2 = end.next().unwrap();

        Self { x1, y1, x2, y2 }
    }
}

fn generic_range(a: usize, b: usize) -> Box<dyn Iterator<Item = usize>> {
    if a < b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}

fn challenge1(lines: &[Line]) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    lines
        .iter()
        .filter(|line| line.x1 == line.x2 || line.y1 == line.y2)
        .for_each(|line| {
            for x in generic_range(line.x1, line.x2) {
                for y in generic_range(line.y1, line.y2) {
                    let current_count = *map.entry((x, y)).or_insert(0);
                    map.insert((x, y), current_count + 1);
                }
            }
        });

    map.into_iter().filter(|(_, v)| *v >= 2).count()
}

fn challenge2(lines: &[Line]) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    lines.iter().for_each(|line| {
        if line.x1 == line.x2 || line.y1 == line.y2 {
            for x in generic_range(line.x1, line.x2) {
                for y in generic_range(line.y1, line.y2) {
                    let current_count = *map.entry((x, y)).or_insert(0);
                    map.insert((x, y), current_count + 1);
                }
            }
        } else {
            for (x, y) in generic_range(line.x1, line.x2).zip(generic_range(line.y1, line.y2)) {
                let current_count = *map.entry((x, y)).or_insert(0);
                map.insert((x, y), current_count + 1);
            }
        }
    });

    map.into_iter().filter(|(_, v)| *v >= 2).count()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed reading input");
    let mut lines: Vec<Line> = Vec::new();
    for line in input.lines() {
        lines.push(Line::from(line));
    }

    // Challenge 1
    println!("Challenge 1: {}", challenge1(lines.as_slice()));

    // Challenge 2
    println!("Challenge 2: {}", challenge2(lines.as_slice()));
}
