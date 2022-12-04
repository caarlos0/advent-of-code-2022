#![feature(iter_next_chunk)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn find_common(c1: &str, c2: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for i in c1.chars() {
        for j in c2.chars() {
            if i == j {
                result.push(i)
            }
        }
    }
    result.dedup();
    return result;
}

fn get_pri(c: &char) -> i32 {
    let p: HashMap<char, i32> = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);
    return p.get(c).unwrap().to_owned();
}

fn ex01() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let mut priority: i32 = 0;

    for line in r.lines() {
        let line = match line {
            Ok(v) => v,
            Err(_) => break,
        };

        let (c1, c2) = line.split_at(line.len() / 2);
        let common = find_common(c1, c2);
        for i in common.iter() {
            priority += get_pri(i);
        }
    }
    println!("score: {}", priority);
}

fn ex02() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let mut priority: i32 = 0;

    let mut sacks: Vec<String> = Vec::new();
    for line in r.lines() {
        let line = match line {
            Ok(v) => v,
            Err(_) => break,
        };
        sacks.push(line);
        if sacks.len() == 3 {
            let common = find_common(
                find_common(sacks.get(0).unwrap(), sacks.get(1).unwrap())
                    .iter()
                    .cloned()
                    .collect::<String>()
                    .as_ref(),
                sacks.get(2).unwrap(),
            );
            for i in common.iter() {
                priority += get_pri(i);
            }

            sacks = Vec::new()
        }
    }
    println!("score: {}", priority);
}

fn main() {
    ex01();
    ex02();
}
