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

     left_array.sort();
     right_array.sort();

     println!("{left_array:?}");
     println!("{right_array:?}");

     let mut differences = Vec::new();

     for (left, right) in left_array.iter().zip(right_array.iter()) {
        differences.push((left - right).abs());
     }

     println!("{differences:?}");

     let sum: i32 = differences.iter().sum();
     println!("{sum}");

}