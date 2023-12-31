use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use super::line_reader::LineReader;

/**
 *
 * Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
 * Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
 * Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
 * Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
 * Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
 *
 * In game 1, three sets of cubes are revealed from the bag (and then put back again).
 * The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes;
 * the third set is only 2 green cubes.
 *
 * The Elf would first like to know which games would have been possible
 * if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
 *
 * In the example above, games 1, 2, and 5 would have been possible
 * if the bag had been loaded with that configuration. However, game 3 would have been impossible
 * because at one point the Elf showed you 20 red cubes at once; similarly,
 * game 4 would also have been impossible because the Elf showed you 15 blue cubes at once.
 * If you add up the IDs of the games that would have been possible, you get 8.
 *
 * Determine which games would have been possible if the bag had been loaded
 * with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
 */

#[derive(PartialEq, Debug, Clone)]
struct Cubes {
    color: String,
    count: u32,
}

#[derive(PartialEq, Debug, Clone)]
struct Game {
    id: u32,
    sets: Vec<Vec<Cubes>>,
}

lazy_static! {
    static ref VALID_GAME_REGEX: Regex =
        Regex::new(r"^Game \d+: (\d+ [a-z]+[;,] )?+(\d+ [a-z]+)$").unwrap();
}

struct GameParser {}

impl GameParser {
    fn new() -> Self {
        Self {}
    }

    fn validate_game_string(line: &String) -> Result<&String, &str> {
        match VALID_GAME_REGEX.is_match(line) {
            false => Err("Not a valid game string"),
            true => Ok(line),
        }
    }

    fn parse_id(meta_str: &str) -> u32 {
        meta_str
            .split(" ")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .parse()
            .unwrap()
    }

    fn parse_cubes(cubes_str: &str) -> Cubes {
        let cubes_split: Vec<&str> = cubes_str.split(" ").collect();
        let count = cubes_split.get(0).unwrap().parse().unwrap();
        let color = cubes_split.get(1).unwrap().to_string();
        Cubes { count, color }
    }

    fn parse_set(set_str: &str) -> Vec<Cubes> {
        set_str
            .split(", ")
            .map(|cubes_str| Self::parse_cubes(cubes_str))
            .collect()
    }

    fn parse_sets(sets_str: &str) -> Vec<Vec<Cubes>> {
        sets_str
            .split("; ")
            .map(|set_str| Self::parse_set(set_str))
            .collect()
    }

    fn parse_game(game_str: &str) -> Game {
        let split_game_str: Vec<&str> = game_str.split(": ").collect();

        let meta_str = split_game_str.get(0).unwrap();
        let sets_str = split_game_str.get(1).unwrap();

        let id = Self::parse_id(meta_str);
        let sets = Self::parse_sets(sets_str);

        Game { id, sets }
    }

    fn parse_line(&mut self, line: String) -> Option<Game> {
        match Self::validate_game_string(&line) {
            Err(err) => {
                eprintln!("{}", err);
                None
            }
            Ok(game_str) => Some(Self::parse_game(game_str)),
        }
    }

    fn parse_games(&mut self, lines: Vec<String>) -> Vec<Game> {
        let mut games = vec![];
        for line in lines {
            match self.parse_line(line) {
                None => (),
                Some(game) => games.push(game),
            }
        }
        games
    }
}

struct Bag {
    cubes_count_by_color: HashMap<String, u32>,
}

impl Bag {
    fn new() -> Self {
        Self {
            cubes_count_by_color: HashMap::new(),
        }
    }

    fn insert_cubes(&mut self, count: u32, color: String) {
        self.cubes_count_by_color.insert(color, count);
    }

    fn get_cubes_count_by_color(&self, color: &String) -> u32 {
        match self.cubes_count_by_color.get(color) {
            None => 0,
            Some(&count) => count,
        }
    }
}

struct GameValidator {}

impl GameValidator {
    fn new() -> Self {
        Self {}
    }

    fn validate_cubes_of_single_color(bag: &Bag, cubes: &Cubes) -> bool {
        cubes.count <= bag.get_cubes_count_by_color(&cubes.color)
    }

    fn validate_set(bag: &Bag, set: &Vec<Cubes>) -> bool {
        set.iter()
            .all(|cubes| Self::validate_cubes_of_single_color(bag, cubes))
    }

    fn validate_game(bag: &Bag, game: &Game) -> bool {
        game.sets.iter().all(|set| Self::validate_set(bag, set))
    }

    fn get_sum_of_valid_game_ids(&self, bag: &Bag, games: &Vec<Game>) -> u32 {
        games.iter().fold(0, |acc, game| {
            acc + match Self::validate_game(bag, &game) {
                true => game.id,
                false => 0,
            }
        })
    }
}

struct GamePowerCalculator {}

impl GamePowerCalculator {
    fn new() -> Self {
        Self {}
    }

    fn set_minimum_needed_cube_counts_of_each_color<'a>(
        hashmap: &mut HashMap<&'a String, u32>,
        set: &'a Vec<Cubes>,
    ) {
        set.iter().for_each(move |cubes| {
            let current = match hashmap.get(&cubes.color) {
                Some(&count) => count,
                None => 0,
            };
            if cubes.count > current {
                hashmap.insert(&cubes.color, cubes.count);
            };
        })
    }

    fn get_game_power(game: &Game) -> u32 {
        let hashmap = &mut HashMap::<&String, u32>::new();
        game.sets
            .iter()
            .for_each(|set| Self::set_minimum_needed_cube_counts_of_each_color(hashmap, set));
        hashmap.values().fold(1, |acc, val| acc * val)
    }

    fn get_sum_of_game_powers(&self, games: &Vec<Game>) -> u32 {
        games
            .iter()
            .map(|game| Self::get_game_power(game))
            .fold(0, |acc, val| acc + val)
    }
}

