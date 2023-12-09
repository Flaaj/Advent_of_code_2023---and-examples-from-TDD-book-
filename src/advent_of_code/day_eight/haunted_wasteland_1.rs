use std::collections::HashMap;

use super::file_reader::FileReader;

type NodeData = (String, (String, String));

fn parse_node_data_from_line(line: &str) -> NodeData {
    let split: Vec<&str> = line.split(['=', ',', ' ', '(', ')']).collect();
    let this_node = split.get(0).unwrap().to_string();
    let left_child = split.get(4).unwrap().to_string();
    let right_child = split.get(6).unwrap().to_string();
    (this_node, (left_child, right_child))
}

pub struct Nodes {
    directions: Vec<char>,
    map: HashMap<String, (String, String)>,
}

impl Nodes {
    pub fn traverse(&self) -> u64 {
        let mut next = &"AAA".to_string();
        let mut iters: u64 = 0;
        for i in 0.. {
            let next_node = self.map.get(next).unwrap();
            let dir = self.directions.get(i % self.directions.len()).unwrap();
            match dir {
                'L' => next = &next_node.0,
                'R' => next = &next_node.1,
                _ => (),
            }
            if next == &"ZZZ".to_string() {
                iters = 1 + i as u64;
                break;
            }
        }
        iters
    }
}

pub fn create_nodes_from_file(filename: &str) -> Nodes {
    let mut directions: Vec<char> = vec![];
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let file = FileReader::read(filename);
    let lines: Vec<_> = file.lines().collect();
    for (line_num, line) in lines.iter().enumerate() {
        match line_num {
            0 => directions = line.chars().collect(),
            1 => (),
            _ => {
                let node_data = parse_node_data_from_line(line);
                map.insert(node_data.0, node_data.1);
            }
        }
    }
    Nodes { map, directions }
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_eight::haunted_wasteland::{
        create_nodes_from_file, parse_node_data_from_line,
    };

    #[test]
    fn parses_node_data_from_line() {
        let node_record = parse_node_data_from_line("AAA = (BBB, CCC)");

        let expected = ("AAA".to_string(), ("BBB".to_string(), "CCC".to_string()));
        assert_eq!(node_record, expected);
    }

    #[test]
    fn creates_nodes_hashmap_from_input_file() {
        let nodes = create_nodes_from_file("./src/advent_of_code/day_eight/test-input-1.txt");

        assert_eq!(nodes.directions, vec!['R', 'L']);
        assert_eq!(
            nodes.map.get(&"CCC".to_string()).unwrap().clone(),
            ("ZZZ".to_string(), "GGG".to_string())
        );
    }

    #[test]
    fn traverses_from_start_to_end_node() {
        let nodes = create_nodes_from_file("./src/advent_of_code/day_eight/test-input-1.txt");

        let steps = nodes.traverse();

        assert_eq!(steps, 2);
    }
}
