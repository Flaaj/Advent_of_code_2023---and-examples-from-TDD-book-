use crate::advent_of_code::day_seven::camel_cards_2::{
    calculate_total_winnings, read_hands_from_file,
};

// mod currency;
// mod codewars;
mod advent_of_code;

fn main() {
    let hands = &mut read_hands_from_file("./src/advent_of_code/day_seven/test-input.txt");

    let total_winnings = calculate_total_winnings(hands);

    println!("{total_winnings}");
}
