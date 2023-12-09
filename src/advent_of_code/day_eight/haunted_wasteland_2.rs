use std::collections::{HashMap, HashSet};

use super::file_reader::FileReader;

type NodeData = (String, (String, String));

fn parse_node_data_from_line(line: &str) -> NodeData {
    let split: Vec<&str> = line.split(['=', ',', ' ', '(', ')']).collect();
    let this_node = split.get(0).unwrap().to_string();
    let left_child = split.get(4).unwrap().to_string();
    let right_child = split.get(6).unwrap().to_string();
    (this_node, (left_child, right_child))
}

pub fn create_nodes_from_file(filename: &str) -> Nodes {
    let mut directions: Vec<char> = vec![];
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut start_nodes: HashSet<String> = HashSet::new();
    let file = FileReader::read(filename);
    for (line_num, line) in file.lines().enumerate() {
        match line_num {
            0 => directions = line.chars().collect(),
            1 => (),
            _ => {
                let (node, children) = parse_node_data_from_line(line);
                if node.ends_with('A') {
                    start_nodes.insert(node.clone());
                }
                map.insert(node, children);
            }
        }
    }
    Nodes {
        map,
        directions,
        start_nodes,
    }
}
pub struct Nodes {
    directions: Vec<char>,
    start_nodes: HashSet<String>,
    map: HashMap<String, (String, String)>,
}

impl Nodes {
    pub fn traverse_single(&self, start: &String) -> u64 {
        let mut next = start;
        let mut iters: u64 = 0;
        for i in 0.. {
            let next_node = self.map.get(next).unwrap();
            let dir = self.directions.get(i % self.directions.len()).unwrap();
            match dir {
                'L' => next = &next_node.0,
                'R' => next = &next_node.1,
                _ => (),
            }
            if next.ends_with('Z') {
                iters = 1 + i as u64;
                break;
            }
        }
        iters
    }

    pub fn traverse(&self) -> u64 {
        self.start_nodes
            .iter()
            .map(|start_node| self.traverse_single(start_node))
            .fold(1, |acc, curr| lcm(acc, curr))
    }
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_eight::haunted_wasteland_2::{
        create_nodes_from_file, lcm, parse_node_data_from_line,
    };

    #[test]
    fn parses_node_data_from_line() {
        let node_record = parse_node_data_from_line("AAA = (BBB, CCC)");

        let expected = ("AAA".to_string(), ("BBB".to_string(), "CCC".to_string()));
        assert_eq!(node_record, expected);
    }

    #[test]
    fn creates_nodes_hashmap_from_input_file() {
        let nodes = create_nodes_from_file("./src/advent_of_code/day_eight/test-input-2.txt");

        assert_eq!(nodes.directions, vec!['L', 'R']);
        assert!(nodes.start_nodes.contains(&"11A".to_string()));
        assert!(nodes.start_nodes.contains(&"22A".to_string()));
        assert_eq!(
            *nodes.map.get(&"11Z".to_string()).unwrap(),
            ("11B".to_string(), "XXX".to_string())
        );
    }

    #[test]
    fn traverses_from_all_a_nodes_to_all_z_nodes() {
        let nodes = create_nodes_from_file("./src/advent_of_code/day_eight/test-input-2.txt");

        let steps = nodes.traverse();

        assert_eq!(steps, 6);
    }

    #[test]
    fn calculates_lowest_common_multiply() {
        assert_eq!(lcm(15, 20), 60);
        assert_eq!(lcm(10, 15), 30);
        assert_eq!(lcm(11, 13), 143);
    }
}
