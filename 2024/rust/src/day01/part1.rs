use std::{fs:: File, io::{BufRead, BufReader}};

pub fn run() -> i32 {
    let input = File::open("src/day01/input1.txt").expect("can't open the file");
    let reader = BufReader::new(input);

    let mut left:Vec<i32> = Vec::new();
    let mut right:Vec<i32> = Vec::new();

    for i in reader.lines() {
        let line = i.unwrap();
        let line_vec: Vec<&str> = line.split(" ").collect();
        let (l,r) = (line_vec[0].parse::<i32>().unwrap(),line_vec[line_vec.len()-1].parse::<i32>().unwrap());
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let output:i32 = left.iter().zip(right.iter()).map(|(l,r)| (l-r).abs()).sum();

    println!("{:?}",output);
    output
}
