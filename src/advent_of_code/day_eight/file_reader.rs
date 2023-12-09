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
    use crate::advent_of_code::day_eight::file_reader::FileReader;

    #[test]
    fn reads_lines_from_file() {
        let string = FileReader::read("./src/advent_of_code/day_eight/test-input-1.txt");

        assert_eq!(
            string,
            String::from("RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)")
        )
    }
}
