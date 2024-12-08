use std::fs;

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

fn parse_line(line: &str) -> Equation {
    let parts: Vec<&str> = line.split(':').collect();
    let test_value = parts[0].trim().parse().unwrap();
    let numbers: Vec<i64> = parts[1]
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    Equation { test_value, numbers }
}

fn evaluate(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];
    for i in 0..operators.len() {
        match operators[i] {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            '|' => result = concatenate(result, numbers[i  + 1]),
            _ => panic!(),
        }
    }

    result
}

fn try_combinations(equation: &Equation) -> bool {
    let operator_count = equation.numbers.len() - 1;
    let combinations = 3_i32.pow(operator_count as u32);

    for i in 0..combinations {
        let mut operators = Vec::new();
        let mut n = i;
        for _ in 0..operator_count {
            operators.push(match n % 3 {
                0 => '+',
                1 => '*',
                2 => '|',
                _ => unreachable!(),
            });
            n /= 3;
        }
        if evaluate(&equation.numbers, &operators) == equation.test_value {
            return true;
        }
    }

    false
}

fn concatenate(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}

fn main() {
    let input = fs::read_to_string("Day_7/input.txt").unwrap();
    let equations: Vec<Equation> = input.lines().map(parse_line).collect();

    let result: i64 = equations
        .iter()
        .filter(|eq| try_combinations(eq))
        .map(|eq| eq.test_value)
        .sum();

    println!("{}", result);
}