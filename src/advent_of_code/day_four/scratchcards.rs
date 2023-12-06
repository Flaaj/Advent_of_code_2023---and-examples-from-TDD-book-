use std::collections::{HashMap, HashSet};

use super::file_reader::FileReader;

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Game {
    fn new(id: u32, winning_numbers: Vec<u32>, numbers: Vec<u32>) -> Self {
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

    fn parse_id(meta_str: &str) -> u32 {
        let split = meta_str.split(" ").collect::<Vec<&str>>();
        split.get(split.len() - 1).unwrap().parse().unwrap()
    }

    fn parse_numbers(string: &str) -> Vec<u32> {
        let mut digits = String::from("");
        let mut numbers: Vec<u32> = Vec::new();
        string.chars().for_each(|c| match c.is_digit(10) {
            true => digits.push(c),
            false => {
                if digits.len() > 0 {
                    numbers.push(digits.parse::<u32>().unwrap());
                    digits = String::from("");
                }
            }
        });
        if digits.len() > 0 {
            numbers.push(digits.parse::<u32>().unwrap());
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

    fn calculate_hits(&self, game: &Game) -> u32 {
        let winning_set: HashSet<&u32> = HashSet::from_iter(game.winning_numbers.iter());
        let hits = game
            .numbers
            .iter()
            .filter(|&num| winning_set.contains(num))
            .collect::<Vec<&u32>>();
        hits.len() as u32
    }

    fn calculate_game_points(&self, game: &Game) -> u32 {
        let hits = self.calculate_hits(game);
        match hits {
            0 => 0,
            hits => 2u32.pow(hits - 1),
        }
    }
}

struct SumOfCardsCalculator {}

impl SumOfCardsCalculator {
    fn new() -> Self {
        Self {}
    }

    fn calculate_sum_of_cards(
        &self,
        game_points_calculator: &GamePointsCalculator,
        games: &Vec<Game>,
    ) -> u32 {
        let mut hashmap: HashMap<u32, u32> = HashMap::new();
        games.iter().for_each(|game| {
            hashmap.insert(game.id, 1);
        });

        games.iter().for_each(|game| {
            let hits = game_points_calculator.calculate_hits(game);
            let cards_count = hashmap.get(&game.id).unwrap().clone();
            if hits > 0 {
                (1..(hits + 1)).for_each(|i| {
                    let game_id = game.id + i as u32;
                    match hashmap.get(&game_id) {
                        None => (),
                        Some(&current) => {
                            hashmap.insert(game_id, current + cards_count);
                        }
                    };
                });
            }
        });

        hashmap.values().fold(0, |acc, val| acc + val)
    }
}

pub struct Scratchcards {
    games: Vec<Game>,
    file_reader: FileReader,
    game_parser: GameParser,
    game_points_calculator: GamePointsCalculator,
    sum_of_cards_calculator: SumOfCardsCalculator,
}

impl Scratchcards {
    pub fn new() -> Self {
        Self {
            games: vec![],
            file_reader: FileReader::new(),
            game_parser: GameParser::new(),
            game_points_calculator: GamePointsCalculator::new(),
            sum_of_cards_calculator: SumOfCardsCalculator::new(),
        }
    }

    pub fn load_from_file(&mut self, filename: &str) {
        let string = self.file_reader.read(filename);
        let games: Vec<Game> = string
            .lines()
            .map(|line| self.game_parser.parse_game(line))
            .collect();
        self.games = games;
    }

    pub fn calculate_sum_of_points(&self) -> u32 {
        self.games.iter().fold(0, |acc, game| {
            acc + self.game_points_calculator.calculate_game_points(game)
        })
    }

    pub fn calculate_sum_of_cards(&self) -> u32 {
        self.sum_of_cards_calculator
            .calculate_sum_of_cards(&self.game_points_calculator, &self.games)
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
        #[case] winning_numbers: Vec<u32>,
        #[case] numbers: Vec<u32>,
        #[case] expected: u32,
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

    #[test]
    fn calculates_sum_of_cards() {
        let mut scratchcards = Scratchcards::new();
        scratchcards.load_from_file("./src/advent_of_code/day_four/test-input.txt");

        let points = scratchcards.calculate_sum_of_cards();

        assert_eq!(points, 30);
    }
}
