use std::{fs::File, io::Read, str::FromStr};

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("works");
    println!("result 1: {}", part1(&buf));
    println!("result 2: {}", part2(&buf));
}

fn part1(_line: &String) -> usize {
    unimplemented!()
}

fn part2(_line: &String) -> usize {
    unimplemented!()
}

#[derive(Debug)]
struct Node {
    name: String,
    size: usize,
    children: Vec<Node>,
    node_type: NodeType,
}

impl Node {
    fn new(name: String) -> Node {
        return Node {
            name,
            size: 0,
            children: Vec::new(),
            node_type: NodeType::Dir,
        };
    }

    fn new_file(name: String, size: usize) -> Node {
        return Node {
            name,
            size,
            children: Vec::new(),
            node_type: NodeType::File,
        };
    }

    fn add(&mut self, node: Node) {
        self.children.push(node);
    }
}

#[derive(Debug)]
enum NodeType {
    Dir,
    File,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut root = Node::new("/".to_string());
        let mut current = &mut root;
        for line in s.lines().skip(1) {
            if line == "$ ls" {
                continue;
            }
            if line.starts_with("$ cd ") {
                let (_, dir) = line.split_at(5);
                let mut node: Node = Node::new(dir.to_string());
                current.add(node);
                current = &mut node;
                continue;
            }
            println!("{}", line);
            let (type_or_size, name) = line.split_once(" ").expect("split in 2");
            if type_or_size == "dir" {
                current.add(Node::new(name.to_string()));
                continue;
            }
            let size: usize = type_or_size.parse().expect("ok");
            current.add(Node::new_file(name.to_string(), size));
        }

        Ok(root)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    #[test]
    fn test_part1() {
        let input = include_str!("../input1.txt");
        assert_eq!(3, super::Node::from_str(input).expect("yep").children.len());
    }
}
