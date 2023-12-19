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
    use crate::advent_of_code::day_eleven::file_reader::read_file;

    #[test]
    fn reads_lines_from_file() {
        let string = read_file("./src/advent_of_code/day_eleven/test-input.txt");

        assert_eq!(
            string,
            String::from("....1........\n.........2...\n3............\n.............\n.............\n........4....\n.5...........\n.##.........6\n..##.........\n...##........\n....##...7...\n8....9.......")
        )
    }
}
