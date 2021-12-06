use std::fs;

const BOARDSIZE: usize = 5;

#[derive(Clone)]
struct Number {
    value: usize,
    marked: bool,
}

impl Number {
    fn new(value: usize) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

#[derive(Clone)]
struct Board {
    numbers: Vec<Vec<Number>>,
    rows: Vec<usize>,
    cols: Vec<usize>,
    win: bool,
}

impl From<&[&str]> for Board {
    fn from(lines: &[&str]) -> Self {
        let mut numbers: Vec<Vec<Number>> = Vec::with_capacity(BOARDSIZE);

        // Skip first empty line
        for row in lines.iter().skip(1) {
            let new = row
                .trim()
                .split(" ")
                .filter(|n| *n != "")
                .map(|n| Number::new(n.trim().parse::<usize>().unwrap()))
                .collect();

            numbers.push(new);
        }

        Self {
            numbers,
            rows: vec![0; BOARDSIZE],
            cols: vec![0; BOARDSIZE],
            win: false,
        }
    }
}

fn winning_board_call<'a>(calls: &[usize], boards: &'a mut Vec<Board>) -> (&'a Board, usize) {
    for call in calls {
        for board in 0..boards.len() {
            for row in 0..BOARDSIZE {
                for col in 0..BOARDSIZE {
                    if boards[board].numbers[row][col].value == *call {
                        boards[board].numbers[row][col].marked = true;

                        boards[board].rows[row] += 1;
                        if boards[board].rows[row] == BOARDSIZE {
                            return (&boards[board], *call);
                        }

                        boards[board].cols[col] += 1;
                        if boards[board].cols[col] == BOARDSIZE {
                            return (&boards[board], *call);
                        }
                    }
                }
            }
        }
    }

    (&boards[0], calls[0])
}

fn last_winning_board_call(calls: &[usize], boards: &mut Vec<Board>) -> (Board, usize) {
    let mut win_board = boards[0].clone();
    let mut win_call = calls[0];
    for call in calls {
        for board in 0..boards.len() {
            for row in 0..BOARDSIZE {
                for col in 0..BOARDSIZE {
                    if boards[board].numbers[row][col].value == *call {
                        boards[board].numbers[row][col].marked = true;

                        boards[board].rows[row] += 1;
                        if boards[board].rows[row] == BOARDSIZE && !boards[board].win {
                            boards[board].win = true;
                            win_board = boards[board].clone();
                            win_call = *call;
                        }

                        boards[board].cols[col] += 1;
                        if boards[board].cols[col] == BOARDSIZE && !boards[board].win {
                            boards[board].win = true;
                            win_board = boards[board].clone();
                            win_call = *call;
                        }
                    }
                }
            }
        }
    }

    (win_board, win_call)
}

fn challenge1(calls: &[usize], boards: &mut Vec<Board>) -> usize {
    let (board, call) = winning_board_call(calls, boards);
    let mut sum = 0;

    for row in &board.numbers {
        for col in row {
            if !col.marked {
                sum += col.value;
            }
        }
    }

    sum * call
}

fn challenge2(calls: &[usize], boards: &mut Vec<Board>) -> usize {
    let (board, call) = last_winning_board_call(calls, boards);
    let mut sum = 0;

    for row in &board.numbers {
        for col in row {
            if !col.marked {
                sum += col.value;
            }
        }
    }

    sum * call
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed reading input");
    let calls: Vec<usize> = input
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    input
        .lines()
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks(BOARDSIZE + 1)
        .for_each(|b| boards.push(Board::from(b)));

    // Challenge 1
    //println!("Challenge 1: {}", challenge1(calls.as_slice(), &mut boards));

    // Challenge 2
    println!("Challenge 2: {}", challenge2(calls.as_slice(), &mut boards));
}
