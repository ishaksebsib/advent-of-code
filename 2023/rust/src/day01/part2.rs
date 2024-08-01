use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() -> u32 {
    let input = File::open("src/day01/input2.txt").unwrap();

    let reader = BufReader::new(input);

    let sum = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut it = (0..line.len()).filter_map(|i| {
                let reduced_line = &line[i..];
                let result = if reduced_line.starts_with("one") {
                    '1'
                } else if reduced_line.starts_with("two") {
                    '2'
                } else if reduced_line.starts_with("three") {
                    '3'
                } else if reduced_line.starts_with("four") {
                    '4'
                } else if reduced_line.starts_with("five") {
                    '5'
                } else if reduced_line.starts_with("six") {
                    '6'
                } else if reduced_line.starts_with("seven") {
                    '7'
                } else if reduced_line.starts_with("eight") {
                    '8'
                } else if reduced_line.starts_with("nine") {
                    '9'
                } else if reduced_line.starts_with("zero") {
                    '0'
                } else {
                    reduced_line.chars().next().unwrap()
                };

                result.to_digit(10)
            });

            let first_digit = it.next().unwrap();

            match it.last() {
                Some(last_digit) => format!("{first_digit}{last_digit}"),
                None => format!("{first_digit}{first_digit}"),
            }
            .parse::<u32>()
            .expect("string should be int")
        })
        .sum::<u32>();

    println!("Day 1 Part 2: {}", sum);
    sum
}
