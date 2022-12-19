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
        ex01(include_str!("input.txt").to_string())
    );
    println!(
        "example 02: {}",
        ex02(include_str!("input2.txt").to_string())
    );
    println!(
        "problem 02: {}",
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

fn ex02(input: String) -> usize {
    unimplemented!()
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Noop,
    Addx,
}

#[derive(Debug, PartialEq, Eq)]
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
}

impl CPU {
    fn new() -> Self {
        Self { cycles: vec![1] }
    }

    fn apply(&mut self, op: Op) {
        match op.kind {
            Type::Noop => self.cycles.push(self.x()),
            Type::Addx => {
                self.cycles.push(self.x());
                self.cycles.push(op.x + self.x());
            }
        }
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
}
