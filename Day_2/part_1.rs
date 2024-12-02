use std::fs;

fn main() {
    let contents = fs::read_to_string("Day_2/input.txt")
        .expect("Something went wrong reading the file");

        let lines: Vec<&str> = contents.lines().collect();
        let mut safe_reports  = 0;

        for line in lines {
            let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            
            if numbers.windows(2).all(|w| w[0] < w[1] && (w[0] - w[1]).abs() <= 3) || 
                numbers.windows(2).all(|w| w[0] > w[1] && (w[0] - w[1].abs() <= 3)) {
                safe_reports += 1;
            }
            
            println!("{numbers:?}");
        }

        println!("{safe_reports}");
}