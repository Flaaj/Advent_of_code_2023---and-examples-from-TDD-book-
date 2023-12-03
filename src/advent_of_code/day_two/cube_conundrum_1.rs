use lazy_static::lazy_static;
use regex::Regex;

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

#[derive(PartialEq, Debug)]
struct Cubes {
    pub color: String,
    pub amount: u32,
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    sets: Vec<Vec<Cubes>>,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^Game \d+: ((\d+ [a-zA-Z]+[;,] )?+(\d+ [a-zA-Z]+)?)$").unwrap();
}

struct GameExtractor {
    games: Vec<Game>,
}

impl GameExtractor {
    pub fn new() -> Self {
        Self { games: vec![] }
    }

    fn validate_line(line: String) -> bool {
        RE.is_match(&line)
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
        let cubes_split = cubes_str.split(" ").collect::<Vec<&str>>();
        let amount = cubes_split.get(0).unwrap().parse::<u32>().unwrap();
        let color = cubes_split.get(1).unwrap().to_string();
        Cubes { amount, color }
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

    pub fn parse_line(&mut self, line: String) {
        if !Self::validate_line(line.clone()) {
            return;
        }

        let split_line = line.split(": ").collect::<Vec<&str>>();

        let meta_str = split_line.get(0).unwrap();
        let sets_str = split_line.get(1).unwrap();

        let id = Self::parse_id(meta_str);
        let sets = Self::parse_sets(sets_str);

        self.games.push(Game { id, sets });
    }

    pub fn get_games(self) -> Vec<Game> {
        self.games
    }
}

struct Sac {}

impl Sac {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_cubes(&self, amount: u32, color: &str) {}
}

struct GameValidator {}

impl GameValidator {
    fn validate_single_game(sac: &Sac, game: &Game) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::advent_of_code::day_two::cube_conundrum_1::{
        Cubes, Game, GameExtractor, GameValidator, Sac,
    };

    #[test]
    fn extracts_game_data_from_input_line_case_1() {
        let mut game_extractor = GameExtractor::new();

        game_extractor.parse_line(String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        ));

        assert_eq!(
            game_extractor.get_games(),
            vec![Game {
                id: 1,
                sets: vec![
                    vec![
                        Cubes {
                            color: String::from("blue"),
                            amount: 3
                        },
                        Cubes {
                            color: String::from("red"),
                            amount: 4
                        },
                    ],
                    vec![
                        Cubes {
                            color: String::from("red"),
                            amount: 1
                        },
                        Cubes {
                            color: String::from("green"),
                            amount: 2
                        },
                        Cubes {
                            color: String::from("blue"),
                            amount: 6
                        },
                    ],
                    vec![Cubes {
                        color: String::from("green"),
                        amount: 2
                    },]
                ]
            }]
        )
    }

    #[test]
    fn extracts_game_data_from_input_line_case_2() {
        let mut game_extractor = GameExtractor::new();

        game_extractor.parse_line(String::from(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        ));

        assert_eq!(
            game_extractor.get_games(),
            vec![Game {
                id: 3,
                sets: vec![
                    vec![
                        Cubes {
                            color: String::from("green"),
                            amount: 8
                        },
                        Cubes {
                            color: String::from("blue"),
                            amount: 6
                        },
                        Cubes {
                            color: String::from("red"),
                            amount: 20
                        },
                    ],
                    vec![
                        Cubes {
                            color: String::from("blue"),
                            amount: 5
                        },
                        Cubes {
                            color: String::from("red"),
                            amount: 4
                        },
                        Cubes {
                            color: String::from("green"),
                            amount: 13
                        },
                    ],
                    vec![
                        Cubes {
                            color: String::from("green"),
                            amount: 5
                        },
                        Cubes {
                            color: String::from("red"),
                            amount: 1
                        },
                    ]
                ]
            }]
        )
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green; 2 green; 2 green; 2 green")]
    #[case("Game 2: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green")]
    #[case("Game 4: 8 green")]
    #[case("Game 5: 8 green, 8 green, 8 green, 8 green, 8 green, 8 green, 8 green, 8 green, 8 green, 8 green, 8 green, 8 green")]
    fn parses_correct_lines(#[case] line: String) {
        let mut game_extractor = GameExtractor::new();

        game_extractor.parse_line(line);

        assert_eq!(game_extractor.get_games().len(), 1);
    }

    #[rstest]
    #[case("fafsgasdkyany7i632qfqwd")]
    #[case("")]
    #[case("Game 1 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")]
    #[case("Game 1 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")]
    #[case("3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")]
    fn doesnt_parse_incorrect_lines(#[case] line: String) {
        let mut game_extractor = GameExtractor::new();

        game_extractor.parse_line(line);

        assert_eq!(game_extractor.get_games().len(), 0);
    }

    #[rstest]
    #[case("Game 1: 8 green", true)]
    #[case("Game 2: 12 green", false)]
    #[case("Game 3: 12 green, 8 red, 9 blue", false)]
    #[case("Game 3: 9 green, 8 red, 9 blue", true)]
    fn determines_if_the_game_is_valid(#[case] line: String, #[case] should_be_valid: bool) {
        let sac = Sac::new();
        sac.add_cubes(10, "red");
        sac.add_cubes(10, "green");
        sac.add_cubes(10, "blue");
        let mut game_extractor = GameExtractor::new();
        game_extractor.parse_line(line);
        let games = game_extractor.get_games();
        let game = games.get(0).unwrap();

        let is_valid = GameValidator::validate_single_game(&sac, game);

        assert_eq!(is_valid, should_be_valid)
    }
}
