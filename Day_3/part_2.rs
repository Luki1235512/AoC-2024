use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("Day_3/input.txt")
        .expect("Something went wrong reading the file");

    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap(); 
    let mut sum = 0;

    let mut strings = Vec::new();

    let mut start = 0;
    if let Some(dont_mat) = dont_regex.find(&contents[start..]) {
        strings.push(&contents[start..dont_mat.start()]);
        start = dont_mat.end();
    }

    while let Some(do_mat) = do_regex.find(&contents[start..]) {
        if let Some(dont_mat) = dont_regex.find(&contents[start + do_mat.end()..]) {
            strings.push(&contents[start + do_mat.end()..start + do_mat.end() + dont_mat.start()]);
            start += do_mat.end() + dont_mat.end();
        } else {
            strings.push(&contents[start + do_mat.end()..]);
            break;
        }
    }

    for s in &strings {
        for cap in regex.captures_iter(s) {
            let num1: i32 = cap[1].parse().unwrap();
            let num2: i32 = cap[2].parse().unwrap();
            sum += num1 * num2;
        }
    }

    for s in &strings {
        println!("{:?}", s);
    }

    println!("{sum}");
}