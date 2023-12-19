use std::collections::HashSet;

use super::file_reader::read_file;

type Node = (u8, u8);

type Direction = (i32, i32);
const LEFT: Direction = (0, -1);
const UP: Direction = (-1, 0);
const RIGHT: Direction = (0, 1);
const DOWN: Direction = (1, 0);
const DIRECTIONS: [Direction; 4] = [LEFT, UP, RIGHT, DOWN];

#[derive(PartialEq, Debug)]
struct Maze {
    start: Node,
    rows: Vec<Vec<char>>,
}

impl From<String> for Maze {
    fn from(string: String) -> Self {
        Self::new(string)
    }
}

impl Maze {
    fn new(string: String) -> Self {
        let mut start: Option<Node> = None;
        let rows = string
            .lines()
            .enumerate()
            .map(|(i, row)| {
                let chars = row.chars().into_iter();
                match chars.clone().enumerate().find(|(_, b)| *b == 'S') {
                    Some((j, _)) => start = Some((i as u8, j as u8)),
                    None => (),
                }
                chars.collect()
            })
            .collect();
        Self {
            rows,
            start: start.unwrap(),
        }
    }

    fn get_starting_nodes(&self) -> Vec<Node> {
        let len_y = self.rows.len() as i32;
        let len_x = self.rows.get(0).unwrap().len() as i32;
        let mut starting_nodes: Vec<Node> = vec![];
        let (y, x) = self.start;
        for (dy, dx) in DIRECTIONS {
            let y = y as i32 + dy;
            let x = x as i32 + dx;
            if x >= 0 && y >= 0 && x < len_x && y < len_y {
                starting_nodes.push((y as u8, x as u8));
            }
        }
        starting_nodes
    }

    fn get_char_at(&self, node: &Node) -> Option<char> {
        let row = self.rows.get(node.0 as usize)?;
        let c = row.get(node.1 as usize)?;
        Some(*c)
    }

    fn get_directions(&self, c: char) -> Option<[Direction; 2]> {
        match c {
            'J' => Some([UP, LEFT]),
            'L' => Some([UP, RIGHT]),
            '7' => Some([DOWN, LEFT]),
            'F' => Some([DOWN, RIGHT]),
            '|' => Some([UP, DOWN]),
            '-' => Some([LEFT, RIGHT]),
            _ => None,
        }
    }

    fn get_end_nodes(&self, node: &Node) -> Vec<Node> {
        let c = self.get_char_at(node).unwrap_or('.');
        match self.get_directions(c) {
            None => vec![],
            Some(directions) => {
                let mut v: Vec<Node> = vec![];
                directions.iter().for_each(|(dy, dx)| {
                    let y = node.0 as i32 + dy;
                    let x = node.1 as i32 + dx;
                    let len_y = self.rows.len() as i32;
                    let len_x = self.rows.get(0).unwrap().len() as i32;
                    if x >= 0 && y >= 0 && x < len_x && y < len_y {
                        v.push((y as u8, x as u8));
                    }
                });
                v
            }
        }
    }

    fn are_nodes_connected(&self, node_one: &Node, node_two: &Node) -> bool {
        let is_starting_node = self.get_char_at(node_one) == Some('S');
        let is_connected = self.get_end_nodes(node_one).contains(node_two);
        is_starting_node || is_connected
    }

    fn get_connected_nodes(&self, node: &Node) -> Vec<Node> {
        self.get_end_nodes(node)
            .iter()
            .filter(|&end_node| self.are_nodes_connected(end_node, node))
            .map(|v| *v)
            .collect()
    }

