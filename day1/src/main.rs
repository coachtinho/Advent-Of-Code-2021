use std::fs;

fn challenge1(measurements: &[usize]) -> usize {
    measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .filter(|(m1, m2)| m2 > m1)
        .count()
}

fn challenge2(measurements: &[usize]) -> usize {
    let windows: Vec<usize> = measurements.windows(3).map(|w| w.iter().sum()).collect();

    windows
        .iter()
        .zip(windows.iter().skip(1))
        .filter(|(m1, m2)| m2 > m1)
        .count()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed reading input");
    let measurements = input
        .lines()
        .map(|m| m.parse().expect("Failed parsing numbers"))
        .collect::<Vec<usize>>();

    // Challenge 1
    println!("Challenge 1: {}", challenge1(measurements.as_slice()));

    // Challenge 2
    println!("Challenge 2: {}", challenge2(measurements.as_slice()));
}
