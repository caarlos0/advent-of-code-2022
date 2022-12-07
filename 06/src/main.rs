use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("works");
    println!("result 1: {}", part1(&buf));
    println!("result 2: {}", part2(&buf));
}

fn part1(line: &String) -> usize {
    find(line, 4)
}

fn part2(line: &String) -> usize {
    find(line, 14)
}

fn find(line: &String, x: usize) -> usize {
    let size = line.chars().count();
    for i in 0..size - x - 1 {
        let l = match line.get(i..i + x) {
            Some(expr) => expr,
            None => break,
        }
        .chars()
        .collect::<HashSet<char>>()
        .len();
        if l == x {
            return i + x;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn test_part1() {
        for (k, v) in HashMap::from([
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ])
        .into_iter()
        {
            assert_eq!(super::part1(&k.to_string()), v);
        }
    }

    #[test]
    fn test_part2() {
        for (k, v) in HashMap::from([
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ])
        .into_iter()
        {
            assert_eq!(super::part2(&k.to_string()), v);
        }
    }
}
