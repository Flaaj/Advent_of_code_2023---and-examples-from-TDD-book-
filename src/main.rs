use crate::advent_of_code::day_eight::haunted_wasteland_2::create_nodes_from_file;

// mod currency;
// mod codewars;
mod advent_of_code;

fn main() {
    let nodes = create_nodes_from_file("./src/advent_of_code/day_eight/input.txt");

    let steps = nodes.traverse();

    assert_eq!(steps, 2);
}
