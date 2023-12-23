use crate::advent_of_code::day_eleven::{cosmic_expansion::GalaxyMap, file_reader::read_file};

// mod currency;
// mod codewars;
mod advent_of_code;

fn main() {
    let string = read_file("./src/advent_of_code/day_eleven/input.txt");
    let mut galaxy_map = GalaxyMap::from(string);

    galaxy_map.expand();
    let sum = galaxy_map.get_sum_of_shortest_distances_between_galaxies();

    println!("{sum}");
}
