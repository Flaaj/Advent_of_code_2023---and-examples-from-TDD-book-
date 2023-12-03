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
        self.lines.clear();
        match read_to_string(filename) {
            Err(err) => println!("{}", err),
            Ok(open_file) => {
                for line in open_file.lines() {
                    self.add_line(line.to_string());
                }
            }
        }
    }

    pub fn get_lines(&self) -> Vec<String> {
        self.lines.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_two::line_reader::LineReader;

    #[test]
    fn reads_lines_from_file() {
        let mut line_reader = LineReader::new();

        line_reader.read_lines_from_file("./src/advent_of_code/day_two/test-input.txt");

        assert_eq!(
            line_reader.get_lines(),
            vec![
                String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
                String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
                String::from(
                    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                ),
                String::from(
                    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                ),
                String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            ]
        )
    }
}
