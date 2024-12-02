use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn run() -> io::Result<u32> {
    let file = File::open("src/day02/input1.txt")?;
    let reader = BufReader::new(file);
    let mut output = 0;

    for line in reader.lines() {
        let line = line?;
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

        if is_valid_sequence(&line_vec) {
            output += 1;
        }
    }

    println!("{}", output);
    Ok(output)
}

fn is_valid_sequence(line_vec: &[i32]) -> bool {
    if line_vec.len() < 2 {
        return false;
    }

    let mut is_increasing = true;
    let mut is_decreasing = true;

    for window in line_vec.windows(2) {
        let diff = window[1] - window[0];

        if !(1..=3).contains(&diff.abs()) {
            return false;
        }

        if diff > 0 {
            is_decreasing = false;
        } else if diff < 0 {
            is_increasing = false;
        }
    }

    is_increasing || is_decreasing
}
