use std::fs;

fn main() {
    let contents = fs::read_to_string("Day_4/input.txt")
        .expect("Something went wrong reading the file");

    let matrix: Vec<Vec<char>> = contents.lines()
        .map(|line| line.chars().collect())
        .collect();
    
    for row in &matrix {
        println!("{row:?}");
    }

    let word = "XMAS";
    let count = count_word(&matrix, word);

    println!("{count}");
}

fn count_word(matrix: &Vec<Vec<char>>, word: &str) -> usize {
    let word_len = word.len();
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut count = 0;
    let word_chars: Vec<char> = word.chars().collect();

    // Horizontal
    for row in 0..rows {
        for col in 0..=cols - word_len {
            if matrix[row][col..col + word_len] == word_chars[..] || matrix[row][col..col + word_len].iter().rev().cloned().collect::<Vec<char>>() == word_chars {
                count += 1;
            }
        }
    }

    // Vertical
    for col in 0..cols {
        for row in 0..=rows - word_len {
            let vertical: Vec<char> = (0..word_len).map(|i| matrix[row + i][col]).collect();
            if vertical == word_chars || vertical.iter().rev().cloned().collect::<Vec<char>>() == word_chars {
                count += 1;
            }
        }
    }

    // Diagonal (top-left to bottom-right)
    for row in 0..=rows - word_len {
        for col in 0..=cols - word_len {
            let diagonal: Vec<char> = (0..word_len).map(|i| matrix[row + i][col + i]).collect();
            if diagonal == word_chars || diagonal.iter().rev().cloned().collect::<Vec<char>>() == word_chars {
                count += 1;
            }
        }
    }

    // Diagonal (top-right to bottom-left)
    for row in 0..=rows - word_len {
        for col in (word_len - 1)..cols {
            let diagonal: Vec<char> = (0..word_len).map(|i| matrix[row + i][col - i]).collect();
            if diagonal == word_chars || diagonal.iter().rev().cloned().collect::<Vec<char>>() == word_chars {
                count += 1;
            }
        }
    }

    count
}