pub struct CubeConundrum {
    line_reader: LineReader,
    game_parser: GameParser,
    game_validator: GameValidator,
    game_power_calculator: GamePowerCalculator,
    bag: Bag,
    games: Vec<Game>,
}

impl CubeConundrum {
    pub fn new() -> Self {
        Self {
            line_reader: LineReader::new(),
            game_parser: GameParser::new(),
            game_validator: GameValidator::new(),
            game_power_calculator: GamePowerCalculator::new(),
            bag: Bag::new(),
            games: vec![],
        }
    }

    pub fn read_games_from_file(&mut self, filename: &str) {
        let lines = self.line_reader.read_lines_from_file(filename);
        let games = self.game_parser.parse_games(lines);
        self.games = games
    }

    pub fn insert_cubes_into_bag(&mut self, count: u32, color: &str) {
        self.bag.insert_cubes(count, color.to_owned());
    }

    pub fn get_sum_of_valid_game_ids(&self) -> u32 {
        self.game_validator
            .get_sum_of_valid_game_ids(&self.bag, &self.games)
    }

    pub fn get_sum_of_game_powers(&self) -> u32 {
        self.game_power_calculator
            .get_sum_of_game_powers(&self.games)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::advent_of_code::day_two::cube_conundrum_2::{
        Bag, CubeConundrum, Cubes, Game, GameParser, GameValidator,
    };

    #[test]
    fn extracts_game_from_input_line_case_1() {
        let mut game_parser = GameParser::new();

        let game = game_parser
            .parse_line(String::from(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            ))
            .unwrap();

        assert_eq!(
            game,
            Game {
                id: 1,
                sets: vec![
                    vec![
                        Cubes {
                            color: String::from("blue"),
                            count: 3
                        },
                        Cubes {
                            color: String::from("red"),
                            count: 4
                        },
                    ],
                    vec![
                        Cubes {
                            color: String::from("red"),
                            count: 1
                        },
                        Cubes {
                            color: String::from("green"),
                            count: 2
                        },
                        Cubes {
                            color: String::from("blue"),
                            count: 6
                        },
                    ],
                    vec![Cubes {
                        color: String::from("green"),
                        count: 2
                    },]
                ]
            }
        )
    }

    #[test]
    fn extracts_game_from_input_line_case_2() {
        let mut game_parser = GameParser::new();

        let game = game_parser
            .parse_line(String::from(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            ))
            .unwrap();

        assert_eq!(
            game,
            Game {
                id: 3,
                sets: vec![
                    vec![
                        Cubes {
                            color: String::from("green"),
                            count: 8
                        },
                        Cubes {
                            color: String::from("blue"),
                            count: 6
                        },
                        Cubes {
                            color: String::from("red"),
                            count: 20
                        },
                    ],
                    vec![
                        Cubes {
                            color: String::from("blue"),
                            count: 5
                        },
                        Cubes {
                            color: String::from("red"),
                            count: 4
                        },
                        Cubes {
                            color: String::from("green"),
                            count: 13
                        },
                    ],
                    vec![
                        Cubes {
                            color: String::from("green"),
                            count: 5
                        },
                        Cubes {
                            color: String::from("red"),
                            count: 1
                        },
                    ]
                ]
            }
        )
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green; 2 green; 2 green; 2 green")]
    #[case("Game 2: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green")]
    #[case("Game 4: 8 green")]
    #[case("Game 5: 562691234 green, 562655 red, 8124124 blue")]
    fn parses_correct_lines(#[case] line: String) {
        let mut game_parser = GameParser::new();

        let game = game_parser.parse_line(line);

        assert!(game.is_some());
    }

    #[rstest]
    #[case("Game 3 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")]
    #[case("3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")]
    #[case("Game 2: -8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")]
    #[case("Game -2: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")]
    fn doesnt_parse_incorrect_lines(#[case] line: String) {
        let mut game_parser = GameParser::new();

        let game = game_parser.parse_line(line);

        assert!(game.is_none());
    }

    #[rstest]
    #[case("Game 3: 9 green, 8 red, 10 blue", true)]
    #[case("Game 1: 8 green", true)]
    #[case("Game 3: 12 green, 8 red, 9 blue", false)]
    #[case("Game 2: 12 green", false)]
    fn validates_single_game_correctly(#[case] line: String, #[case] should_be_valid: bool) {
        let mut bag = Bag::new();
        bag.insert_cubes(10, String::from("red"));
        bag.insert_cubes(10, String::from("green"));
        bag.insert_cubes(10, String::from("blue"));
        let mut game_parser = GameParser::new();
        let game = game_parser.parse_line(line).unwrap();

        let is_valid = GameValidator::validate_game(&bag, &game);

        assert_eq!(is_valid, should_be_valid)
    }

    #[test]
    fn calculates_sum_of_valid_game_ids() {
        let mut cube_conundrum = CubeConundrum::new();
        cube_conundrum.read_games_from_file("./src/advent_of_code/day_two/test-input.txt");
        cube_conundrum.insert_cubes_into_bag(12, "red");
        cube_conundrum.insert_cubes_into_bag(13, "green");
        cube_conundrum.insert_cubes_into_bag(14, "blue");

        let sum = cube_conundrum.get_sum_of_valid_game_ids();

        assert_eq!(sum, 8);
    }

    #[test]
    fn calculates_sum_of_game_powers() {
        let mut cube_conundrum = CubeConundrum::new();
        cube_conundrum.read_games_from_file("./src/advent_of_code/day_two/test-input.txt");

        let sum = cube_conundrum.get_sum_of_game_powers();

        assert_eq!(sum, 2286);
    }
}
