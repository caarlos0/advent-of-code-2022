use std::{fs::File, io::Read};

use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("works");
    println!("result 1: {}", part1(&buf));
    println!("result 2: {}", part2(&buf));
}

fn part1(input: &String) -> usize {
    let lines = input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    for line in lines {
        println!("{line:?}");
    }
    0
}

fn part2(_line: &String) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_part1() {
        // let input = include_str!("../input1.txt");
        // assert_eq!(10, from_str(input).len());
    }
}
