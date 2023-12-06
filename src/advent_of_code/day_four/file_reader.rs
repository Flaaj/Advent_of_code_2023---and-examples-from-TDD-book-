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
    use crate::advent_of_code::day_four::file_reader::FileReader;

    #[test]
    fn reads_lines_from_file() {
        let mut file_reader = FileReader::new();

        let lines = file_reader.read("./src/advent_of_code/day_four/test-input.txt");

        assert_eq!(
            lines,
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
        )
    }
}
