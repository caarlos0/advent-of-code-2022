use std::{fs::File, io::Read};

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("works");
    println!("ex01: {}", ex01(buf.clone()));
    println!("ex01: {}", ex02(buf.clone()));
}

fn ex02(input: String) -> usize {
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
            let left = count_visible_left(current.to_owned(), j);
            let right = count_visible_right(current.to_owned(), j);
            let up = count_visible_up(lines.to_owned(), i, j);
            let down = count_visible_down(lines.to_owned(), i, j);
            let score = left * right * up * down;
            println!(
                "score for ({},{}) {} = ({}*{}*{}*{}) = {}",
                i, j, char, up, left, down, right, score
            );
            if score > result {
                result = score;
            }
        }
    }
    return result;
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

fn count_visible_up(matrix: Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    let mut result = 0;
    let current = matrix.get(i).expect("ok").get(j).expect("ok");
    if i == 0 {
        return 0;
    }
    for ii in (0..i).rev() {
        result = result + 1;
        let got = matrix.get(ii).expect("ok").get(j).expect("ok");
        if got >= current {
            break;
        }
    }
    return result;
}

fn count_visible_down(matrix: Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    let mut result = 0;
    let current = matrix.get(i).expect("ok").get(j).expect("ok");
    let count = matrix.iter().count();
    if i == count {
        return 0;
    }
    for ii in i + 1..count {
        result = result + 1;
        let got = matrix.get(ii).expect("ok").get(j).expect("ok");
        if got >= current {
            break;
        }
    }
    return result;
}

fn count_visible_left(line: Vec<usize>, i: usize) -> usize {
    let mut result = 0;
    let current = line.get(i).expect("ok");
    if i == 0 {
        return 0;
    }
    for ii in (0..i).rev() {
        result = result + 1;
        let got = line.get(ii).expect("ok");
        if got >= current {
            break;
        }
    }
    return result;
}

fn count_visible_right(line: Vec<usize>, i: usize) -> usize {
    let mut result = 0;
    let current = line.get(i).expect("ok");
    let count = line.iter().count();
    if i == count {
        return 0;
    }
    for ii in i + 1..count {
        result = result + 1;
        let got = line.get(ii).expect("ok");
        if got >= current {
            break;
        }
    }
    return result;
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

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn input() {
        let input = include_str!("input1.txt");
        assert_eq!(21, ex01(input.to_string()));
        assert_eq!(8, ex02(input.to_string()));
    }

    #[test]
    fn count_visible_left_bug() {
        let line = vec![3, 3, 5, 4, 9];
        assert_eq!(0, count_visible_left(line.clone(), 0));
        assert_eq!(1, count_visible_left(line.clone(), 1));
        assert_eq!(2, count_visible_left(line.clone(), 2));
        assert_eq!(1, count_visible_left(line.clone(), 3));
        assert_eq!(4, count_visible_left(line.clone(), 4));
    }

    #[test]
    fn test_count_visible_right() {
        let line = vec![3, 3, 5, 4, 9];
        assert_eq!(1, count_visible_right(line.clone(), 0));
        assert_eq!(1, count_visible_right(line.clone(), 1));
        assert_eq!(2, count_visible_right(line.clone(), 2));
        assert_eq!(1, count_visible_right(line.clone(), 3));
        assert_eq!(0, count_visible_right(line.clone(), 4));
    }

    // #[test]
    // fn input_valendo_01() {
    //     let input = include_str!("../input.txt");
    //     let result =ex01(input.to_string());
    //     assert_eq!(true, result > 1443, "got {}", result);
    // }
    //
    #[test]
    fn input_valendo_02() {
        let input = include_str!("../input.txt");
        let result = ex02(input.to_string());
        assert_eq!(true, result < 1982880, "got {}", result);
        assert_eq!(true, result > 74200, "got {}", result);
        assert_eq!(true, result != 740250, "got {}", result);
    }
    //
    // #[test]
    // fn visible_up_err() {
    //     let matrix = vec![
    //         vec![3, 0, 3, 7, 3],
    //         vec![2, 5, 5, 1, 2],
    //         vec![6, 5, 3, 3, 2],
    //         vec![3, 3, 5, 4, 9],
    //         vec![3, 5, 3, 9, 0],
    //     ];
    //     assert_eq!(false, is_visible_up(matrix, 1, 3))
    // }

    // #[test]
    // fn is_visible_right() {
    //     assert_eq!(false, is_visible_right(vec![2, 5, 5, 1, 2], 1), "t1");
    //     assert_eq!(true, is_visible_right(vec![2, 5, 5, 2, 1], 2), "t2");
    // }
    //
    // #[test]
    // fn is_visible_left() {
    //     assert_eq!(true, is_visible_left(vec![2, 5, 5, 1, 2], 1));
    //     assert_eq!(true, is_visible_left(vec![4, 3, 5, 1, 2], 2));
    // }
    //
    // #[test]
    // fn is_visible_up() {
    //     let matrix = vec![vec![0, 1, 2], vec![1, 2, 2], vec![1, 3, 2]];
    //     assert_eq!(true, is_visible_up(matrix.clone(), 1, 1), "t1");
    //     assert_eq!(true, is_visible_up(matrix.clone(), 1, 2), "t2");
    //     assert_eq!(false, is_visible_up(matrix.clone(), 2, 1), "t3");
    // }
    //
    // #[test]
    // fn is_visible_down() {
    //     let matrix = vec![vec![0, 1, 2], vec![1, 2, 2], vec![1, 3, 2]];
    //     assert_eq!(false, is_visible_up(matrix.clone(), 1, 1), "t1");
    //     assert_eq!(false, is_visible_up(matrix.clone(), 1, 2), "t2");
    //     assert_eq!(true, is_visible_up(matrix.clone(), 2, 1), "t3");
    // }
}
