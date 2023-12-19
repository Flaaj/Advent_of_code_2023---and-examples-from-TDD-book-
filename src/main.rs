use advent_of_code::day_ten::pipe_maze::get_longest_loop_from_input_file;

use crate::advent_of_code::day_ten::pipe_maze::get_area_enclosed_by_main_loop;

// mod currency;
// mod codewars;
mod advent_of_code;

fn main() {
    let longest_loop = get_area_enclosed_by_main_loop("./src/advent_of_code/day_ten/input.txt");

    println!("{longest_loop}")
}
