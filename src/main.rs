use crate::advent_of_code::day_five::seed_fertilizer::LocationFinder;

// mod currency;
// mod codewars;
mod advent_of_code;

fn main() {
    let mut location_finder = LocationFinder::new();
    location_finder.load_mappers_from_file("./src/advent_of_code/day_five/input.txt");

    let lowers_location_number = location_finder.find_lowest_location_number_part_two();

    println!("{lowers_location_number}");
}
