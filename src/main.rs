use crate::advent_of_code::day_nine::mirage_maintenance::get_sum_of_next_numbers_from_file;

// mod currency;
// mod codewars;
mod advent_of_code;

fn main() {
    let sum_of_next_numbers =
        get_sum_of_next_numbers_from_file("./src/advent_of_code/day_nine/input.txt");

    assert_eq!(sum_of_next_numbers, 114);
}
