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
    use crate::advent_of_code::day_three::file_reader::FileReader;

    #[test]
    fn reads_lines_from_file() {
        let mut file_reader = FileReader::new();

        let lines = file_reader.read("./src/advent_of_code/day_three/test-input.txt");

        assert_eq!(
            lines,
            String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..")
        )
    }
}
