use std::{fs::File, io::{BufRead, BufReader}};

pub fn run() -> u32 {
    let file = File::open("src/day02/input1.txt").expect("can't open file");
    let reader = BufReader::new(file);
    let mut output = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line_vec: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut pass = true;

        for i in 0..line_vec.len() - 1 {
            let diff = line_vec[i + 1] - line_vec[i];

            if diff.abs() < 1 || diff.abs() > 3 {
                pass = false;
                break;
            }

            if diff > 0 {
                is_decreasing = false;
            } else if diff < 0 {
                is_increasing = false;
            }
        }

        if !(is_increasing || is_decreasing) {
            pass = false;
        }

        if pass {
            output += 1;
        }
    }

    println!("{}", output);
    output
}
