pub struct NumberExtractor {}

impl NumberExtractor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_numbers(&self, string: &str) -> Vec<u64> {
        let mut digits = String::from("");
        let mut numbers: Vec<u64> = Vec::new();
        string.chars().for_each(|c| match c.is_digit(10) {
            true => digits.push(c),
            false => {
                if digits.len() > 0 {
                    numbers.push(digits.parse::<u64>().unwrap());
                    digits = String::from("");
                }
            }
        });
        if digits.len() > 0 {
            numbers.push(digits.parse::<u64>().unwrap());
        }
        numbers
    }
}

#[cfg(test)]
mod test {
    use crate::advent_of_code::day_five::number_extractor::NumberExtractor;

    #[test]
    fn parses_numbers_from_line() {
        let line = "   41 48 83 86 17 83 86  6 31 17  9 48 53 4000000000 ";
        let number_extractor = NumberExtractor::new();

        let numbers = number_extractor.parse_numbers(line);

        assert_eq!(
            numbers,
            vec![41, 48, 83, 86, 17, 83, 86, 6, 31, 17, 9, 48, 53, 4000000000]
        );
    }
}
