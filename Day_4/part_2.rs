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

    let word = "MAS";
    let count = count_x_shape(&matrix, word);

    println!("{count}");
}

fn count_x_shape(matrix: &Vec<Vec<char>>, word: &str) -> usize {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut count = 0;
    let word_chars: Vec<char> = word.chars().collect();

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if (
                matrix[i][j] == word_chars[1] &&
                matrix[i - 1][j - 1] == word_chars[0] &&
                matrix[i - 1][j + 1] == word_chars[2] &&
                matrix[i + 1][j - 1] == word_chars[0] &&
                matrix[i + 1][j + 1] == word_chars[2]
            ) ||
            (
                matrix[i][j] == word_chars[1] &&
                matrix[i - 1][j - 1] == word_chars[2] &&
                matrix[i - 1][j + 1] == word_chars[0] &&
                matrix[i + 1][j - 1] == word_chars[2] &&
                matrix[i + 1][j + 1] == word_chars[0] 
            ) ||
            (
                matrix[i][j] == word_chars[1] &&
                matrix[i - 1][j - 1] == word_chars[0] &&
                matrix[i - 1][j + 1] == word_chars[0] &&
                matrix[i + 1][j - 1] == word_chars[2] &&
                matrix[i + 1][j + 1] == word_chars[2] 
            ) ||
            (
                matrix[i][j] == word_chars[1] &&
                matrix[i - 1][j - 1] == word_chars[2] &&
                matrix[i - 1][j + 1] == word_chars[2] &&
                matrix[i + 1][j - 1] == word_chars[0] &&
                matrix[i + 1][j + 1] == word_chars[0] 
            )
            {
                count += 1;
            }
        }
    }

    count
}