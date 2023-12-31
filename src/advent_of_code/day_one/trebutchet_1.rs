pub struct CalibrationValueExtractor {
    lines: Vec<String>,
    value: u32,
}

impl CalibrationValueExtractor {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines, value: 0 }
    }

    fn extract_single_line_value(line: &String) -> u32 {
        let digits: Vec<u32> = line
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

    use crate::advent_of_code::day_one::{
        line_reader::LineReader, trebutchet_1::CalibrationValueExtractor,
    };

    #[rstest]
    #[case(String::from("1abc2"))]
    #[case(String::from("treb7uchet"))]
    fn adds_lines_of_calibration_document(#[case] line: String) {
        let mut line_reader = LineReader::new();

        line_reader.add_line(line.clone());

        assert_eq!(line_reader.get_lines(), vec![line]);
    }
    #[rstest]
    #[case(String::from("1abc2"), 12)]
    #[case(String::from("treb7uchet"), 77)]
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
