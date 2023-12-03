use std::fs::read_to_string;

pub struct LineReader {
    lines: Vec<String>,
}

impl LineReader {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }

    pub fn add_line(&mut self, line: String) {
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

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_one::line_reader::LineReader;

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
