use advent_of_code::day_one::trebutchet_2::{CalibrationValueExtractor, LineReader};

// mod currency;
// mod codewars;
mod advent_of_code;

fn main() {
    let mut line_reader = LineReader::new();
    line_reader.read_lines_from_file("./src/advent_of_code/day_two/input.txt");

    let mut calibration_value_extractor = CalibrationValueExtractor::new(line_reader.get_lines());
    calibration_value_extractor.extract_value();

    println!("{}", calibration_value_extractor.get_value());
}
