use std::fs;

fn main() {
    let contents = fs::read_to_string("Day_1/input.txt")
        .expect("Something went wrong reading the file");

    let inputs: Vec<&str> = contents.split_whitespace().collect();
    println!("{inputs:?}");

    let mut left_array = Vec::new();
    let mut right_array = Vec::new();

    for (i, c) in inputs.iter().enumerate() {
        let num = c.parse::<i32>().expect("Invalid number");
        if i % 2 == 0 {
            left_array.push(num);
        } else {
            right_array.push(num);
        }
     }

    let mut similarity = Vec::new();

    for left in &left_array {
        let count = right_array.iter().filter(|&&right| right == *left).count() as i32;
        let result = left * count;
        similarity.push(result);
    }

     let sum: i32 = similarity.iter().sum();
     println!("{sum}");
}