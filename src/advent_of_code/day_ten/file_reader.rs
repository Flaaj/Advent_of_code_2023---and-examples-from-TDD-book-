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
    use crate::advent_of_code::day_ten::file_reader::read_file;

    #[test]
    fn reads_lines_from_file() {
        let string = read_file("./src/advent_of_code/day_ten/test-input.txt");

        assert_eq!(
            string,
            String::from("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...")
        )
    }
}
