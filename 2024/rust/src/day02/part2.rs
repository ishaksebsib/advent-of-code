use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() -> u32 {
    let file = File::open("src/day02/input1.txt").expect("can't read the file");
    let reader = BufReader::new(file);
    let mut output = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line_vec: Result<Vec<i32>, _> = line
            .split_whitespace()
            .map(str::parse)
            .collect();

        let line_vec = match line_vec {
            Ok(vec) => vec,
            Err(_) => {
                eprintln!("Skipping invalid line: {}", line);
                continue;
            }
        };

        // First check if the line is safe without removing any levels
        if is_valid_sequence(&line_vec) {
            output += 1;
        } else {
            for i in 0..line_vec.len() {
                let mut remove_one = line_vec.clone();
                remove_one.remove(i);

                if is_valid_sequence(&remove_one) {
                    output += 1;
                    break;
                }
            }
        }
    }

    println!("{}", output);
    output
}

fn is_valid_sequence(line_vec: &[i32]) -> bool {
    if line_vec.len() < 2 {
        return false;
    }

    let mut is_increasing = true;
    let mut is_decreasing = true;

    for window in line_vec.windows(2) {
        let diff = window[1] - window[0];

        // Check if the difference is within the valid range of 1 to 3
        if !(1..=3).contains(&diff.abs()) {
            return false;
        }

        // Check if the sequence is increasing or decreasing
        if diff > 0 {
            is_decreasing = false;
        } else if diff < 0 {
            is_increasing = false;
        }
    }

    is_increasing || is_decreasing
}
