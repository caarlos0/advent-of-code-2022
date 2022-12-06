use std::{fs::File, io::Read, str::FromStr};

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("works");
    println!("result: {}", part1(buf));
}

fn part1(buf: String) -> String {
    let (crates, moves) = buf.split_once("\n\n").unwrap();

    let mut crane = Crane::from_str(crates).expect("no fail");
    let movements: Vec<Move> = moves
        .lines()
        .map(|l| Move::from_str(l).expect("success"))
        .collect();

    crane.apply(movements);

    println!("{:?}", crane.crates);

    crane.top()
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Self { count, from, to }
    }
}

impl FromIterator<usize> for Move {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        return Move {
            count: iter.next().expect("shut up"),
            from: iter.next().expect("shut up"),
            to: iter.next().expect("shut up"),
        };
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_whitespace()
            .flat_map(|c| c.parse::<usize>())
            .collect::<Move>())
    }
}

#[derive(Debug)]
struct Crane {
    crates: Vec<Vec<char>>,
}

impl Crane {
    fn apply(&mut self, movements: Vec<Move>) {
        for m in movements {
            for _ in 0..m.count {
                let item = self.crates[m.from - 1].pop().expect("thx bye");
                self.crates[m.to - 1].push(item);
                println!(
                    "moving {:?} from {} to {}, result: {:?}",
                    item, m.from, m.to, self.crates
                );
            }
        }
    }

    fn top(&mut self) -> String {
        let top: Vec<&char> = self
            .crates
            .iter()
            .map(|c| c.last().expect("yesp"))
            .collect();
        String::from_iter(top)
    }
}

impl FromStr for Crane {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{}", s);
        let mut crates = Vec::new();
        for line in s.lines().rev().skip(1) {
            let mut i = 0;
            let mut ci = 0;
            println!("{} {} {}", line, i, ci);
            while i < line.len() {
                if crates.len() <= ci {
                    crates.push(Vec::new())
                }
                if line[i..].starts_with("[") {
                    let c = line.chars().nth(i + 1).expect("yes man");
                    crates[ci].push(c)
                }
                i += 4;
                ci += 1;
            }
        }

        return Ok(Crane { crates });
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    #[test]
    fn parse_crane_test() {
        let input = include_str!("../input1.txt");
        let (crates, _) = input.split_once("\n\n").unwrap();
        assert_eq!(
            super::Crane::from_str(crates).expect("ok").crates,
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],]
        );
    }

    #[test]
    fn parse_moves_test() {
        let input = include_str!("../input1.txt");
        let (_, moves) = input.split_once("\n\n").unwrap();

        let parsed: Vec<super::Move> = moves
            .lines()
            .map(|m| super::Move::from_str(m).expect("success"))
            .collect();
        assert_eq!(
            parsed,
            vec![
                super::Move::new(1, 2, 1),
                super::Move::new(3, 1, 3),
                super::Move::new(2, 2, 1),
                super::Move::new(1, 1, 2),
            ]
        );
    }

    #[test]
    fn test() {
        let input = include_str!("../input1.txt");
        assert_eq!(super::part1(input.to_string()), "CMZ"); // "QNDWLMGNS");
    }
}
