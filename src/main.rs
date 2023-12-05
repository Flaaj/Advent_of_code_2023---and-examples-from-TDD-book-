use advent_of_code::day_three::gear_ratios::GearRatios;

// mod currency;
// mod codewars;
mod advent_of_code;

fn main() {
    let mut gear_ratios = GearRatios::new();
    gear_ratios.read_from_file("./src/advent_of_code/day_three/input.txt");

    let sum = gear_ratios.calculatr_sum_of_part_numbers();

    println!("{sum}")
}
