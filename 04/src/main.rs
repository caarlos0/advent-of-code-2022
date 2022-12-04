use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn to_vec(s: String) -> Vec<i32> {
    let (lower, upper) = s.split_once("-").unwrap();
    let l: i32 = lower.parse().unwrap();
    let u: i32 = upper.parse().unwrap();
    let mut result: Vec<i32> = Vec::new();
    let mut i = l;
    while i <= u {
        result.push(i);
        i += 1;
    }
    return result;
}

fn ex01() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let mut count: i32 = 0;

    for line in r.lines() {
        let line = match line {
            Ok(v) => v,
            Err(_) => break,
        };

        let (s1, s2) = line.split_once(",").unwrap();
        let r1 = to_vec(s1.to_string());
        let r2 = to_vec(s2.to_string());

        if contains(&r1, &r2) || contains(&r2, &r1) {
            count += 1;
        }
    }
    println!("count: {}", count);
}

fn contains(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    for i in a {
        if !b.contains(&i) {
            return false;
        }
    }
    return true;
}

fn contains_any(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    for i in a {
        if b.contains(&i) {
            return true;
        }
    }
    return false;
}

fn ex02() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let mut count: i32 = 0;

    for line in r.lines() {
        let line = match line {
            Ok(v) => v,
            Err(_) => break,
        };

        let (s1, s2) = line.split_once(",").unwrap();
        let r1 = to_vec(s1.to_string());
        let r2 = to_vec(s2.to_string());

        if contains_any(&r1, &r2) || contains_any(&r2, &r1) {
            count += 1;
        }
    }
    println!("count: {}", count);
}

fn main() {
    ex01();
    ex02();
}
