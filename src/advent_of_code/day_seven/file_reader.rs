use std::fs::read_to_string;

pub struct FileReader {}

impl FileReader {
    pub fn read(filename: &str) -> String {
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
    use crate::advent_of_code::day_seven::file_reader::FileReader;

    #[test]
    fn reads_lines_from_file() {
        let string = FileReader::read("./src/advent_of_code/day_seven/test-input.txt");

        assert_eq!(
            string,
            String::from("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483")
        )
    }
}
