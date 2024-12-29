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
            update.iter().combinations(2).any(|r| {
                let (&x, &y) = (r[0], r[1]);
                criteria.iter().any(|&(before, after)| {
                    before == y
                        && after == x
                        && update.iter().position(|&p| p == before).unwrap()
                        > update.iter().position(|&p| p == after).unwrap()
                })
            })
        })
        .map(|update| {
            let mut sorted_update = update.clone();
            sorted_update.sort_by(|&a, &b| {
                let forward_rule = criteria.iter().find(|&&(x, y)| x == a && y == b);
                let reverse_rule = criteria.iter().find(|&&(x, y)| x == b && y == a);

                match (forward_rule, reverse_rule) {
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                }
            });

            sorted_update[sorted_update.len() / 2]
        }).sum();


    println!("{}", sum);
}