// mod currency;
// mod codewars;
mod advent_of_code;

use advent_of_code::day_two::cube_conundrum_2::CubeConundrum;

fn main() {
    let mut cube_conundrum = CubeConundrum::new();
    cube_conundrum.read_games_from_file("./src/advent_of_code/day_two/input.txt");
    cube_conundrum.insert_cubes_into_bag(12, "red");
    cube_conundrum.insert_cubes_into_bag(13, "green");
    cube_conundrum.insert_cubes_into_bag(14, "blue");

    let sum_of_ids = cube_conundrum.get_sum_of_valid_game_ids();
    let sum_of_powers = cube_conundrum.get_sum_of_game_powers();

    println!("Sum of ids: {}", sum_of_ids);
    println!("Sum of powers: {}", sum_of_powers);
}
