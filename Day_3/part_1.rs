use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("Day_3/input.txt")
        .expect("Something went wrong reading the file");

    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for cap in regex.captures_iter(&contents) {
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();
        sum += num1 * num2;
    }

    println!("{sum}");
}