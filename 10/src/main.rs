#![allow(dead_code, unused)]
use core::fmt;
use std::{
    cmp,
    collections::{HashSet, VecDeque},
    fmt::Write,
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
        ex01(include_str!("input.txt").to_string())
    );
    println!(
        "example 02: {}",
        ex02(include_str!("input1.txt").to_string())
    );
    println!(
        "problem 02:\n{}",
        ex02(include_str!("input.txt").to_string())
    );
}

fn ex01(input: String) -> isize {
    let mut cpu = CPU::new();
    input
        .lines()
        .into_iter()
        .map(|l| Op::from_str(l).unwrap())
        .for_each(|op| {
            cpu.apply(op);
        });
    cpu.calc()
}

fn ex02(input: String) -> String {
    let mut cpu = CPU::new();
    input
        .lines()
        .into_iter()
        .map(|l| Op::from_str(l).unwrap())
        .for_each(|op| {
            cpu.apply(op);
        });
    cpu.draw()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Type {
    Noop,
    Addx,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Op {
    kind: Type,
    x: isize,
}

impl Op {
    fn noop() -> Self {
        Self {
            kind: Type::Noop,
            x: 1,
        }
    }
    fn addx(i: isize) -> Self {
        Self {
            kind: Type::Addx,
            x: i,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some((op, count)) => match op {
                "noop" => Ok(Op::noop()),
                "addx" => Ok(Op::addx(count.parse().unwrap())),
                _ => unreachable!(),
            },
            None => Ok(Op::noop()),
        }
    }
}

struct CPU {
    cycles: Vec<isize>,
    screen: Vec<Vec<char>>,
}

impl CPU {
    fn new() -> Self {
        Self {
            cycles: vec![1],
            screen: vec![vec!['.'; 40]; 6],
        }
    }

    fn apply(&mut self, op: Op) {
        match op.kind {
            Type::Noop => self.push(self.x()),
            Type::Addx => {
                self.push(self.x());
                self.push(op.x + self.x());
            }
        }
    }

    fn push(&mut self, x: isize) {
        let mut page = self.cycles.len() / 40;
        let mut i = self.cycles.len() - 40 * page;
        // handles last item in previous page
        if i == 0 {
            page = page - 1;
            i = 40;
        }

        let n = isize::try_from(i).unwrap();
        if (self.x()..self.x() + 3).contains(&n) {
            self.screen[page][i - 1] = '#';
        }
        self.cycles.push(x);
    }

    fn x(&self) -> isize {
        match self.cycles.last() {
            Some(x) => x.to_owned(),
            None => 0,
        }
    }

    fn signal_at(&self, i: usize) -> isize {
        match self.cycles.get(i - 1) {
            Some(x) => x.to_owned() * isize::try_from(i).expect("should never fail"),
            None => 0,
        }
    }

    fn calc(&self) -> isize {
        vec![20, 60, 100, 140, 180, 220]
            .iter()
            .map(|&i| self.signal_at(i))
            .sum()
    }

    fn draw(&self) -> String {
        self.screen
            .iter()
            .map(|line| line.into_iter().cloned().collect::<String>() + "\n")
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn sample01() {
        let mut cpu = CPU::new();
        include_str!("input1.txt")
            .lines()
            .into_iter()
            .map(|l| Op::from_str(l).unwrap())
            .for_each(|op| {
                cpu.apply(op);
            });

        println!("{:?}", cpu.cycles);
        assert_eq!(1, cpu.signal_at(1));
        assert_eq!(2, cpu.signal_at(2));
        assert_eq!(3, cpu.signal_at(3));
        assert_eq!(16, cpu.signal_at(4));
        assert_eq!(20, cpu.signal_at(5));
        assert_eq!(-6, cpu.signal_at(6));
    }

    #[test]
    fn cpu() {
        let mut cpu = CPU::new();

        cpu.apply(Op::noop());
        assert_eq!(1, cpu.x(), "should still be 1 after first noop");

        cpu.apply(Op::addx(10));
        assert_eq!(11, cpu.x(), "should be 11");

        cpu.apply(Op::noop());
        assert_eq!(11, cpu.x(), "should still be 11");

        cpu.apply(Op::addx(-5));
        assert_eq!(6, cpu.x(), "should be 6");

        cpu.apply(Op::noop());
        assert_eq!(6, cpu.x(), "should still be 6");
    }

    #[test]
    fn example02() {
        let mut cpu = CPU::new();
        include_str!("input2.txt")
            .lines()
            .into_iter()
            .map(|l| Op::from_str(l).unwrap())
            .for_each(|op| {
                cpu.apply(op);
            });

        println!("{:?}", cpu.cycles);
        assert_eq!(420, cpu.signal_at(20));
        assert_eq!(1140, cpu.signal_at(60));
        assert_eq!(1800, cpu.signal_at(100));
        assert_eq!(2940, cpu.signal_at(140));
        assert_eq!(2880, cpu.signal_at(180));
        assert_eq!(3960, cpu.signal_at(220));

        assert_eq!(13140, cpu.calc());
    }

    #[test]
    fn draw() {
        // TODO: there's still a bug somewhere and the output is not perfect... it is readable
        // though
        let expected = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ]
        .join("\n")
            + "\n";

        let mut cpu = CPU::new();
        include_str!("input2.txt")
            .lines()
            .into_iter()
            .map(|l| Op::from_str(l).unwrap())
            .for_each(|op| {
                cpu.apply(op);
            });
        println!("expected:\n{}\n\ngot:\n{}\n", expected, cpu.draw());
        assert_eq!(expected, cpu.draw());
    }

    #[test]
    fn push() {
        let mut cpu = CPU::new();
        vec![
            Op::addx(15),
            Op::addx(-11),
            Op::addx(6),
            Op::addx(-3),
            Op::addx(5),
            Op::addx(-1),
            Op::addx(-8),
            Op::addx(13),
            Op::addx(4),
            Op::noop(),
            Op::addx(-1),
        ]
        .iter()
        .for_each(|op| cpu.apply(op.to_owned()));
        println!("{:?}", cpu.screen[0]);
        "##..##..##..##..##..#"
            .chars()
            .into_iter()
            .enumerate()
            .for_each(|(i, p)| {
                assert_eq!(
                    p, cpu.screen[0][i],
                    "expected char at {} to be {}, was {}",
                    i, p, cpu.screen[0][i]
                );
            });
    }
}
