use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("./input.txt").unwrap();
    let r = BufReader::new(f);
    let iter = r.lines().enumerate();
    let mut sums: Vec<i32> = vec![];
    let mut sum: i32 = 0;
    for (_, line) in iter {
        let l = line.unwrap_or("".to_string());
        if l == "" {
            sums.push(sum);
            sum = 0;
        } else {
            let v: i32 = l.parse().unwrap();
            sum += v;
        }
    }

    sums.sort();

    println!("max:   {}", sums[sums.len() - 1]);
    println!(
        "top 3: {}",
        sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3]
    );
}
