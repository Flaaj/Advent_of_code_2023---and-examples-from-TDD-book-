use crate::advent_of_code::day_four::scratchcards::Scratchcards;

// mod currency;
// mod codewars;
mod advent_of_code;

fn main() {
    let mut scratchcards = Scratchcards::new();
    scratchcards.load_from_file("./src/advent_of_code/day_four/input.txt");

    let cards = scratchcards.calculate_sum_of_cards();

    println!("{cards}");
}
