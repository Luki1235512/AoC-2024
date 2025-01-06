use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point {
            x: x as isize,
            y: y as isize,
        }
    }

    fn difference(&self, other: &Self) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct Map {
    graph: HashMap<Point, char>,
    frequencies: HashMap<char, Vec<Point>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut graph = HashMap::new();
        let mut frequencies: HashMap<char, Vec<Point>> = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = Point::new(x, y);
                graph.insert(point, c);
                if c == '.' {
                    continue;
                }
                frequencies.entry(c).or_default().push(point);
            }
        }
        Map { graph, frequencies }
    }

    fn process_frequency_pairs(
        &self,
        f: &mut dyn FnMut(&HashMap<Point, char>, &mut HashSet<Point>, &Point, &Point),
    ) -> HashSet<Point> {
        let mut antinodes = HashSet::new();
        for (_, nodes) in self.frequencies.iter() {
            nodes
                .iter()
                .cartesian_product(nodes.iter())
                .filter(|(a, b)| a != b)
                .for_each(|(a, b)| {
                    f(&self.graph, &mut antinodes, a, b);
                });
        }
        antinodes
    }
}

fn main() {
    let input = include_str!("input.txt");
    let map = Map::new(input);

    let answer = map.process_frequency_pairs(&mut |graph, antinodes, cur, node| {
        let l = node + node.difference(cur);
        if graph.get(&l).is_some() {
            antinodes.insert(l);
        }
        let r = cur + cur.difference(node);
        if graph.get(&r).is_some() {
            antinodes.insert(r);
        }
    });
    println!("{}", answer.len());
}