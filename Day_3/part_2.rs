use std::fs;

fn main() {
    let contents = fs::read_to_string("Day_3/input.txt")
        .expect("Something went wrong reading the file");

        let lines: Vec<&str> = contents.lines().collect();

        println!("{lines:?}");
}