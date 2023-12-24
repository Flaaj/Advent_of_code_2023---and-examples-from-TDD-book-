use crate::advent_of_code::day_eleven::{file_reader::read_file, cosmic_expansion::Universe};

// mod currency;
// mod codewars;
mod advent_of_code;

fn main() {
    let string = read_file("./src/advent_of_code/day_eleven/input.txt");
    let mut universe = Universe::from(string);

    universe.expand(999_999);
    let sum = universe.get_sum_of_shortest_distances_between_galaxies();

    println!("{sum}");
}
