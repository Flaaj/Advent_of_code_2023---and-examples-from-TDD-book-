use advent_of_code::day_six::wait_for_it::get_product_of_winning_races;

// mod currency;
// mod codewars;
mod advent_of_code;

fn main() {
    let product_1 = get_product_of_winning_races(&[(50, 242), (74, 1017), (86, 1691), (85, 1252)]);
    let product_2 = get_product_of_winning_races(&[(50748685, 242101716911252)]);

    println!("part 1: {product_1}");
    println!("part 2: {product_2}");
}
