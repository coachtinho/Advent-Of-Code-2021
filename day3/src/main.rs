use std::fs;

const BITSIZE: usize = 12;

fn challenge1(numbers: &[&str]) -> i32 {
    let mut counts: Vec<isize> = vec![0; BITSIZE];
    let mut gamma = 0;
    let mut epsilon = 0;

    for num in numbers {
        num.chars().enumerate().for_each(|(i, bit)| {
            match bit {
                '0' => counts[i] -= 1,
                '1' => counts[i] += 1,
                _ => {}
            };
        });
    }

    counts.iter().enumerate().for_each(|(i, count)| {
        if *count > 0 {
            gamma += 2i32.pow(BITSIZE as u32 - i as u32 - 1);
        } else if *count < 0 {
            epsilon += 2i32.pow(BITSIZE as u32 - i as u32 - 1);
        }
    });

    gamma * epsilon
}

fn oxygen_count<'a>(numbers: &[&'a str], bit: usize) -> &'a str {
    let mut count = 0;

    for num in numbers {
        match num.chars().nth(bit).unwrap() {
            '0' => count -= 1,
            '1' => count += 1,
            _ => {}
        };
    }

    let mut new: Vec<&str> = Vec::new();
    for num in numbers {
        if (count > 0 && num.chars().nth(bit).unwrap() == '1')
            || (count < 0 && num.chars().nth(bit).unwrap() == '0')
            || (count == 0 && num.chars().nth(bit).unwrap() == '1')
        {
            new.push(num);
        }
    }

    if new.len() == 1 {
        new[0]
    } else {
        oxygen_count(new.as_slice(), (bit + 1) % BITSIZE)
    }
}

fn co2_count<'a>(numbers: &[&'a str], bit: usize) -> &'a str {
    let mut count = 0;

    for num in numbers {
        match num.chars().nth(bit).unwrap() {
            '0' => count -= 1,
            '1' => count += 1,
            _ => {}
        };
    }

    let mut new: Vec<&str> = Vec::new();
    for num in numbers {
        if (count > 0 && num.chars().nth(bit).unwrap() == '0')
            || (count < 0 && num.chars().nth(bit).unwrap() == '1')
            || (count == 0 && num.chars().nth(bit).unwrap() == '0')
        {
            new.push(num);
        }
    }

    if new.len() == 1 {
        new[0]
    } else {
        co2_count(new.as_slice(), (bit + 1) % BITSIZE)
    }
}

fn challenge2(numbers: &[&str]) -> i32 {
    let oxygen = i32::from_str_radix(oxygen_count(numbers, 0), 2).unwrap();
    let co2 = i32::from_str_radix(co2_count(numbers, 0), 2).unwrap();

    oxygen * co2
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed reading input");
    let numbers: Vec<&str> = input.lines().collect();

    // Challenge 1
    println!("Chalenge 1: {}", challenge1(numbers.as_slice()));

    // Challenge 2
    println!("Chalenge 2: {}", challenge2(numbers.as_slice()));
}
