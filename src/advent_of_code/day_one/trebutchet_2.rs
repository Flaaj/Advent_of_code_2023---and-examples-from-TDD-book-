use std::fs::read_to_string;

pub struct LineReader {
    lines: Vec<String>,
}

impl LineReader {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }

    fn add_line(&mut self, line: String) {
        self.lines.push(line)
    }

    pub fn read_lines_from_file(&mut self, filename: &str) {
        match read_to_string(filename) {
            Err(err) => println!("{}", err),
            Ok(open_file) => {
                for line in open_file.lines() {
                    self.add_line(line.to_string());
                }
            }
        }
    }

    pub fn get_lines(self) -> Vec<String> {
        self.lines
    }
}

pub struct CalibrationValueExtractor {
    lines: Vec<String>,
    value: u32,
}

impl CalibrationValueExtractor {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines, value: 0 }
    }

    fn replace_digit_names_with_digits(line: &String) -> String {
        // very ugly:
        line.replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
    }

    fn get_digits_from_line(line: &String) -> Vec<u32> {
        todo!()
    }

    fn extract_single_line_value(line: &String) -> u32 {
        let digits: Vec<u32> = Self::replace_digit_names_with_digits(line)
            .chars()
            .map(|c| c.to_digit(10))
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect();
        match digits.len() {
            0 => 0,
            len => 10 * digits.get(0).unwrap() + digits.get(len - 1).unwrap(),
        }
    }

    pub fn extract_value(&mut self) {
        for line in self.lines.iter() {
            self.value += Self::extract_single_line_value(line)
        }
    }

    pub fn get_value(self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::advent_of_code::day_one::trebutchet_2::LineReader;

    use super::CalibrationValueExtractor;

    #[rstest]
    #[case(String::from("1abc2"))]
    #[case(String::from("treb7uchet"))]
    fn adds_lines_of_calibration_document(#[case] line: String) {
        let mut line_reader = LineReader::new();

        line_reader.add_line(line.clone());

        assert_eq!(line_reader.get_lines(), vec![line]);
    }

    #[test]
    fn extracts_all_digits_from_line() {
        let digits = CalibrationValueExtractor::get_digits_from_line(&String::from("xtwone3four"));

        assert_eq!(digits, [2, 1, 3, 4])
    }

    #[rstest]
    #[case(String::from("1abc2"), 12)]
    #[case(String::from("treb7uchet"), 77)]
    #[case(String::from("two1"), 21)]
    #[case(String::from("two1nine"), 29)]
    #[case(String::from("eightwothree"), 83)]
    #[case(String::from("abcone2threexyz"), 13)]
    #[case(String::from("xtwone3four"), 24)]
    #[case(String::from("4nineeightseven2"), 42)]
    #[case(String::from("zoneight234"), 14)]
    #[case(String::from("7pqrstsixteen"), 76)]
    fn extracts_calibration_value_of_a_single_line(#[case] line: String, #[case] expected: u32) {
        let mut calibration_value_extractor = CalibrationValueExtractor::new(vec![line]);

        calibration_value_extractor.extract_value();

        assert_eq!(calibration_value_extractor.get_value(), expected)
    }

    #[test]
    fn extracts_calibration_value_of_multiple_lines() {
        let mut line_reader = LineReader::new();
        line_reader.add_line(String::from("1abc2"));
        line_reader.add_line(String::from("pqr3stu8vwx"));
        line_reader.add_line(String::from("a1b2c3d4e5f"));
        line_reader.add_line(String::from("treb7uchet"));
        let mut calibration_value_extractor =
            CalibrationValueExtractor::new(line_reader.get_lines());

        calibration_value_extractor.extract_value();

        assert_eq!(calibration_value_extractor.get_value(), 142)
    }

    #[test]
    fn reads_lines_from_file() {
        let mut line_reader = LineReader::new();

        line_reader.read_lines_from_file("./src/advent_of_code/day_one/test-input.txt");

        assert_eq!(
            line_reader.get_lines(),
            vec![
                String::from("1abc2"),
                String::from("pqr3stu8vwx"),
                String::from("a1b2c3d4e5f"),
                String::from("treb7uchet"),
            ]
        )
    }
}
