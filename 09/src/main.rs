use std::{cmp, collections::HashSet, fs::File, io::Read, str::FromStr};

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("works");
    println!("ex01: {}", ex01(buf.clone()));
    println!("ex02: {}", ex02(buf.clone()));
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

fn parse(input: &String) -> Vec<Movement> {
    return input
        .lines()
        .map(|line| {
            let (dir, len) = line.split_once(" ").expect("ok");
            return Movement {
                dir: Direction::from_str(dir).expect("ok"),
                count: len.parse().expect("ok"),
            };
        })
        .collect();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Position {
        Position {
            x: cmp::max(x, 0),
            y: cmp::max(y, 0),
        }
    }

    fn chebychev_distance_to(&self, other: &Position) -> isize {
        // f64::sqrt((self.x as f64 - pos.x as f64).powi(2) + (self.y as f64 - pos.y as f64).powi(2))
        cmp::max((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    fn is_close_to(&self, pos: &Position) -> bool {
        self.chebychev_distance_to(pos) <= 1
    }

    fn neighbour(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Right => Position::new(self.x + 1, self.y),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Up => Position::new(self.x, self.y + 1),
            Direction::Down => Position::new(self.x, self.y - 1),
        }
    }

    fn new_tail(head: &Self, dir: &Direction) -> Self {
        match dir {
            Direction::Right => Position::new(head.x - 1, head.y),
            Direction::Left => Position::new(head.x + 1, head.y),
            Direction::Up => Position::new(head.x, head.y - 1),
            Direction::Down => Position::new(head.x, head.y + 1),
        }
    }
}

impl ToString for Position {
    fn to_string(&self) -> String {
        format!("{}x{}", self.x, self.y)
    }
}

fn ex02(input: String) -> usize {
    0
}

fn ex01(input: String) -> usize {
    let mut touched: HashSet<Position> = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut head = Position::new(0, 0);
    let mut tail = Position::new(0, 0);
    parse(&input).iter().for_each(|m| {
        for _ in 0..m.count {
            // let new_head = match m.dir {
            //     Direction::Right => Position::new(head.x + 1, head.y),
            //     Direction::Left => Position::new(head.x - 1, head.y),
            //     Direction::Up => Position::new(head.x, head.y + 1),
            //     Direction::Down => Position::new(head.x, head.y - 1),
            // };

            head = head.neighbour(&m.dir);
            max_x = cmp::max(head.x, max_x);
            max_y = cmp::max(head.y, max_y);
            if !tail.is_close_to(&head) {
                tail = Position::new_tail(&head, &m.dir);
                // tail = Position::new(
                //     tail.x + cmp::max(cmp::min(head.x - tail.x, 1), -1),
                //     tail.y + cmp::max(cmp::min(head.y - tail.y, 1), -1),
                // );
            }
            touched.insert(tail.clone());

            // println!("h:{} t:{}", head.to_string(), tail.to_string());
        }
    });

    println!();
    dbg!(touched.to_owned());
    dbg!(max_x);
    dbg!(max_y);
    for y in 0..max_y + 1 {
        println!();
        for x in 0..max_x + 1 {
            let pos = Position::new(x, y);
            if head == pos {
                print!("H");
            } else if tail == pos {
                print!("T");
            } else if touched.contains(&pos) {
                print!("o");
            } else {
                print!(".")
            }
        }
    }
    println!();
    touched.len()
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
            vec![Position::new(1, 1), Position::new(0, 0)],
            vec![Position::new(3, 1), Position::new(4, 1)],
            vec![Position::new(3, 0), Position::new(2, 0)],
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
            vec![Position::new(2, 1), Position::new(0, 0)],
            vec![Position::new(2, 0), Position::new(4, 1)],
            vec![Position::new(0, 0), Position::new(2, 0)],
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
}
