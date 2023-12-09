pub fn parse_numbers_from_string(string: &str) -> Vec<i32> {
    let mut digits = String::from("");
    let mut numbers: Vec<i32> = Vec::new();
    string.chars().for_each(|c| match c.is_digit(10) || c == '-' {
        true => digits.push(c),
        false => {
            if digits.len() > 0 {
                numbers.push(digits.parse::<i32>().unwrap());
                digits = String::from("");
            }
        }
    });
    if digits.len() > 0 {
        numbers.push(digits.parse::<i32>().unwrap());
    }
    numbers
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_nine::number_extractor::parse_numbers_from_string;

    #[test]
    fn parses_numbers_from_line() {
        let line = "   41 -48 83 86 17 83 86  6 31 17  9 48 53 4000000 ";

        let numbers = parse_numbers_from_string(line);

        assert_eq!(
            numbers,
            vec![41, -48, 83, 86, 17, 83, 86, 6, 31, 17, 9, 48, 53, 4000000]
        );
    }
}
