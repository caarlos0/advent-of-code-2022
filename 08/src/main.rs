use std::{fs::File, io::Read};

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("works");
    println!("ex01: {}", ex01(buf));
}

fn ex01(input: String) -> usize {
    let lines: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse::<usize>().expect("ok"))
                .collect()
        })
        .collect();

    let count = lines.iter().count();
    let mut result = 0;
    for i in 0..count {
        let current = lines.get(i).expect("ok");
        for (j, char) in current.iter().enumerate() {
            if is_visible_left(current.to_owned(), j) {
                result = result + 1;
                continue;
            }
            if is_visible_right(current.to_owned(), j) {
                result = result + 1;
                continue;
            }
            if is_visible_up(lines.clone(), i, j) {
                result = result + 1;
                continue;
            }
            if is_visible_down(lines.clone(), i, j) {
                result = result + 1;
                continue;
            }
            println!("not visible ({},{}): {}", i, j, char);
        }
    }
    return result;
}

fn is_visible_up(matrix: Vec<Vec<usize>>, i: usize, j: usize) -> bool {
    let current = matrix.get(i).expect("ok").get(j).expect("ok");
    for ii in 0..i {
        if i == ii {
            continue;
        }
        if matrix.get(ii).expect("ok").get(j).expect("ok") >= current {
            return false;
        }
    }
    return true;
}
fn is_visible_down(matrix: Vec<Vec<usize>>, i: usize, j: usize) -> bool {
    let current = matrix.get(i).expect("ok").get(j).expect("ok");
    let count = matrix.iter().count();
    for ii in i..count {
        if i == ii {
            continue;
        }
        if matrix.get(ii).expect("ok").get(j).expect("ok") >= current {
            return false;
        }
    }
    return true;
}

fn is_visible_left(line: Vec<usize>, i: usize) -> bool {
    let current = line.get(i).expect("ok");
    for ii in 0..i {
        if i == ii {
            continue;
        }
        if line.get(ii).expect("ok") >= current {
            return false;
        }
    }
    return true;
}

fn is_visible_right(heights: Vec<usize>, i: usize) -> bool {
    let current = heights.get(i).expect("ok");
    let count = heights.iter().count();
    for ii in i..count {
        if i == ii {
            continue;
        }
        if heights.get(ii).expect("get i") >= current {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    #[test]
    fn input() {
        let input = include_str!("input1.txt");
        assert_eq!(21, super::ex01(input.to_string()));
    }

    #[test]
    fn input_valendo() {
        let input = include_str!("../input.txt");
        let result = super::ex01(input.to_string());
        assert_eq!(true, result > 1443, "got {}", result);
    }

    #[test]
    fn visible_up_err() {
        let matrix = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(false, super::is_visible_up(matrix, 1, 3))
    }

    // #[test]
    // fn is_visible_right() {
    //     assert_eq!(false, super::is_visible_right(vec![2, 5, 5, 1, 2], 1), "t1");
    //     assert_eq!(true, super::is_visible_right(vec![2, 5, 5, 2, 1], 2), "t2");
    // }
    //
    // #[test]
    // fn is_visible_left() {
    //     assert_eq!(true, super::is_visible_left(vec![2, 5, 5, 1, 2], 1));
    //     assert_eq!(true, super::is_visible_left(vec![4, 3, 5, 1, 2], 2));
    // }
    //
    // #[test]
    // fn is_visible_up() {
    //     let matrix = vec![vec![0, 1, 2], vec![1, 2, 2], vec![1, 3, 2]];
    //     assert_eq!(true, super::is_visible_up(matrix.clone(), 1, 1), "t1");
    //     assert_eq!(true, super::is_visible_up(matrix.clone(), 1, 2), "t2");
    //     assert_eq!(false, super::is_visible_up(matrix.clone(), 2, 1), "t3");
    // }
    //
    // #[test]
    // fn is_visible_down() {
    //     let matrix = vec![vec![0, 1, 2], vec![1, 2, 2], vec![1, 3, 2]];
    //     assert_eq!(false, super::is_visible_up(matrix.clone(), 1, 1), "t1");
    //     assert_eq!(false, super::is_visible_up(matrix.clone(), 1, 2), "t2");
    //     assert_eq!(true, super::is_visible_up(matrix.clone(), 2, 1), "t3");
    // }
}