    fn get_next_node(&self, node_from: &Node, node_through: &Node) -> Option<Node> {
        match self.get_connected_nodes(node_through)[..] {
            [a, b] => {
                if &a == node_from {
                    Some(b)
                } else if &b == node_from {
                    Some(a)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn traverse_from(&self, node_from: &Node) -> Option<u64> {
        let mut current_node = self.start;
        let mut next_node = *node_from;
        let mut counter = 1u64;
        loop {
            let next = self.get_next_node(&current_node, &next_node);
            match next {
                None => None?,
                Some(next) => {
                    counter += 1;
                    current_node = next_node;
                    next_node = next;
                    if self.get_char_at(&next).unwrap() == 'S' {
                        break;
                    }
                }
            }
        }
        Some(counter / 2)
    }

    fn get_loop_starting_from(&self, node_from: &Node) -> Option<HashSet<Node>> {
        let mut current_node = self.start;
        let mut next_node = *node_from;
        let mut loop_nodes: HashSet<Node> = HashSet::new();
        loop_nodes.insert(next_node);
        loop {
            let next = self.get_next_node(&current_node, &next_node);
            match next {
                None => None?,
                Some(next) => {
                    current_node = next_node;
                    next_node = next;
                    loop_nodes.insert(next);
                    if self.get_char_at(&next).unwrap() == 'S' {
                        break;
                    }
                }
            }
        }
        Some(loop_nodes)
    }

    fn traverse(&self) -> u64 {
        let mut longest_loop_length = 0u64;
        let starting_nodes = self.get_starting_nodes();
        for node in starting_nodes {
            match self.traverse_from(&node) {
                None => (),
                Some(loop_length) => {
                    if loop_length > longest_loop_length {
                        longest_loop_length = loop_length
                    }
                }
            }
        }
        longest_loop_length
    }

    fn get_main_loop(&self) -> HashSet<Node> {
        let mut main_loop_nodes: HashSet<Node> = HashSet::new();
        let mut longest_loop_length: usize = 0;
        let starting_nodes = self.get_starting_nodes();
        for node in starting_nodes {
            match self.get_loop_starting_from(&node) {
                None => (),
                Some(loop_nodes) => {
                    if loop_nodes.len() > longest_loop_length {
                        longest_loop_length = loop_nodes.len();
                        main_loop_nodes = loop_nodes;
                    }
                }
            }
        }
        main_loop_nodes
    }

    fn is_opening_wall(&self, c: char) -> bool {
        ['L', 'F', '|'].contains(&c)
    }
    fn is_closing_wall(&self, c: char) -> bool {
        ['J', '7', '|'].contains(&c)
    }

    fn get_area_enclosed_by_main_loop(&self) -> u64 {
        let main_loop = self.get_main_loop();
        let mut enclosed_tiles = 0u64;
        for (y, row) in self.rows.iter().enumerate() {
            let mut walls = 0u64;
            let row = row
                .iter()
                .enumerate()
                .map(|(x, c)| match main_loop.contains(&(y as u8, x as u8)) {
                    true => c,
                    false => &'.',
                })
                .collect::<String>()
                .replace("S", "|")
                .replace("-", "")
                .replace("F7", "")
                .replace("LJ", "")
                .replace("L7", "|")
                .replace("FJ", "|")
                .replace("||", "");

            row.chars().for_each(|c| {
                if c == '|' {
                    walls += 1;
                } else if c == '.' {
                    if walls % 2 == 1 {
                        enclosed_tiles += 1
                    }
                }
            });

            println!("{row}")
        }
        enclosed_tiles
    }
}

pub fn get_longest_loop_from_input_file(filename: &str) -> u64 {
    let string = read_file(filename);
    let maze = Maze::from(string);
    maze.traverse()
}

pub fn get_area_enclosed_by_main_loop(filename: &str) -> u64 {
    let string = read_file(filename);
    let maze = Maze::from(string);

    maze.get_area_enclosed_by_main_loop()
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::advent_of_code::day_ten::pipe_maze::{
        get_area_enclosed_by_main_loop, get_longest_loop_from_input_file, Maze,
    };

    use super::Node;

    #[test]
    fn creates_maze_from_string() {
        let string = String::from("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...");

        let maze = Maze::from(string);

        assert_eq!(
            maze,
            Maze {
                start: (2, 0),
                rows: vec![
                    vec!['.', '.', 'F', '7', '.'],
                    vec!['.', 'F', 'J', '|', '.'],
                    vec!['S', 'J', '.', 'L', '7'],
                    vec!['|', 'F', '-', '-', 'J'],
                    vec!['L', 'J', '.', '.', '.'],
                ],
            },
        )
    }

    #[test]
    fn creates_maze_map() {
        let expected_starting_nodes: Vec<Node> = vec![(1, 0), (2, 1), (3, 0)];
        let maze = Maze {
            start: (2, 0),
            rows: vec![
                vec!['.', '.', 'F', '7', '.'],
                vec!['.', 'F', 'J', '|', '.'],
                vec!['S', 'J', '.', 'L', '7'],
                vec!['|', 'F', '-', '-', 'J'],
                vec!['L', 'J', '.', '.', '.'],
            ],
        };

        let starting_nodes = maze.get_starting_nodes();

        for expected_node in expected_starting_nodes {
            assert!(starting_nodes.contains(&expected_node));
        }
    }

    #[rstest]
    #[case((2, 1), vec![(2, 0), (1, 1)])] // J
    #[case((1, 3), vec![(0, 3), (2, 3)])] // |
    #[case((3, 3), vec![(3, 2), (3, 4)])] // -
    #[case((2, 3), vec![(1, 3), (2, 4)])] // L
    #[case((1, 1), vec![(2, 1), (1, 2)])] // F
    #[case((0, 3), vec![(0, 2), (1, 3)])] // 7
    #[case((0, 4), vec![(1, 4)])] //         | (egde)
    #[case((1, 4), vec![(0, 4)])] //         J (hitting pipe that's not turned in correct direction)
    #[case((2, 2), vec![])] //               - (hitting pipe that's not turned in correct direction)
    #[case((4, 4), vec![])] //               L (egde + hitting pipe that's not turned in correct direction)
    #[case((4, 2), vec![])] //               . dots always have no neighbours
    fn gets_possible_neighbouring_nodes(
        #[case] node: Node,
        #[case] expected_connected_nodes: Vec<Node>,
    ) {
        let maze = Maze {
            start: (2, 0),
            rows: vec![
                vec!['.', '.', 'F', '7', '|'],
                vec!['.', 'F', 'J', '|', 'J'],
                vec!['S', 'J', '-', 'L', '7'],
                vec!['|', 'F', '-', '-', 'J'],
                vec!['L', 'J', '.', '.', 'L'],
            ],
        };

        let connected_nodes = maze.get_connected_nodes(&node);

        assert_eq!(connected_nodes.len(), expected_connected_nodes.len());
        for node in connected_nodes {
            assert!(expected_connected_nodes.contains(&node));
        }
    }

    #[rstest]
    #[case((2, 1), (1, 1), Some((1, 2)))]
    #[case((1, 1), (1, 2), Some((0, 2)))]
    #[case((0, 4), (1, 4), None)]
    fn gets_next_node(
        #[case] node_from: Node,
        #[case] node_through: Node,
        #[case] expected_resulting_node: Option<Node>,
    ) {
        let maze = Maze {
            start: (2, 0),
            rows: vec![
                vec!['.', '.', 'F', '7', '|'],
                vec!['.', 'F', 'J', '|', 'J'],
                vec!['S', 'J', '.', 'L', '7'],
                vec!['|', 'F', '-', '-', 'J'],
                vec!['L', 'J', '.', '.', '.'],
            ],
        };

        let resulting_node = maze.get_next_node(&node_from, &node_through);

        assert_eq!(resulting_node, expected_resulting_node)
    }

    #[rstest]
    #[case((2, 1), Some(8))]
    #[case((3, 0), Some(8))]
    #[case((1, 0), None)]
    fn gets_loop_length_from_single_node(
        #[case] starting_node: Node,
        #[case] expected: Option<u64>,
    ) {
        let maze = Maze {
            start: (2, 0),
            rows: vec![
                vec!['.', '.', 'F', '7', '|'],
                vec!['|', 'F', 'J', '|', 'J'],
                vec!['S', 'J', '.', 'L', '7'],
                vec!['|', 'F', '-', '-', 'J'],
                vec!['L', 'J', '.', '.', '.'],
            ],
        };

        let loop_length = maze.traverse_from(&starting_node);

        assert_eq!(loop_length, expected)
    }

    #[test]
    fn gets_longest_loop() {
        let maze = Maze {
            start: (2, 0),
            rows: vec![
                vec!['.', '.', 'F', '7', '.'],
                vec!['.', 'F', 'J', '|', '.'],
                vec!['S', 'J', '.', 'L', '7'],
                vec!['|', 'F', '-', '-', 'J'],
                vec!['L', 'J', '.', '.', '.'],
            ],
        };

        let longest_loop_length = maze.traverse();

        assert_eq!(longest_loop_length, 8)
    }

    #[test]
    fn gets_loop_set() {
        let maze = Maze {
            start: (2, 0),
            rows: vec![
                vec!['.', '.', 'F', '7', '.'],
                vec!['.', 'F', 'J', '|', '.'],
                vec!['S', 'J', '.', 'L', '7'],
                vec!['|', 'F', '-', '-', 'J'],
                vec!['L', 'J', '.', '.', '.'],
            ],
        };

        let main_loop = maze.get_main_loop();

        let expected: Vec<Node> = vec![
            (2, 0),
            (2, 1),
            (1, 1),
            (1, 2),
            (0, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (2, 4),
            (3, 4),
            (3, 3),
            (3, 2),
            (3, 1),
            (4, 1),
            (4, 0),
            (3, 0),
        ];
        for node in expected {
            assert!(main_loop.contains(&node));
        }
    }

    #[test]
    fn gets_longest_loop_from_input_file() {
        let longest_loop_length =
            get_longest_loop_from_input_file("./src/advent_of_code/day_ten/test-input.txt");

        assert_eq!(longest_loop_length, 8)
    }

    #[test]
    fn gets_area_enclosed_by_main_loop() {
        let enclosed_area =
            get_area_enclosed_by_main_loop("./src/advent_of_code/day_ten/test-input-2.txt");

        assert_eq!(enclosed_area, 4);
    }
}
