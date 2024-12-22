use std::fs;

fn main() {
    let input = fs::read_to_string("Day_9/input.txt").unwrap();

    let number_sequence: Vec<i32> = input
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

        let mut result: Vec<String> = Vec::new();
        let mut id_number: u32 = 0;
        let mut sum: i64 = 0;
    
        for (i, &num) in number_sequence.iter().enumerate() {
            let sequence = if i % 2 == 0 {
                let seq = vec![id_number.to_string(); num as usize];
                id_number += 1;
                seq
            } else {
                vec![".".to_string(); num as usize]
            };
    
            result.extend(sequence);
        }

        println!("{:?}", result);

        let mut i = 0;

        while i < result.len() {
            if result[i] == "." {

                let mut last_num_pos = result.len() - 1;

                while last_num_pos > i && result[last_num_pos] == "." {
                    last_num_pos -= 1;
                }
                result.swap(i, last_num_pos);
            }

            i += 1;
        }

        for (index, str_num) in result.iter_mut().enumerate() {
            if str_num != "." {
                if let Ok(number) = str_num.parse::<i64>() {
                    let multiplied = number * index as i64;
                    sum += multiplied;
                }
            }
        }

    println!("{:?}", result);
    println!("{:?}", sum);
}