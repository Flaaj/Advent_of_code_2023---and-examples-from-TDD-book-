use std::collections::HashSet;

use super::file_reader::FileReader;

#[derive(PartialEq, Debug)]
struct Game {
    id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
}

impl Game {
    fn new(id: i32, winning_numbers: Vec<i32>, numbers: Vec<i32>) -> Self {
        Self {
            id,
            winning_numbers,
            numbers,
        }
    }
}

struct GameParser {}

impl GameParser {
    fn new() -> Self {
        Self {}
    }

    fn parse_id(meta_str: &str) -> i32 {
        let split = meta_str.split(" ").collect::<Vec<&str>>();
        split.get(split.len() - 1).unwrap().parse().unwrap()
    }

    fn parse_numbers(string: &str) -> Vec<i32> {
        let mut digits = String::from("");
        let mut numbers: Vec<i32> = Vec::new();
        string.chars().for_each(|c| match c.is_digit(10) {
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

    fn parse_game(&self, line: &str) -> Game {
        let split_line: Vec<&str> = line.split([':', '|'].as_ref()).collect();

        let meta_str = split_line.get(0).unwrap();
        let winning_numbers_str = split_line.get(1).unwrap();
        let numbers_str = split_line.get(2).unwrap();

        let id = Self::parse_id(meta_str);
        let winning_numbers = Self::parse_numbers(winning_numbers_str);
        let numbers = Self::parse_numbers(numbers_str);

        Game::new(id, winning_numbers, numbers)
    }
}

struct GamePointsCalculator {}

impl GamePointsCalculator {
    fn new() -> Self {
        Self {}
    }

    fn calculate_game_points(&self, game: &Game) -> i32 {
        let winning_set: HashSet<&i32> = HashSet::from_iter(game.winning_numbers.iter());
        let hits = game
            .numbers
            .iter()
            .filter(|&num| winning_set.contains(num))
            .collect::<Vec<&i32>>();
        match hits.len() {
            0 => 0,
            len => 2i32.pow(len as u32 - 1),
        }
    }
}

pub struct Scratchcards {
    games: Vec<Game>,
    file_reader: FileReader,
    game_parser: GameParser,
    game_points_calculator: GamePointsCalculator,
}

impl Scratchcards {
    pub fn new() -> Self {
        Self {
            games: vec![],
            file_reader: FileReader::new(),
            game_parser: GameParser::new(),
            game_points_calculator: GamePointsCalculator::new(),
        }
    }

    pub fn load_from_file(&mut self, filename: &str) {
        let string = self.file_reader.read(filename);
        let lines = string.lines();
        let games: Vec<Game> = lines
            .map(|line| self.game_parser.parse_game(line))
            .collect();
        self.games = games;
    }

    pub fn calculate_sum_of_points(&self) -> i32 {
        self.games.iter().fold(0, |acc, game| {
            acc + self.game_points_calculator.calculate_game_points(game)
        })
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::advent_of_code::day_four::scratchcards::{
        Game, GameParser, GamePointsCalculator, Scratchcards,
    };

    #[test]
    fn parses_game_from_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let game_parser = GameParser::new();

        let game = game_parser.parse_game(line);

        assert_eq!(
            game,
            Game {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );
    }

    #[rstest]
    #[case(vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53], 8)]
    #[case(vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19], 2)]
    #[case(vec![87, 83, 26, 28, 32], vec![88, 30, 70, 12, 93, 22, 82, 36], 0)]
    fn calculates_points_for_a_game(
        #[case] winning_numbers: Vec<i32>,
        #[case] numbers: Vec<i32>,
        #[case] expected: i32,
    ) {
        let game = Game::new(1, winning_numbers, numbers);
        let game_points_calculator = GamePointsCalculator::new();

        let points = game_points_calculator.calculate_game_points(&game);

        assert_eq!(points, expected);
    }

    #[test]
    fn calculates_sum_of_points() {
        let mut scratchcards = Scratchcards::new();
        scratchcards.load_from_file("./src/advent_of_code/day_four/test-input.txt");

        let points = scratchcards.calculate_sum_of_points();

        assert_eq!(points, 13);
    }
}
