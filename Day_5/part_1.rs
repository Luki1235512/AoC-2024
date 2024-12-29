use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("Day_5/input.txt").unwrap();
    
    let sections: Vec<&str> = input.lines().collect();
    let split_lines: Vec<_> = sections.split(|l| l.is_empty()).collect();

    let criteria: Vec<(i32, i32)> = split_lines[0].iter().map(|line| {
        let l: Vec<&str> = line.split("|").collect();
        (l[0].parse().unwrap(), l[1].parse().unwrap())
    }).collect();

    let required_page_sets: Vec<Vec<i32>> = split_lines[1].iter().map(|line| line.split(',').map(|num| num.parse().unwrap()).collect()).collect();

    let sum: i32 = required_page_sets
        .iter()
        .filter(|update| {
            update.iter().combinations(2).all(|r| {
                let (&x, &y) = (r[0], r[1]);
                !criteria.iter().any(|&(before, after)| {
                    before == y
                        && after == x
                        && update.iter().position(|&p| p == before).unwrap()
                        > update.iter().position(|&p| p == after).unwrap()
                })
            })
        })
        .map(|update| update[update.len() / 2])
        .sum();

    println!("{}", sum);
}