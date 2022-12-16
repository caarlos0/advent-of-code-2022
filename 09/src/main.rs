#![allow(dead_code, unused)]
use std::{cmp, collections::HashSet, fs::File, io::Read, str::FromStr};

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
        "problem 02: {}",
        ex02(include_str!("../input.txt").to_string())
    );
}

#[derive(Debug)]
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

#[derive(Debug)]
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
                dir: Direction::from_str(dir).expect("ok"),
                count: count.parse().expect("ok"),
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
        Point {
            x: cmp::max(x, 0),
            y: cmp::max(y, 0),
        }
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

    fn new_tail(head: &Self, dir: &Direction) -> Self {
        match dir {
            Direction::Right => Point::new(head.x - 1, head.y),
            Direction::Left => Point::new(head.x + 1, head.y),
            Direction::Up => Point::new(head.x, head.y - 1),
            Direction::Down => Point::new(head.x, head.y + 1),
        }
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

fn ex02(input: String) -> usize {
    0
}

fn execute(m: &Movement, head: &Point, tail: &Point) -> (Point, Point) {
    let head = head.neighbour(&m.dir);
    if !tail.is_close_to(&head) {
        return (head, Point::new_tail(&head, &m.dir));
    }
    (head, tail.clone())
}

fn ex01(input: String) -> usize {
    let mut touched: HashSet<Point> = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    for m in parse(&input).iter() {
        for _ in 0..m.count {
            (head, tail) = execute(m, &head, &tail);
            touched.insert(tail.clone());
            max_x = cmp::max(head.x, max_x);
            max_y = cmp::max(head.y, max_y);
            // println!("h:{} t:{}", head.to_string(), tail.to_string());
        }
    }

    println!();
    // dbg!(touched.to_owned());
    // dbg!(max_x);
    // dbg!(max_y);
    print_matrix(max_x, max_y, head, tail, touched.clone());

    touched.len()
}

fn print_matrix(max_x: isize, max_y: isize, head: Point, tail: Point, touched: HashSet<Point>) {
    println!();
    for y in (0..max_y + 1).rev() {
        println!();
        for x in 0..max_x + 1 {
            let pos = Point::new(x, y);
            if head == pos {
                print!("H");
            } else if tail == pos {
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
        let res = ex01(input.to_string());
        assert_eq!(true, res < 6000, "should be higher than 6000: {}", res,);
        assert_eq!(true, res > 5746, "should be higher than 5746: {}", res,);
        // assert_eq!(8, ex02(input.to_string()));
    }

    #[test]
    fn input_sample() {
        let input = include_str!("input1.txt");
        assert_eq!(13, ex01(input.to_string()));
        // assert_eq!(8, ex02(input.to_string()));
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
    fn tail_diag_two_steps() {
        let mut head = Point::new(2, 2);
        let mut tail = Point::new(1, 1);
        let m = Movement::new(Direction::Up, 1);
        (head, tail) = execute(&m, &head, &tail);
        assert_eq!(
            Point::new(2, 3),
            head,
            "should be in 2,3, is {}",
            head.to_string()
        );
        assert_eq!(
            Point::new(2, 2),
            tail,
            "should be in 2,2, is {}",
            tail.to_string()
        );
    }

    #[test]
    fn tail_hor_two_steps_rigth() {
        let mut head = Point::new(2, 1);
        let mut tail = Point::new(1, 1);
        let m = Movement::new(Direction::Right, 1);
        (head, tail) = execute(&m, &head, &tail);
        assert_eq!(
            Point::new(3, 1),
            head,
            "should be in 3,1, is {}",
            head.to_string()
        );
        assert_eq!(
            Point::new(2, 1),
            tail,
            "should be in 2,1, is {}",
            tail.to_string()
        );
    }

    #[test]
    fn tail_hor_two_steps_left() {
        let mut head = Point::new(1, 1);
        let mut tail = Point::new(2, 1);
        let m = Movement::new(Direction::Left, 1);
        (head, tail) = execute(&m, &head, &tail);
        assert_eq!(
            Point::new(0, 1),
            head,
            "should be in 0,1, is {}",
            head.to_string()
        );
        assert_eq!(
            Point::new(1, 1),
            tail,
            "should be in 1,1, is {}",
            tail.to_string()
        );
    }

    #[test]
    fn tail_hor_vert_steps_up() {
        let mut head = Point::new(1, 2);
        let mut tail = Point::new(1, 1);
        let m = Movement::new(Direction::Up, 1);
        (head, tail) = execute(&m, &head, &tail);
        assert_eq!(
            Point::new(1, 3),
            head,
            "should be in 1,3, is {}",
            head.to_string()
        );
        assert_eq!(
            Point::new(1, 2),
            tail,
            "should be in 1,2, is {}",
            tail.to_string()
        );
    }

    #[test]
    fn tail_hor_vert_steps_down() {
        let mut head = Point::new(1, 1);
        let mut tail = Point::new(1, 2);
        let m = Movement::new(Direction::Down, 1);
        (head, tail) = execute(&m, &head, &tail);
        assert_eq!(
            Point::new(1, 0),
            head,
            "should be in 1,0, is {}",
            head.to_string()
        );
        assert_eq!(
            Point::new(1, 1),
            tail,
            "should be in 1,1, is {}",
            tail.to_string()
        );
    }
}
