use super::file_reader::read_file;

type Node = (u32, u32);

type DirectionVector = (i32, i32);
const UP: DirectionVector = (-1, 0);
const DOWN: DirectionVector = (1, 0);
const LEFT: DirectionVector = (0, -1);
const RIGHT: DirectionVector = (0, 1);
const UP_RIGHT: DirectionVector = (-1, 1);
const UP_LEFT: DirectionVector = (-1, -1);
const DOWN_RIGHT: DirectionVector = (1, 1);
const DOWN_LEFT: DirectionVector = (1, -1);
const DIRECTION_VECTORS: [DirectionVector; 8] = [
    UP_LEFT, UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT,
];

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
                    Some((j, _)) => start = Some((i as u32, j as u32)),
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
        let len = self.rows.get(0).unwrap().len() as i32;
        let mut starting_nodes: Vec<Node> = vec![];
        let (y, x) = self.start;
        for (dy, dx) in DIRECTION_VECTORS {
            let y = y as i32 + dy;
            let x = x as i32 + dx;
            if x >= 0 && y >= 0 && x < len && y < len {
                starting_nodes.push((y as u32, x as u32));
            }
        }
        starting_nodes
    }

    fn get_char_at(&self, node: &Node) -> Option<char> {
        let row = self.rows.get(node.0 as usize)?;
        let c = row.get(node.1 as usize)?;
        Some(*c)
    }

    fn get_directions(&self, c: char) -> Option<[DirectionVector; 2]> {
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
                    let len = self.rows.len() as i32;
                    if x >= 0 && y >= 0 && x < len && y < len {
                        v.push((y as u32, x as u32));
                    }
                });
                v
            }
        }
    }

    fn get_connected_nodes(&self, node: &Node) -> Vec<Node> {
        self.get_end_nodes(node)
            .iter()
            .filter(|&end_node| {
                let is_starting_node = self.get_char_at(end_node) == Some('S');
                let is_connected = self.get_end_nodes(end_node).contains(&node);
                is_starting_node || is_connected
            })
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
        let mut limit = 100_000u64;
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
            //
            limit -= 1;
            if limit == 0 {
                break;
            }
        }
        Some(counter / 2)
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
}

pub fn get_longest_loop_from_input_file(filename: &str) -> u64 {
    let string = read_file(filename);
    let maze = Maze::from(string);
    maze.traverse()
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::advent_of_code::day_ten::pipe_maze::{get_longest_loop_from_input_file, Maze};

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
    fn gets_longest_loop_from_input_file() {
        let longest_loop_length =
            get_longest_loop_from_input_file("./src/advent_of_code/day_ten/test-input.txt");

        assert_eq!(longest_loop_length, 8)
    }
}
