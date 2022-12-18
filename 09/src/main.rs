#![allow(dead_code, unused)]
use std::{
    cmp,
    collections::{HashSet, VecDeque},
    fs::File,
    io::Read,
    str::FromStr,
};

fn main() {
    println!(
        "example 01: {}",
        ex01(include_str!("input1.txt").to_string())
    );
    println!(
        "problem 01: {}",
        ex01(include_str!("../input.txt").to_string())
    );
    println!(
        "example 02: {}",
        ex02(include_str!("input2.txt").to_string())
    );
    println!(
        "problem 02: {}",
        ex02(include_str!("../input.txt").to_string())
    );
}

fn ex01(input: String) -> usize {
    let start = Point::new(0, 0);
    let (touched, max, min, rope) = process(start.clone(), 2, input);
    // print_matrix(max, min, start, rope, touched.clone());
    touched.len()
}

fn ex02(input: String) -> usize {
    let start = Point::new(0, 0);
    let (touched, max, min, rope) = process(start.clone(), 10, input);
    // print_matrix(max, min, start, rope, touched.clone());
    touched.len()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Movement {
    dir: Direction,
    count: usize,
}

impl Movement {
    fn new(dir: Direction, count: usize) -> Self {
        Self { dir, count }
    }
}

fn parse(input: &String) -> Vec<Movement> {
    return input
        .lines()
        .map(|line| {
            let (dir, count) = line.split_once(" ").expect("ok");
            return Movement {
                dir: Direction::from_str(dir).expect(dir),
                count: count.parse().expect(count),
            };
        })
        .collect();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    // pythagoras
    // f64::sqrt((self.x as f64 - other.x as f64).powi(2) + (self.y as f64 - other.y as f64).powi(2))

    fn chebychev_distance_to(&self, other: &Point) -> isize {
        cmp::max((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    fn is_close_to(&self, pos: &Point) -> bool {
        self.chebychev_distance_to(pos) <= 1
    }

    fn neighbour(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Right => Point::new(self.x + 1, self.y),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Up => Point::new(self.x, self.y + 1),
            Direction::Down => Point::new(self.x, self.y - 1),
        }
    }

    fn new_tail(head: &Self, tail: &Self) -> Self {
        // match dir {
        //     Direction::Right => Point::new(head.x - 1, head.y),
        //     Direction::Left => Point::new(head.x + 1, head.y),
        //     Direction::Up => Point::new(head.x, head.y - 1),
        //     Direction::Down => Point::new(head.x, head.y + 1),
        // }
        Self {
            x: tail.x + cmp::max(cmp::min(head.x - tail.x, 1), -1),
            y: tail.y + cmp::max(cmp::min(head.y - tail.y, 1), -1),
        }
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

fn process(
    start: Point,
    rope_len: usize,
    input: String,
) -> (HashSet<Point>, Point, Point, Vec<Point>) {
    let mut touched: HashSet<Point> = HashSet::new();
    let mut max = start.clone();
    let mut min = start.clone();
    let mut rope: Vec<Point> = vec![start; rope_len];

    for m in parse(&input).iter() {
        for _ in 0..m.count {
            rope[0] = rope[0].neighbour(&m.dir);
            max = Point::new(cmp::max(rope[0].x, max.x), cmp::max(rope[0].y, max.y));
            min = Point::new(cmp::min(rope[0].x, min.x), cmp::min(rope[0].y, min.y));
            for i in 0..rope_len - 1 {
                let head = rope[i];
                let tail = rope[i + 1];
                if !tail.is_close_to(&head) {
                    rope[i + 1] = Point::new_tail(&head, &tail);
                }
            }
            touched.insert(rope[rope_len - 1].clone());
        }
    }
    (touched, max, min, rope.into())
}

fn print_matrix(max: Point, min: Point, start: Point, rope: Vec<Point>, touched: HashSet<Point>) {
    println!();
    println!();
    for y in (min.y..max.y + 1).rev() {
        println!();
        for x in min.x..max.x + 1 {
            let pos = Point::new(x, y);
            if pos == start {
                print!("s");
            } else if rope[rope.len() - 1] == pos {
                print!("H");
            } else if rope[0] == pos {
                print!("T");
            } else if touched.contains(&pos) {
                print!("#");
            } else {
                print!(".")
            }
        }
    }
    println!();
    println!();
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn input_real() {
        let input = include_str!("../input.txt");
        assert_eq!(6190, ex01(input.to_string()));
        assert_eq!(2516, ex02(input.to_string()));
    }

    #[test]
    fn input_sample() {
        assert_eq!(13, ex01(include_str!("input1.txt").to_string()));
    }

    #[test]
    fn input_sample_02() {
        assert_eq!(1, ex02(include_str!("input1.txt").to_string()));
        assert_eq!(36, ex02(include_str!("input2.txt").to_string()));
    }

    #[test]
    fn is_pos_close() {
        vec![
            vec![Point::new(1, 1), Point::new(0, 0)],
            vec![Point::new(3, 1), Point::new(4, 1)],
            vec![Point::new(3, 0), Point::new(2, 0)],
        ]
        .iter()
        .for_each(|item| {
            let p1 = &item[0];
            let p2 = &item[1];
            assert_eq!(
                1,
                p1.chebychev_distance_to(p2),
                "should be 1, {}",
                p1.chebychev_distance_to(p2)
            );
            assert_eq!(
                true,
                p1.is_close_to(p2),
                "{} should be close to {}",
                p1.to_string(),
                p2.to_string()
            );
        });
    }

    #[test]
    fn is_pos_not_close() {
        vec![
            vec![Point::new(2, 1), Point::new(0, 0)],
            vec![Point::new(2, 0), Point::new(4, 1)],
            vec![Point::new(0, 0), Point::new(2, 0)],
        ]
        .iter()
        .for_each(|item| {
            let p1 = &item[0];
            let p2 = &item[1];
            assert_eq!(
                false,
                p1.is_close_to(p2),
                "{} should NOT be close to {}",
                p1.to_string(),
                p2.to_string()
            );
        });
    }

    #[test]
    fn test_parse() {
        let input = "U 1024\nD 21\nR 253\nL 12\nR 53\nD 999";
        let result = parse(&input.to_string());
        let expected = vec![
            Movement::new(Direction::Up, 1024),
            Movement::new(Direction::Down, 21),
            Movement::new(Direction::Right, 253),
            Movement::new(Direction::Left, 12),
            Movement::new(Direction::Right, 53),
            Movement::new(Direction::Down, 999),
        ];
        assert_eq!(
            true,
            expected
                .iter()
                .zip(result.clone())
                .all(|(a, b)| a.clone() == b.clone()),
            "expected {:?}, got {:?}",
            expected,
            result,
        );
    }

    #[test]
    fn reddit_comment() {
        let start = Point::new(0, 0);
        let (touched, max, min, rope) = process(start, 2, "R 1\nU 2".to_string());
        print_matrix(max, min, start, rope, touched.clone());
        assert_eq!(2, touched.len());
        assert_eq!(true, touched.contains(&Point::new(0, 0)));
        assert_eq!(true, touched.contains(&Point::new(1, 1)));
    }

    #[test]
    fn reddit_comment_2() {
        let start = Point::new(4, 0);
        let (touched, max, min, rope) = process(start.clone(), 2, "L 2".to_string());
        print_matrix(max, min, start, rope, touched.clone());
        assert_eq!(2, touched.len());
        assert_eq!(true, touched.contains(&Point::new(4, 0)));
        assert_eq!(true, touched.contains(&Point::new(3, 0)));
    }
}
