use std::fs::read_to_string;

pub fn read_file(filename: &str) -> String {
    match read_to_string(filename) {
        Err(err) => {
            eprintln!("{}", err);
            "".to_string()
        }
        Ok(open_file) => open_file,
    }
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_nine::file_reader::read_file;

    #[test]
    fn reads_lines_from_file() {
        let string = read_file("./src/advent_of_code/day_nine/test-input.txt");

        assert_eq!(
            string,
            String::from("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45")
        )
    }
}
