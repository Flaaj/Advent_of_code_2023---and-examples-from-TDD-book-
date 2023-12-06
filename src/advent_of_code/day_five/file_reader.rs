use std::fs::read_to_string;

pub struct FileReader {}

impl FileReader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read(&mut self, filename: &str) -> String {
        match read_to_string(filename) {
            Err(err) => {
                eprintln!("{}", err);
                "".to_string()
            }
            Ok(open_file) => open_file,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_five::file_reader::FileReader;

    #[test]
    fn reads_lines_from_file() {
        let mut file_reader = FileReader::new();

        let string = file_reader.read("./src/advent_of_code/day_five/test-input.txt");

        let lines = string.lines().collect::<Vec<&str>>();
        let first_line = lines.first().unwrap().to_owned();
        assert_eq!(first_line, "seeds: 79 14 55 13")
    }
}
