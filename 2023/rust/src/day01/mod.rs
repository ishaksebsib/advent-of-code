use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() -> u32 {
    let input = File::open("src/day01/input1.txt").unwrap();

    let reader = BufReader::new(input);

    let output = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut only_digit = line.chars().filter_map(|character| character.to_digit(10));
            let first_digit = only_digit.next().expect("first digit");

            match only_digit.last() {
                Some(last_digit) => format!("{first_digit}{last_digit}"),
                None => format!("{first_digit}{first_digit}"),
            }
            .parse::<u32>()
            .expect("string should be int")
        })
        .sum::<u32>();

    println!("Day 1: {}", output);
    output
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_run() {
        assert_eq!(run(), 54940);
    }
}
